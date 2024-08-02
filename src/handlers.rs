use crate::applications::dto::user_dto::{UserDTO, UserRegisterResponse};
use crate::applications::user_usecase::user_interactor::UserInteractor;
use crate::domains::user::user_factory::DefaultUserFactory;
use crate::infrastructures::user_repository_impl::UserRepositoryImpl;
use crate::presentations::controller::UserController;
use crate::presentations::presenter::UserPresenter;
use axum::{http::StatusCode, Json};

pub async fn register_user(
    Json(user_dto): Json<UserDTO>,
) -> (StatusCode, Json<UserRegisterResponse>) {
    let user_repository = UserRepositoryImpl;
    let user_presenter = UserPresenter;
    let user_factory = DefaultUserFactory;
    let user_interactor = UserInteractor::new(user_repository, user_presenter, user_factory);
    let controller = UserController::new(user_interactor);

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

pub async fn hello_world() -> &'static str {
    "Hello, world!"
}
