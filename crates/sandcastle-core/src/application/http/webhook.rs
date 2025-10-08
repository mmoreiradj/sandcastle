use axum::Router;
use axum_extra::routing::RouterExt;

mod github;

pub fn router() -> Router {
    Router::new().typed_post(github::handler::handle_webhook)
}
