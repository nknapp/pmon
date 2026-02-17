use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub providers: Vec<ProviderConfig>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ProviderConfig {
    Github {
        token: TokenConfig,
        repos: Vec<GithubRepoConfig>,
    },
    Gitlab {
        token: TokenConfig,
        repos: Vec<GitlabRepoConfig>,
    },
}

#[derive(Debug, Deserialize)]
pub struct TokenConfig {
    pub env: String,
}

#[derive(Debug, Deserialize)]
pub struct GithubRepoConfig {
    pub name: String,
    pub main_branch: String,
    pub workflow: String,
    pub show_in_tray: bool,
}

#[derive(Debug, Deserialize)]
pub struct GitlabRepoConfig {
    pub name: String,
    pub main_branch: String,
    pub show_in_tray: bool,
}

