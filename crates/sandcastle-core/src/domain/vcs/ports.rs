use crate::error::SandcastleError;

#[derive(Debug, Clone)]
pub struct DownloadFileRequest {
  pub uri: String,
  pub content_type: String,
}

pub trait VCService: Send + Sync {
  fn download_file(&self, request: DownloadFileRequest) -> impl Future<Output = Result<Vec<u8>, SandcastleError>> + Send;
}
