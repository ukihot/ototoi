mod applications;
mod domains;
mod infrastructures;
mod presentations;
use crate::applications::dto::user_dto::{UserDTO, UserRegisterResponse};
use crate::applications::user_usecase::user_interactor::UserInteractor;
use crate::infrastructures::user_repository_impl::UserRepositoryImpl;
use crate::presentations::presenter::UserPresenter;
use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use domains::user::user_factory::DefaultUserFactory;

use crate::presentations::controller::UserController;

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn register_user(Json(user_dto): Json<UserDTO>) -> (StatusCode, Json<UserRegisterResponse>) {
    let user_repository = UserRepositoryImpl;
    let user_presenter = UserPresenter;
    let user_factory = DefaultUserFactory;
    let user_interactor = UserInteractor::new(user_repository, user_presenter, user_factory);
    let controller = UserController::new(user_interactor);

    // `register_user`がResultを返すように修正した場合
    match controller.register(user_dto) {
        Ok(response) => (StatusCode::OK, Json(response)),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(UserRegisterResponse {
                success: false,
                message: e,
            }),
        ),
    }
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/register", post(register_user));

    Ok(router.into())
}
