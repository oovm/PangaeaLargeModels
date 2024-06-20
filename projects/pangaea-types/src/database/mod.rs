use mongodb::bson::oid::ObjectId;
use mongodb::{Client, Collection};
use mongodb::bson::{doc, Timestamp};
use mongodb::options::{ClientOptions, ServerApi, ServerApiVersion};
use mongodb::results::InsertOneResult;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

mod users;

pub struct PangaeaClient {
    db_service: Client,
}


impl PangaeaClient {
    // 获取或创建用户集合
    async fn users_collection(&self) -> Collection<UserObject> {
        self.db_service.database("my_database").collection("users")
    }
}

impl PangaeaClient {
    async fn users_collection(&self) -> Collection<UserObject> {
        self.db_service.database("Authorization").collection("users")
    }
    async fn email_collection(&self) -> Collection<UserEmailObject> {
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
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserPasswordObject {
    /// The hashed password in database
    password: u64,
    /// The salt used to hash the password
    salt: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserEmailObject {
    /// The user's ID
    id: ObjectId,
    /// The user's email, must be lowercase
    email: String,
    /// Whether the user's email has been verified
    email_verified: bool,
    /// The code used to verify the email
    email_code: u64,
    /// The expiration time of the email code
    expired_at: Timestamp,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct UserEmailRegistrationRequest {
    /// The user's email
    email: String,
    /// The hashed password generate at frontend
    #[serde(default)]
    password: u64,
    #[serde(default)]
    username: String,
}

impl UserEmailRegistrationRequest {
    pub fn execute(&self, request: PangaeaClient) -> Result<InsertOneResult, mongodb::error::Error> {}
}


#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct UserEmailVerificationRequest {
    email: ObjectId,
    code: u64,
}

impl UserEmailVerificationRequest {
    pub fn execute(&self, request: PangaeaClient) -> Result<InsertOneResult, mongodb::error::Error> {}
}


