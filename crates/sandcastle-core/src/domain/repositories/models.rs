#[derive(Debug, Clone)]
pub struct RepositoryConfiguration {
    pub repository_url: String,
    pub authentication: Authentication,
}

#[derive(Debug, Clone)]
pub enum Authentication {
    GitHubApp(GitHubAppAuthentication),
}

#[derive(Debug, Clone)]
pub struct GitHubAppAuthentication {
    pub app_id: String,
    pub installation_id: String,
    pub private_key: String,
}
