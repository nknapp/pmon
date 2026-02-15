use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use serde::Deserialize;

use crate::core::{DataProvider, StateSummary, StateSummaryAdapter, StateSummaryGateway};

const DEFAULT_GITLAB_API_BASE_URL: &str = "https://gitlab.com/api/v4";
const DEFAULT_POLL_INTERVAL: Duration = Duration::from_secs(60);

#[derive(Clone)]
pub struct GitlabRepo {
    name: String,
    main_branch: String,
}

impl GitlabRepo {
    pub fn new(name: String, main_branch: String) -> Self {
        Self { name, main_branch }
    }
}

pub struct GitlabProvider {
    token_env: String,
    repos: Vec<GitlabRepo>,
    api_base_url: String,
    poll_interval: Duration,
    stop_signal: Arc<AtomicBool>,
    thread_handle: Option<JoinHandle<()>>,
}

impl GitlabProvider {
    pub fn new(token_env: String, repos: Vec<GitlabRepo>) -> Self {
        Self::new_with_base_url(token_env, repos, DEFAULT_GITLAB_API_BASE_URL.to_string())
    }

    pub fn new_with_base_url(
        token_env: String,
        repos: Vec<GitlabRepo>,
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
            match fetch_pipelines(client, &self.api_base_url, &self.token_env, repo) {
                Ok(pipelines) => {
                    if let Some(summary) = state_from_pipelines(&pipelines) {
                        combined = Some(combine_summaries(combined, summary));
                    }
                }
                Err(error) => {
                    eprintln!("GitLab provider error for {}: {}", repo.name, error);
                }
            }
        }

        if let Some(summary) = combined {
            gateway.set_state_summary(summary);
        }
    }
}

impl DataProvider for GitlabProvider {
    fn refresh(&mut self) {
        let client = reqwest::blocking::Client::new();
        let gateway = StateSummaryGateway::new();
        self.poll_once(&client, &gateway);
    }

    fn start(&mut self, state_summary_gateway: Arc<StateSummaryGateway>) {
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
                    match fetch_pipelines(&client, &api_base_url, &token_env, repo) {
                        Ok(pipelines) => {
                            if let Some(summary) = state_from_pipelines(&pipelines) {
                                combined = Some(combine_summaries(combined, summary));
                            }
                        }
                        Err(error) => {
                            eprintln!("GitLab provider error for {}: {}", repo.name, error);
                        }
                    }
                }

                if let Some(summary) = combined {
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
struct Pipeline {
    status: String,
}

fn fetch_pipelines(
    client: &reqwest::blocking::Client,
    api_base_url: &str,
    token_env: &str,
    repo: &GitlabRepo,
) -> Result<Vec<Pipeline>, String> {
    let token = std::env::var(token_env).map_err(|_| format!("Missing env var {}", token_env))?;
    let repo_encoded = urlencoding::encode(&repo.name);
    let branch_encoded = urlencoding::encode(&repo.main_branch);
    let url = format!(
        "{}/projects/{}/pipelines?ref={}&per_page=2",
        api_base_url, repo_encoded, branch_encoded
    );

    let response = client
        .get(url)
        .header("PRIVATE-TOKEN", token)
        .send()
        .map_err(|error| error.to_string())?;

    let response = response
        .error_for_status()
        .map_err(|error| error.to_string())?;

    response
        .json::<Vec<Pipeline>>()
        .map_err(|error| error.to_string())
}

fn state_from_pipelines(pipelines: &[Pipeline]) -> Option<StateSummary> {
    let latest = pipelines.first()?;
    match latest.status.as_str() {
        "failed" => Some(StateSummary::Failure),
        "success" => Some(StateSummary::Ok),
        "running" | "pending" | "created" | "manual" => Some(pending_state_from_history(pipelines)),
        "canceled" | "skipped" => Some(StateSummary::Failure),
        _ => None,
    }
}

fn pending_state_from_history(pipelines: &[Pipeline]) -> StateSummary {
    for pipeline in pipelines.iter().skip(1) {
        match pipeline.status.as_str() {
            "failed" => return StateSummary::FailurePending,
            "success" => return StateSummary::OkPending,
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

    use super::{GitlabProvider, GitlabRepo};
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
            std::env::set_var("GITLAB_TOKEN", token);
        }

        let _mock = server
            .mock("GET", "/projects/org%2Frepo/pipelines")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("ref".into(), "main".into()),
                mockito::Matcher::UrlEncoded("per_page".into(), "2".into()),
            ]))
            .match_header("PRIVATE-TOKEN", token)
            .with_status(200)
            .with_body(r#"[{"status":"failed"},{"status":"success"}]"#)
            .create();

        let repo = GitlabRepo::new("org/repo".to_string(), "main".to_string());
        let provider =
            GitlabProvider::new_with_base_url("GITLAB_TOKEN".to_string(), vec![repo], server.url())
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
    fn reports_ok_pending_when_latest_running_and_previous_success() {
        let mut server = Server::new();
        let token = "test-token";
        unsafe {
            std::env::set_var("GITLAB_TOKEN", token);
        }

        let _mock = server
            .mock("GET", "/projects/org%2Frepo/pipelines")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("ref".into(), "main".into()),
                mockito::Matcher::UrlEncoded("per_page".into(), "2".into()),
            ]))
            .match_header("PRIVATE-TOKEN", token)
            .with_status(200)
            .with_body(r#"[{"status":"running"},{"status":"success"}]"#)
            .create();

        let repo = GitlabRepo::new("org/repo".to_string(), "main".to_string());
        let provider =
            GitlabProvider::new_with_base_url("GITLAB_TOKEN".to_string(), vec![repo], server.url());

        let gateway = StateSummaryGateway::new();
        let latest = Arc::new(Mutex::new(None));
        gateway.add_controller(Box::new(TestSummaryAdapter {
            latest: latest.clone(),
        }));

        provider.poll_once(&reqwest::blocking::Client::new(), &gateway);

        assert_eq!(*latest.lock().unwrap(), Some(StateSummary::OkPending));
    }
}
