mod applications;
mod domains;
mod handlers;
mod infrastructures;
mod presentations;

use crate::handlers::*;
use axum::{
    routing::{get, post},
    Router,
};

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/register", post(register_user));

    Ok(router.into())
}
