use std::path::Path;
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