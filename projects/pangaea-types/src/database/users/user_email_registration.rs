use super::*;
use chrono::DateTime;

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
    pub async fn execute(&self, client: &PangaeaClient) -> Result<InsertOneResult, mongodb::error::Error> {
        let now: DateTime<Utc> = Utc::now();
        let seconds_since_epoch = now.timestamp() as u32;
        // Optionally, you could manage an increment counter if needed for ordering within the same second.
        let increment = 0u32; // Default increment for simplicity.
        let bson_expired_at = Timestamp { time: seconds_since_epoch, increment };



        let email_lowercase = self.email.to_lowercase();
        let email_object = UserEmailObject {
            id: ObjectId::new(),
            email: email_lowercase.clone(),
            email_verified: false,
            email_code: rand::thread_rng().gen(),
            expired_at: bson_expired_at,
        };
        let user_password_object = UserPasswordObject { password: self.password, salt: rand::thread_rng().gen() };
        let user_object = UserObject {
            id: ObjectId::new(),
            username: self.username.clone(),
            email: email_object,
            password: user_password_object,
            login_verification_required: false,
            banned_status: false,
        };

        let collection = client.users_collection().await;
        collection.insert_one(user_object, None).await
    }
}
