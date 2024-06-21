use super::*;
use chrono::DateTime;
use mongodb::options::TransactionOptions;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct UserEmailRegistrationRequest {
    /// The user's email
    pub email: String,
    /// The hashed password generate at frontend
    #[serde(default)]
    pub password: i64,
    /// Unique name, used for home page
    #[serde(default)]
    pub username: String,
    /// Repeatable custom name for display
    #[serde(default)]
    pub nickname: String,
}

impl UserEmailRegistrationRequest {
    // Transaction Operator: `email` and `user` must be created at the same time
    pub async fn execute(&self, client: &PangaeaClient) -> Result<i64> {
        let mut session = client.db_service.start_session(None).await?;
        session.start_transaction(None).await?;
        let email_collection = client.email_collection().await;
        let users_collection = client.users_collection().await;
        let email_object = self.make_email_object();
        let email_code = email_object.email_code;
        let user_object = self.make_user_object(&email_object);
        email_collection.insert_one_with_session(email_object, None, &mut session).await?;
        users_collection.insert_one_with_session(user_object, None, &mut session).await?;
        session.commit_transaction().await?;
        Ok(email_code)
    }
    pub fn make_email_object(&self) -> EmailObject {
        EmailObject {
            id: ObjectId::new(),
            email: self.email.to_lowercase(),
            email_verified: false,
            email_code: rand::thread_rng().gen(),
            expired_at: expired_after_seconds(60 * 10),
        }
    }

    pub fn make_user_object(&self, email: &EmailObject) -> UserObject {
        let id = ObjectId::new();
        let password = UserPasswordObject { password: self.password, salt: rand::thread_rng().gen() };
        let username = if self.username.is_empty() { format!("user_{id}") } else { self.username.clone() };
        let nickname = if self.nickname.is_empty() { username.clone() } else { self.nickname.clone() };
        UserObject {
            // random
            id,
            // must unique
            username,
            // display name, can't be empty
            nickname,
            email: email.id,
            password,
            login_verification_required: false,
            banned_status: false,
        }
    }
}
