//! Legacy compatibility mount for the old `/api/docs/agent` prefix.
//! The single maintained implementation lives in `crate::agent::router` under `/agent/v1`.

use axum::Router;

pub fn router() -> Router {
    crate::agent::router::router()
}
