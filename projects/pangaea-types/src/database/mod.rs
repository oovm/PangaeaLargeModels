use mongodb::bson::oid::ObjectId;
use mongodb::{Client, Collection};
use mongodb::bson::doc;
use mongodb::options::{ClientOptions, ServerApi, ServerApiVersion};
use mongodb::results::InsertOneResult;
use serde::{Deserialize, Serialize};

pub struct PangaeaDatabase {
    client: Client,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserObject {
    /// The user's ID
    id: ObjectId,
    /// The user's username
    username: String,
    /// The user's email
    email: String,
    /// The hashed password
    password: u64,
    /// The salt used to hash the password
    salt: u64,
}

impl PangaeaDatabase {
    pub async fn new(uri: &str) -> mongodb::error::Result<PangaeaDatabase> {
        let mut client_options = ClientOptions::parse(uri).await?;
        let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
        client_options.server_api = Some(server_api);
        Ok(Self {
            client: Client::with_options(client_options)?
        })
    }
}

impl PangaeaDatabase {
    pub async fn register_user(&self, username: &str, email: &str, password: u64) -> Result<InsertOneResult, mongodb::error::Error> {
        // 检查用户名和邮箱是否已存在
        if self.is_username_taken(username).await? || self.is_email_taken(email).await? {
            return Err(mongodb::error::Error::from(mongodb::error::ErrorKind::SessionsNotSupported));
        }
        // 创建新用户并插入数据库
        let new_user = UserObject {
            id: ObjectId::new(),
            username: username.to_string(),
            email: email.to_string(),
            password: 0,
            salt: 0,
        };
        let users_collection = self.get_or_create_users_collection().await?;
        users_collection.insert_one(new_user, None).await
    }

    // 检查用户名是否已被使用
    async fn is_username_taken(&self, username: &str) -> Result<bool, mongodb::error::Error> {
        let filter = doc! {"username": username};
        let count = self.get_or_create_users_collection().await?.count_documents(filter, None).await?;
        Ok(count > 0)
    }

    async fn is_email_taken(&self, email: &str) -> Result<bool, mongodb::error::Error> {
        let filter = doc! {"email": email};
        let count = self.get_or_create_users_collection().await?.count_documents(filter, None).await?;
        Ok(count > 0)
    }
}

impl PangaeaDatabase {
    pub async fn login_user(&self, identifier: &str, password: &str) -> Result<Option<UserObject>, mongodb::error::Error> {
        let filter = doc! { "$or": [ { "username": identifier }, { "email": identifier } ] };
        let users_collection = self.get_or_create_users_collection().await?;
        let user_opt: Option<UserObject> = users_collection.find_one(filter, None).await?;
        match user_opt {
            Some(user) => Ok(Some(user)),
            None => Ok(None),
        }
    }
}