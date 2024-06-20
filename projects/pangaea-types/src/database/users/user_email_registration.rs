use super::*;
use crate::PangaeaError;
use chrono::DateTime;
use std::sync::Arc;

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
    pub async fn execute(&self, client: &PangaeaClient) -> Result<i64> {
        // If the email doesn't exist, proceed with insertion
        let email_object = self.make_email_object();
        let user_object = self.make_user_object(&email_object);
        // Check if the email already exists
        let email = email_object.email.as_str();
        let email_collection = client.email_collection().await;
        match email_collection.find_one(doc! {"email": email}, None).await? {
            Some(_) => {
                Err(PangaeaError::custom("该邮箱已注册"))?;
            }
            _ => {}
        }
        // Check if the username already exists
        // let user_collection = client.users_collection().await;
        // let username = user_object.username.as_str();
        // match user_collection.find_one(doc! {"username": username}, None).await? {
        //     Some(_) => {
        //         Err(PangaeaError::custom("该用户名已被占用"))?;
        //     }
        //     _ => {}
        // }
        // Insert the email object
        let email_code = email_object.email_code;
        // Everything is ok Insert the user object
        let collection = client.email_collection().await;
        collection.insert_one(email_object.clone(), None).await?;

        let collection = client.users_collection().await;
        collection.insert_one(user_object, None).await?;
        return Ok(email_code);
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
