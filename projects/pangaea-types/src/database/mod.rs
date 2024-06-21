use crate::Result;
use chrono::{DateTime, Utc};
use mongodb::{
    bson::{doc, oid::ObjectId, Bson, Document, Timestamp},
    options::{ClientOptions, ServerApi, ServerApiVersion},
    results::UpdateResult,
    Client, Collection,
};
use rand::Rng;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Formatter};

pub mod models;
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
    pub async fn new(uri: &str) -> Result<PangaeaClient> {
        let mut client_options = ClientOptions::parse(uri).await?;
        client_options.server_api = Some(ServerApi::builder().version(ServerApiVersion::V1).build());
        let client = Client::with_options(client_options)?;
        Ok(Self { db_service: client })
    }
    pub async fn users_collection(&self) -> Collection<UserObject> {
        self.db_service.database("Authorization").collection("users")
    }
    pub async fn email_collection(&self) -> Collection<EmailObject> {
        self.db_service.database("Authorization").collection("email")
    }
    pub async fn stable_diffusion(&self) -> Collection<StableDiffusion15Object> {
        self.db_service.database("LargeModel").collection("sd15")
    }
}

pub struct StableDiffusion15Object {
    /// The user's ID
    #[serde(rename = "_id")]
    id: ObjectId,

    place: StableDiffusionPlace,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type")]
pub enum StableDiffusionPlace {
    CivitAI { model_id: i64 },
    Aliyun { model_id: i64 },
}

pub fn expired_after_seconds(seconds: u32) -> Timestamp {
    let now: DateTime<Utc> = Utc::now();
    let seconds_since_epoch = now.timestamp() as u32;
    Timestamp { time: seconds_since_epoch + seconds, increment: 0 }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserObject {
    /// The user's ID
    #[serde(rename = "_id")]
    id: ObjectId,
    /// The user's username
    username: String,
    nickname: String,
    password: UserPasswordObject,
    email: ObjectId,
    /// Trigger risk control, forcing re-authentication when logging in
    login_verification_required: bool,
    banned_status: bool,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct ApiKeyObject {
    /// The user's ID
    #[serde(rename = "_id")]
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
pub struct EmailObject {
    /// The user's ID
    #[serde(rename = "_id")]
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
