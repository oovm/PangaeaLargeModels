use super::*;

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct StableDiffusionCommon {
    pub id: u64,
    pub positive_prompt: String,
    pub negative_prompt: String,
    pub seed: i64,
    pub clip_stop_at_last_layers: usize,
}

/// A struct that holds the prompts for the image generation.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct StableDiffusion14Task {
    #[serde(flatten)]
    pub common: StableDiffusionCommon,
    pub extension: Vec<DiffuserExtension>,
}

/// A struct that holds the prompts for the image generation.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct StableDiffusion15Task {
    #[serde(flatten)]
    pub common: StableDiffusionCommon,
    pub extension: Vec<DiffuserExtension>,
}

/// A struct that holds the prompts for the image generation.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct StableDiffusion20Task {
    #[serde(flatten)]
    pub common: StableDiffusionCommon,
    pub extension: Vec<DiffuserExtension>,
}

/// A struct that holds the prompts for the image generation.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct StableDiffusion21Task {
    #[serde(flatten)]
    pub common: StableDiffusionCommon,
    pub extension: Vec<DiffuserExtension>,
}

/// A struct that holds the prompts for the image generation.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct StableDiffusion30Task {
    #[serde(flatten)]
    pub common: StableDiffusionCommon,
    pub extension: Vec<DiffuserExtension>,
}


#[derive(Copy, Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub enum DiffuserExtension {
    Lora,
    Dora,
}
