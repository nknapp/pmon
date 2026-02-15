use crate::core::DataProvider;

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
}

impl GitlabProvider {
    pub fn new(token_env: String, repos: Vec<GitlabRepo>) -> Self {
        Self { token_env, repos }
    }
}

impl DataProvider for GitlabProvider {
    fn refresh(&mut self) {
        let _ = self.token_env.as_str();
        for repo in &self.repos {
            let _ = repo.name.as_str();
            let _ = repo.main_branch.as_str();
        }
    }
}
