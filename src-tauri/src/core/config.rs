use std::fmt;
use std::fs;
use std::path::Path;

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
}

#[derive(Debug, Deserialize)]
pub struct GitlabRepoConfig {
    pub name: String,
    pub main_branch: String,
}

#[derive(Debug)]
pub enum ConfigError {
    Io(std::io::Error),
    Parse(serde_yaml::Error),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::Io(err) => write!(f, "Failed to read config file: {}", err),
            ConfigError::Parse(err) => write!(f, "Failed to parse config file: {}", err),
        }
    }
}

impl std::error::Error for ConfigError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ConfigError::Io(err) => Some(err),
            ConfigError::Parse(err) => Some(err),
        }
    }
}

impl From<std::io::Error> for ConfigError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<serde_yaml::Error> for ConfigError {
    fn from(error: serde_yaml::Error) -> Self {
        Self::Parse(error)
    }
}

pub fn read_config(path: impl AsRef<Path>) -> Result<Config, ConfigError> {
    let contents = fs::read_to_string(path)?;
    let config = serde_yaml::from_str(&contents)?;
    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::read_config;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn reads_sample_config() {
        let config = read_config("../docs/sample-config.yaml").expect("read config");

        assert_eq!(config.providers.len(), 1);
        let provider = &config.providers[0];
        match provider {
            super::ProviderConfig::Github { token, repos } => {
                assert_eq!(token.env, "GITHUB_TOKEN");
                assert_eq!(repos.len(), 1);

                let repo = &repos[0];
                assert_eq!(repo.name, "nknapp/frontend-testing");
                assert_eq!(repo.main_branch, "main");
                assert_eq!(repo.workflow, "playwright.yml");
            }
            super::ProviderConfig::Gitlab { .. } => panic!("expected github provider"),
        }
    }

    #[test]
    fn reads_inline_yaml() {
        let yaml = r#"providers:
  - type: github
    token:
      env: GITHUB_TOKEN
    repos:
      - name: nknapp/frontend-testing
        main_branch: main
        workflow: playwright.yml
"#;

        let path = write_temp_yaml(yaml).expect("write temp file");
        let config = read_config(&path).expect("read temp config");

        assert_eq!(config.providers.len(), 1);
        match &config.providers[0] {
            super::ProviderConfig::Github { repos, .. } => {
                assert_eq!(repos.len(), 1);
            }
            super::ProviderConfig::Gitlab { .. } => panic!("expected github provider"),
        }
    }

    fn write_temp_yaml(contents: &str) -> Result<std::path::PathBuf, std::io::Error> {
        let mut path = std::env::temp_dir();
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        path.push(format!("pmon-config-test-{}.yaml", nanos));
        fs::write(&path, contents)?;
        Ok(path)
    }
}
