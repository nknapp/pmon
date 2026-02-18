use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use serde::Deserialize;

use crate::core::{DataProvider, StateSummary, StateSummaryAdapter, StateSummaryGateway};

const DEFAULT_GITHUB_API_BASE_URL: &str = "https://api.github.com";
const DEFAULT_POLL_INTERVAL: Duration = Duration::from_secs(60);

#[derive(Clone)]
pub struct GithubRepo {
    name: String,
    main_branch: String,
    workflow: String,
}

impl GithubRepo {
    pub fn new(name: String, main_branch: String, workflow: String) -> Self {
        Self {
            name,
            main_branch,
            workflow,
        }
    }
}

pub struct GithubProvider {
    token_env: String,
    repos: Vec<GithubRepo>,
    api_base_url: String,
    poll_interval: Duration,
    stop_signal: Arc<AtomicBool>,
    thread_handle: Option<JoinHandle<()>>,
}

impl GithubProvider {
    pub fn new(token_env: String, repos: Vec<GithubRepo>) -> Self {
        Self::new_with_base_url(token_env, repos, DEFAULT_GITHUB_API_BASE_URL.to_string())
    }

    pub fn new_with_base_url(
        token_env: String,
        repos: Vec<GithubRepo>,
        api_base_url: String,
    ) -> Self {
        Self {
            token_env,
            repos,
            api_base_url,
            poll_interval: DEFAULT_POLL_INTERVAL,
            stop_signal: Arc::new(AtomicBool::new(false)),
            thread_handle: None,
        }
    }

    pub fn with_poll_interval(mut self, poll_interval: Duration) -> Self {
        self.poll_interval = poll_interval;
        self
    }

    fn poll_once(&self, client: &reqwest::blocking::Client, gateway: &StateSummaryGateway) {
        let mut combined: Option<StateSummary> = None;

        for repo in &self.repos {
            eprintln!(
                "GitHub provider polling {} on {}",
                repo.name, repo.main_branch
            );
            match fetch_workflow_runs(client, &self.api_base_url, &self.token_env, repo) {
                Ok(runs) => {
                    eprintln!(
                        "GitHub provider received {} runs for {}",
                        runs.len(),
                        repo.name
                    );
                    if let Some(summary) = state_from_runs(&runs) {
                        eprintln!("GitHub provider summary for {} is {:?}", repo.name, summary);
                        combined = Some(combine_summaries(combined, summary));
                    }
                }
                Err(error) => {
                    eprintln!("GitHub provider error for {}: {}", repo.name, error);
                }
            }
        }

        if let Some(summary) = combined {
            gateway.set_state_summary(summary);
        }
    }
}

impl DataProvider for GithubProvider {
    fn refresh(&mut self) {
        let client = reqwest::blocking::Client::new();
        let gateway = StateSummaryGateway::new();
        self.poll_once(&client, &gateway);
    }

    fn start(&mut self, state_summary_gateway: Arc<StateSummaryGateway>) {
        eprintln!("Starting GitHub provider");
        if self.thread_handle.is_some() {
            return;
        }

        let repos = self.repos.clone();
        let api_base_url = self.api_base_url.clone();
        let token_env = self.token_env.clone();
        let poll_interval = self.poll_interval;
        let stop_signal = self.stop_signal.clone();

        self.thread_handle = Some(thread::spawn(move || {
            let client = reqwest::blocking::Client::new();

            loop {
                if stop_signal.load(Ordering::Relaxed) {
                    break;
                }

                let mut combined: Option<StateSummary> = None;
                for repo in &repos {
                    eprintln!(
                        "GitHub provider polling {} on {}",
                        repo.name, repo.main_branch
                    );
                    match fetch_workflow_runs(&client, &api_base_url, &token_env, repo) {
                        Ok(runs) => {
                            eprintln!(
                                "GitHub provider received {} runs for {}",
                                runs.len(),
                                repo.name
                            );
                            if let Some(summary) = state_from_runs(&runs) {
                                eprintln!(
                                    "GitHub provider summary for {} is {:?}",
                                    repo.name, summary
                                );
                                combined = Some(combine_summaries(combined, summary));
                            }
                        }
                        Err(error) => {
                            eprintln!("GitHub provider error for {}: {}", repo.name, error);
                        }
                    }
                }

                if let Some(summary) = combined {
                    eprintln!("GitHub provider combined summary is {:?}", summary);
                    state_summary_gateway.set_state_summary(summary);
                }

                sleep_with_stop(&stop_signal, poll_interval);
            }
        }));
    }

