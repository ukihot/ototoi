use crate::applications::dto::user_dto::UserDTO;
use crate::applications::user_usecase::user_input_port::UserInputPort;
use crate::applications::user_usecase::user_output_port::UserOutputPort;

use crate::domains::user::user_factory::UserFactory;
use crate::domains::user::user_repository::UserRepository;

pub struct UserInteractor<R: UserRepository, O: UserOutputPort, F: UserFactory> {
    repository: R,
    output: O,
    factory: F,
}

impl<R: UserRepository, O: UserOutputPort, F: UserFactory> UserInteractor<R, O, F> {
    pub fn new(repository: R, output: O, factory: F) -> Self {
        UserInteractor {
            repository,
            output,
            factory,
        }
    }
}

impl<R: UserRepository, O: UserOutputPort, F: UserFactory> UserInputPort
    for UserInteractor<R, O, F>
{
    async fn register_user(&self, id: u32, name: String, email: String) -> Result<(), String> {
        let user = self
            .factory
            .create(id, name, email)
            .map_err(|e| format!("Failed to create user: {}", e))?;

        self.repository
            .save(&user)
            .await
            .map_err(|e| format!("Failed to save user: {}", e))?;

        let user_dto = UserDTO::from(&user);
        self.output.show_user(&user_dto);

        Ok(())
    }
}
