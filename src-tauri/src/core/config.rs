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
    Validation(String),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::Io(err) => write!(f, "Failed to read config file: {}", err),
            ConfigError::Parse(err) => write!(f, "Failed to parse config file: {}", err),
            ConfigError::Validation(message) => write!(f, "Invalid config file: {}", message),
        }
    }
}

impl std::error::Error for ConfigError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ConfigError::Io(err) => Some(err),
            ConfigError::Parse(err) => Some(err),
            ConfigError::Validation(_) => None,
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
    let raw_config: serde_yaml::Value = serde_yaml::from_str(&contents)?;
    validate_config_raw(&raw_config)?;
    let config = serde_yaml::from_value(raw_config)?;
    validate_config(&config)?;
    Ok(config)
}

fn validate_config_raw(config: &serde_yaml::Value) -> Result<(), ConfigError> {
    let providers = config
        .get("providers")
        .and_then(|value| value.as_sequence())
        .ok_or_else(|| ConfigError::Validation("At least one provider is required".to_string()))?;

    if providers.len() != 1 {
        return Err(ConfigError::Validation(
            "Only one provider is supported for now".to_string(),
        ));
    }

    let provider = providers
        .first()
        .ok_or_else(|| ConfigError::Validation("At least one provider is required".to_string()))?;
    let provider_type = provider
        .get("type")
        .and_then(|value| value.as_str())
        .ok_or_else(|| {
            ConfigError::Validation(
                "Only github or gitlab providers are supported for now".to_string(),
            )
        })?;

    if provider_type != "github" && provider_type != "gitlab" {
        return Err(ConfigError::Validation(
            "Only github or gitlab providers are supported for now".to_string(),
        ));
    }

    let repo_count = provider
        .get("repos")
        .and_then(|value| value.as_sequence())
        .map(|repos| repos.len())
        .unwrap_or(0);

    if repo_count != 1 {
        return Err(ConfigError::Validation(
            "Only one repository is supported for now".to_string(),
        ));
    }

    Ok(())
}

fn validate_config(config: &Config) -> Result<(), ConfigError> {
    if config.providers.len() != 1 {
        return Err(ConfigError::Validation(
            "Only one provider is supported for now".to_string(),
        ));
    }

    let provider = config
        .providers
        .first()
        .ok_or_else(|| ConfigError::Validation("At least one provider is required".to_string()))?;

    let repo_count = match provider {
        ProviderConfig::Github { repos, .. } => repos.len(),
        ProviderConfig::Gitlab { repos, .. } => repos.len(),
    };

    if repo_count != 1 {
        return Err(ConfigError::Validation(
            "Only one repository is supported for now".to_string(),
        ));
    }

    Ok(())
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

    #[test]
    fn rejects_multiple_providers() {
        let yaml = r#"providers:
  - type: github
    token:
      env: GITHUB_TOKEN
    repos:
      - name: nknapp/frontend-testing
        main_branch: main
        workflow: playwright.yml
  - type: gitlab
    token:
      env: GITLAB_TOKEN
    repos:
      - name: org/repo
        main_branch: main
"#;

        let path = write_temp_yaml(yaml).expect("write temp file");
        let error = read_config(&path).expect_err("expected validation error");

        assert!(format!("{}", error).contains("Only one provider is supported for now"));
    }

    #[test]
    fn rejects_multiple_repos() {
        let yaml = r#"providers:
  - type: github
    token:
      env: GITHUB_TOKEN
    repos:
      - name: nknapp/frontend-testing
        main_branch: main
        workflow: playwright.yml
      - name: nknapp/other-repo
        main_branch: main
        workflow: build.yml
"#;

        let path = write_temp_yaml(yaml).expect("write temp file");
        let error = read_config(&path).expect_err("expected validation error");

        assert!(format!("{}", error).contains("Only one repository is supported for now"));
    }

    #[test]
    fn rejects_unknown_provider() {
        let yaml = r#"providers:
  - type: bitbucket
    token:
      env: BITBUCKET_TOKEN
    repos:
      - name: org/repo
        main_branch: main
"#;

        let path = write_temp_yaml(yaml).expect("write temp file");
        let error = read_config(&path).expect_err("expected validation error");

        assert!(
            format!("{}", error).contains("Only github or gitlab providers are supported for now")
        );
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
