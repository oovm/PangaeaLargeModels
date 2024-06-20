use std::path::Path;
use mongodb::{Client, Collection};
use schemars::{JsonSchema, schema_for};
use serde::{Deserialize, Serialize};
use serde_json::ser::PrettyFormatter;
use serde_json::Serializer;
use pangaea_types::tasks::{PangaeaTaskType, StableDiffusion15Task, StableDiffusionCommon};
use std::env::VarError;
use mongodb::{bson::doc, options::{ClientOptions, ServerApi, ServerApiVersion}};
use mongodb::bson::oid::ObjectId;
use mongodb::options::IndexOptions;
use pangaea_types::database::PangaeaDatabase;

#[test]
fn ready() {
    println!("it works!")
}


#[test]
fn test_schema() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let data = PangaeaTaskType::SD15(StableDiffusion15Task {
        common: StableDiffusionCommon {
            task_id: 0,
            api_token: 0,
            positive_prompt: "".to_string(),
            negative_prompt: "".to_string(),
            steps: 20,
            cfg_scale: 10.0,
            seed: 0,
            ensd: 0,
            high_resolution: None,
        },
        clip_skip: 2,
        extension: vec![],
    });
    println!("{}", serde_json::to_string_pretty(&data).unwrap());

    let schema = schema_for!(PangaeaTaskType);
    println!("{}", serde_json::to_string_pretty(&schema).unwrap());
    json_writer(&here.join("tests/stable_diffusion_15.json"), &data)
}

fn json_writer<T: Serialize + JsonSchema>(path: &Path, data: &T) {
    assert_eq!(path.extension().unwrap(), "json");
    {
        let schema_path = path.with_extension("schema.json");
        let schema_writer = std::fs::File::create(schema_path).unwrap();
        let mut serializer = Serializer::with_formatter(schema_writer, PrettyFormatter::with_indent(b"    "));
        schema_for!(T).serialize(&mut serializer).unwrap();
    }
    {
        let value_writer = std::fs::File::create(path).unwrap();
        let mut serializer = Serializer::with_formatter(value_writer, PrettyFormatter::with_indent(b"    "));
        data.serialize(&mut serializer).unwrap();
    }
}




#[tokio::test]
async fn main() -> mongodb::error::Result<()> {
    let mut client = match std::env::var("MONGODB_URI") {
        Ok(url) => { PangaeaDatabase::new(&url).await? }
        Err(e) => {
            match e {
                VarError::NotPresent => {
                    panic!("Set environment variable `MONGODB_URI` to startup the database!")
                }
                VarError::NotUnicode(_) => {
                    panic!("Environment variable `MONGODB_URI` is not a valid UTF-8 string")
                }
            }
        }
    };
    client
        .database("sample_mflix")
        .run_command(doc! {"ping": 1}, None)
        .await?;
    println!("Pinged your deployment. You successfully connected to MongoDB!");
    Ok(())
}