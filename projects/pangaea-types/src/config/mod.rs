use std::env::VarError;
use schemars::JsonSchema;

#[derive(JsonSchema)]
pub struct PangaeaMatrixConfig {
    pub civit_key: i32,
    pub my_bool: bool,
    pub my_nullable_enum: Option<DatabaseConfig>,
}

#[derive(JsonSchema)]
pub enum DatabaseConfig {
    StringNewType(String),
    StructVariant { floats: Vec<f32> },
}


use mongodb::{bson::doc, options::{ClientOptions, ServerApi, ServerApiVersion}, Client};

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let mut client_options = match std::env::var("MONGODB_URI") {
        Ok(url) => { ClientOptions::parse(url).await? }
        Err(e) => {
            match e {
                VarError::NotPresent => {
                    panic!("MONGODB_URI is not set")
                }
                VarError::NotUnicode(_) => {
                    panic!("MONGODB_URI is not a valid UTF-8 string")
                }
            }
        }
    };
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);
    // Get a handle to the cluster
    let client = Client::with_options(client_options)?;
    // Ping the server to see if you can connect to the cluster
    client
        .database("admin")
        .run_command(doc! {"ping": 1}, None)
        .await?;
    println!("Pinged your deployment. You successfully connected to MongoDB!");
    Ok(())
}