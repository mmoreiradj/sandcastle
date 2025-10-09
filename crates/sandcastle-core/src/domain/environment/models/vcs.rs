#[derive(Debug, Clone)]
pub struct DownloadFileRequest {
    pub repository_id: u64,
    pub path: String,
    pub content_type: String,
    pub r#ref: String,
}

#[derive(Debug, Clone)]
pub struct FetchPRLastCommitSHARequest {
    pub repository_id: u64,
    pub pr_number: u64,
}