use axum::Router;
use axum_extra::routing::RouterExt;

use crate::application::ApplicationState;

mod github;

pub fn router(state: ApplicationState) -> Router {
    Router::new()
        .typed_post(github::handler::handle_webhook)
        .with_state(state)
}
