use crate::repository::user_repository::UserRepository;

#[derive(Clone)]
pub struct UserService {
    repo: UserRepository,
}

impl UserService {
    pub fn new(repo: UserRepository) -> Self {
        Self { repo }
    }

    pub async fn create_user(
        &self,
        username: String,
        password_hash: String,
    ) -> Result<(), sqlx::Error> {
        self.repo.create(&username, &password_hash).await
    }
}
