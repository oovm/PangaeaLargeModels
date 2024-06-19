use std::path::Path;
use mongodb::Client;
use schemars::{JsonSchema, schema_for};
use serde::Serialize;
use serde_json::ser::PrettyFormatter;
use serde_json::Serializer;
use pangaea_types::tasks::{PangaeaTaskType, StableDiffusion15Task, StableDiffusionCommon};

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

pub struct PangaeaDatabase {
    client: Client
}


fn test_mongo() {
    use serde::{Deserialize, Serialize};

    use mongodb::{Client, options::ClientOptions};

    // Parse a connection string into an options struct.
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await?;

    // Manually set an option.
    client_options.app_name = Some("My App".to_string());

    // Get a handle to the deployment.
    let client = Client::with_options(client_options)?;

    // List the names of the databases in that deployment.
    for db_name in client.list_database_names(None, None).await? {
        println!("{}", db_name);
    }

    // Get a handle to a database.
    let db = client.database("mydb");

    // List the names of the collections in that database.
    for collection_name in db.list_collection_names(None).await? {
        println!("{}", collection_name);
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Book {
        title: String,
        author: String,
    }

    // Get a handle to a collection of `Book`.
    let typed_collection = db.collection::<Book>("books");

    let books = vec![
        Book {
            title: "The Grapes of Wrath".to_string(),
            author: "John Steinbeck".to_string(),
        },
        Book {
            title: "To Kill a Mockingbird".to_string(),
            author: "Harper Lee".to_string(),
        },
    ];

    // Insert the books into "mydb.books" collection, no manual conversion to BSON necessary.
    typed_collection.insert_many(books, None).await?;
}