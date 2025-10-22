#[derive(Debug, Clone)]
pub struct CreateOrUpdateArgocdApplicationRequest {
    pub applications: Vec<String>,
    pub labels: Vec<(String, String)>,
}
