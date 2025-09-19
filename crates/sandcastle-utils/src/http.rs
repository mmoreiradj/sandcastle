#[cfg_attr(feature = "testing", mockall::automock)]
pub trait HttpSend: Send + Sync {
    fn send(
        &self,
        request: reqwest::RequestBuilder,
    ) -> impl Future<Output = Result<reqwest::Response, reqwest::Error>> + Send;
}

pub struct HttpSender;

impl HttpSend for HttpSender {
    async fn send(
        &self,
        request: reqwest::RequestBuilder,
    ) -> Result<reqwest::Response, reqwest::Error> {
        request.send().await
    }
}