    fn stop(&mut self) {
        self.stop_signal.store(true, Ordering::Relaxed);
        if let Some(handle) = self.thread_handle.take() {
            let _ = handle.join();
        }
    }
}

#[derive(Debug, Deserialize)]
struct WorkflowRuns {
    workflow_runs: Vec<WorkflowRun>,
}

#[derive(Debug, Deserialize)]
struct WorkflowRun {
    status: String,
    conclusion: Option<String>,
}

fn fetch_workflow_runs(
    client: &reqwest::blocking::Client,
    api_base_url: &str,
    token_env: &str,
    repo: &GithubRepo,
) -> Result<Vec<WorkflowRun>, String> {
    let token = std::env::var(token_env).map_err(|_| format!("Missing env var {}", token_env))?;
    let (owner, repo_name) = split_repo_name(&repo.name)?;
    let workflow = urlencoding::encode(&repo.workflow);
    let branch = urlencoding::encode(&repo.main_branch);
    let url = format!(
        "{}/repos/{}/{}/actions/workflows/{}/runs?branch={}&per_page=2",
        api_base_url, owner, repo_name, workflow, branch
    );

    if github_debug_enabled() {
        eprintln!(
            "GitHub provider request: GET {} (token env: {})",
            url, token_env
        );
    }

    let response = client
        .get(url)
        .header("Authorization", format!("Bearer {}", token))
        .header("Accept", "application/vnd.github+json")
        .header("User-Agent", "pmon")
        .send()
        .map_err(|error| error.to_string())?;

    let status = response.status();
    let request_id = response
        .headers()
        .get("x-github-request-id")
        .and_then(|value| value.to_str().ok())
        .unwrap_or("unknown")
        .to_string();
    let body = response.text().map_err(|error| error.to_string())?;

    log::info!(
        "GitHub API request completed: status {} request_id {}",
        status,
        request_id
    );
    log::debug!("GitHub API response: {}", body);

    if !status.is_success() {
        return Err(format!(
            "GitHub API returned {} (request_id {})",
            status, request_id
        ));
    }

    let runs = serde_json::from_str::<WorkflowRuns>(&body).map_err(|error| error.to_string())?;
    Ok(runs.workflow_runs)
}

fn github_debug_enabled() -> bool {
    matches!(
        std::env::var("PMON_GITHUB_DEBUG").as_deref(),
        Ok("1") | Ok("true") | Ok("TRUE") | Ok("yes") | Ok("YES")
    )
}

fn split_repo_name(name: &str) -> Result<(String, String), String> {
    let (owner, repo) = name
        .split_once('/')
        .ok_or_else(|| format!("Invalid GitHub repo name: {}", name))?;
    Ok((
        urlencoding::encode(owner).into_owned(),
        urlencoding::encode(repo).into_owned(),
    ))
}

fn state_from_runs(runs: &[WorkflowRun]) -> Option<StateSummary> {
    let latest = runs.first()?;

    match latest.status.as_str() {
        "completed" => match latest.conclusion.as_deref() {
            Some("success") => Some(StateSummary::Ok),
            Some("failure")
            | Some("cancelled")
            | Some("timed_out")
            | Some("action_required")
            | Some("startup_failure")
            | Some("neutral")
            | Some("skipped") => Some(StateSummary::Failure),
            _ => None,
        },
        "queued" | "in_progress" => Some(pending_state_from_history(runs)),
        _ => None,
    }
}

fn pending_state_from_history(runs: &[WorkflowRun]) -> StateSummary {
    for run in runs.iter().skip(1) {
        if run.status != "completed" {
            continue;
        }
        match run.conclusion.as_deref() {
            Some("failure")
            | Some("cancelled")
            | Some("timed_out")
            | Some("action_required")
            | Some("startup_failure")
            | Some("neutral")
            | Some("skipped") => return StateSummary::FailurePending,
            Some("success") => return StateSummary::OkPending,
            _ => {}
        }
    }

    StateSummary::OkPending
}

