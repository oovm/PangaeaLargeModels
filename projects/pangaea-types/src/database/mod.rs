use chrono::Utc;
use mongodb::{
    bson,
    bson::{doc, oid::ObjectId, Bson, Document, Timestamp},
    options::{ClientOptions, ServerApi, ServerApiVersion},
    results::{InsertOneResult, UpdateResult},
    Client, Collection,
};
use rand::Rng;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Formatter};

pub mod users;

pub struct PangaeaClient {
    db_service: Client,
}

impl Debug for PangaeaClient {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl PangaeaClient {
    pub async fn new(uri: &str) -> Result<PangaeaClient, mongodb::error::Error> {
        let mut client_options = ClientOptions::parse(uri).await?;
        client_options.server_api = Some(ServerApi::builder().version(ServerApiVersion::V1).build());
        let client = Client::with_options(client_options)?;
        Ok(Self { db_service: client })
    }
    pub async fn users_collection(&self) -> Collection<UserObject> {
        self.db_service.database("Authorization").collection("users")
    }
    pub async fn email_collection(&self) -> Collection<UserEmailObject> {
        self.db_service.database("Authorization").collection("email")
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserObject {
    /// The user's ID
    id: ObjectId,
    /// The user's username
    username: String,
    password: UserPasswordObject,
    email: UserEmailObject,
    /// Trigger risk control, forcing re-authentication when logging in
    login_verification_required: bool,
    banned_status: bool,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct ApiKeyObject {
    /// The user's ID
    id: ObjectId,
    user: ObjectId,
    expired_at: Timestamp,
    banned_status: bool,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct UserPasswordObject {
    /// The hashed password in database
    password: i64,
    /// The salt used to hash the password
    salt: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserEmailObject {
    /// The user's ID
    id: ObjectId,
    /// The user's email, must be lowercase
    email: String,
    /// Whether the user's email has been verified
    email_verified: bool,
    /// The code used to verify the email
    email_code: i64,
    /// The expiration time of the email code
    expired_at: Timestamp,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct UserEmailVerificationRequest {
    email: String,
    code: i64,
}

impl UserEmailVerificationRequest {
    pub async fn execute(&self, client: &PangaeaClient) -> Result<bool, mongodb::error::Error> {
        let filter = {
            let mut object = Document::new();
            object.insert("email.id", &Bson::ObjectId(ObjectId::parse_str(&self.email).unwrap()));
            object.insert("email.email_code", &Bson::Int64(self.code as i64));
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
    pub async fn execute(&self, client: &PangaeaClient) -> Result<bool, mongodb::error::Error> {
        todo!()
    }
}
