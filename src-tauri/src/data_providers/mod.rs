mod github;
mod gitlab;

use crate::core::config::{Config, ProviderConfig};
use crate::core::DataProvider;

pub use github::GithubProvider;
pub use gitlab::GitlabProvider;

pub fn providers_from_config(config: &Config) -> Vec<Box<dyn DataProvider>> {
    let mut providers: Vec<Box<dyn DataProvider>> = Vec::new();

    for provider in &config.providers {
        match provider {
            ProviderConfig::Gitlab { token, repos } => {
                let gitlab_repos = repos
                    .iter()
                    .map(|repo| {
                        gitlab::GitlabRepo::new(repo.name.clone(), repo.main_branch.clone())
                    })
                    .collect();
                providers.push(Box::new(GitlabProvider::new(
                    token.env.clone(),
                    gitlab_repos,
                )));
            }
            ProviderConfig::Github { token, repos } => {
                let github_repos = repos
                    .iter()
                    .map(|repo| {
                        github::GithubRepo::new(
                            repo.name.clone(),
                            repo.main_branch.clone(),
                            repo.workflow.clone(),
                        )
                    })
                    .collect();
                providers.push(Box::new(GithubProvider::new(
                    token.env.clone(),
                    github_repos,
                )));
            }
        }
    }

    providers
}