fn combine_summaries(current: Option<StateSummary>, next: StateSummary) -> StateSummary {
    match current {
        None => next,
        Some(existing) => {
            if summary_rank(next) > summary_rank(existing) {
                next
            } else {
                existing
            }
        }
    }
}

fn summary_rank(summary: StateSummary) -> u8 {
    match summary {
        StateSummary::Ok => 0,
        StateSummary::OkPending => 1,
        StateSummary::FailurePending => 2,
        StateSummary::Failure => 3,
    }
}

fn sleep_with_stop(stop_signal: &AtomicBool, duration: Duration) {
    let mut remaining = duration;
    let tick = Duration::from_secs(1);
    while remaining > Duration::ZERO {
        if stop_signal.load(Ordering::Relaxed) {
            break;
        }
        let sleep_for = if remaining < tick { remaining } else { tick };
        thread::sleep(sleep_for);
        remaining = remaining.saturating_sub(sleep_for);
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use std::time::Duration;

    use mockito::Server;

    use super::{GithubProvider, GithubRepo};
    use crate::core::{StateSummary, StateSummaryAdapter, StateSummaryGateway};

    struct TestSummaryAdapter {
        latest: Arc<Mutex<Option<StateSummary>>>,
    }

    impl StateSummaryAdapter for TestSummaryAdapter {
        fn set_state_summary(&self, state: StateSummary) {
            if let Ok(mut latest) = self.latest.lock() {
                *latest = Some(state);
            }
        }
    }

    #[test]
    fn reports_failure_when_latest_failed() {
        let mut server = Server::new();
        let token = "test-token";
        unsafe {
            std::env::set_var("GITHUB_TOKEN", token);
        }

        let _mock = server
            .mock("GET", "/repos/org/repo/actions/workflows/build.yml/runs")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("branch".into(), "main".into()),
                mockito::Matcher::UrlEncoded("per_page".into(), "2".into()),
            ]))
            .match_header("authorization", format!("Bearer {}", token).as_str())
            .with_status(200)
            .with_body(
                r#"{"workflow_runs":[{"status":"completed","conclusion":"failure"},{"status":"completed","conclusion":"success"}]}"#,
            )
            .create();

        let repo = GithubRepo::new(
            "org/repo".to_string(),
            "main".to_string(),
            "build.yml".to_string(),
        );
        let provider =
            GithubProvider::new_with_base_url("GITHUB_TOKEN".to_string(), vec![repo], server.url())
                .with_poll_interval(Duration::from_secs(1));

        let gateway = StateSummaryGateway::new();
        let latest = Arc::new(Mutex::new(None));
        gateway.add_controller(Box::new(TestSummaryAdapter {
            latest: latest.clone(),
        }));

        provider.poll_once(&reqwest::blocking::Client::new(), &gateway);

        assert_eq!(*latest.lock().unwrap(), Some(StateSummary::Failure));
    }

    #[test]
    fn reports_ok_pending_when_latest_in_progress_and_previous_success() {
        let mut server = Server::new();
        let token = "test-token";
        unsafe {
            std::env::set_var("GITHUB_TOKEN", token);
        }

        let _mock = server
            .mock("GET", "/repos/org/repo/actions/workflows/build.yml/runs")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("branch".into(), "main".into()),
                mockito::Matcher::UrlEncoded("per_page".into(), "2".into()),
            ]))
            .match_header("authorization", format!("Bearer {}", token).as_str())
            .with_status(200)
            .with_body(
                r#"{"workflow_runs":[{"status":"in_progress","conclusion":null},{"status":"completed","conclusion":"success"}]}"#,
            )
            .create();

        let repo = GithubRepo::new(
            "org/repo".to_string(),
            "main".to_string(),
            "build.yml".to_string(),
        );
        let provider =
            GithubProvider::new_with_base_url("GITHUB_TOKEN".to_string(), vec![repo], server.url());

        let gateway = StateSummaryGateway::new();
        let latest = Arc::new(Mutex::new(None));
        gateway.add_controller(Box::new(TestSummaryAdapter {
            latest: latest.clone(),
        }));

        provider.poll_once(&reqwest::blocking::Client::new(), &gateway);

        assert_eq!(*latest.lock().unwrap(), Some(StateSummary::OkPending));
    }
}
