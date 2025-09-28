#[derive(Debug, Clone)]
pub struct DownloadFileRequest {
    pub uri: String,
    pub content_type: String,
}
