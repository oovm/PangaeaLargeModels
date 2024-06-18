use schemars::{JsonSchema, schema_for};

#[derive(JsonSchema)]
pub struct DiffuserMatrixConfig {
    pub my_int: i32,
    pub my_bool: bool,
    pub my_nullable_enum: Option<DatabaseConfig>,
}

#[derive(JsonSchema)]
pub enum DatabaseConfig {
    StringNewType(String),
    StructVariant { floats: Vec<f32> },
}

#[test]
fn test() {
    let schema = schema_for!(DiffuserMatrixConfig);
    println!("{}", serde_json::to_string_pretty(&schema).unwrap());
}