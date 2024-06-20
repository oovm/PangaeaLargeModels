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
