use schemars::{JsonSchema, schema_for};

#[derive(JsonSchema)]
pub struct DeusMatrixConfig {
    pub civit_key: i32,
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
    let schema = schema_for!(DeusMatrixConfig);
    println!("{}", serde_json::to_string_pretty(&schema).unwrap());
}