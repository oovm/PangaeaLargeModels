use super::*;

mod user_email_registration;

pub use self::user_email_registration::UserEmailRegistrationRequest;



#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct UserEmailVerificationRequest {
    email: String,
    code: i64,
}

impl UserEmailVerificationRequest {
    pub async fn execute(&self, client: &PangaeaClient) -> Result<bool> {
        let filter = {
            let mut object = Document::new();
            object.insert("email.id", &Bson::ObjectId(ObjectId::parse_str(&self.email).unwrap()));
            object.insert("email.email_code", &Bson::Int64(self.code));
            object
        };
        let update = doc! {
            "$set": {
            "email.email_verified": true,
        }
        };
        let collection = client.email_collection().await;
        match collection.update_one(filter, update, None).await? {
            UpdateResult { matched_count: 1, .. } => Ok(true),
            _ => Ok(false),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct UserEmailLoginRequest {
    /// The user's email
    email: String,
    /// The hashed password generate at frontend
    ///
    /// if 0, run `User Email Verification`
    #[serde(default)]
    password: i64,
}

impl UserEmailLoginRequest {
    pub async fn execute(&self, client: &PangaeaClient) -> Result<bool> {
        todo!()
    }
}
