use super::*;

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct StableDiffusionCommon {
    pub task_id: u64,
    pub user_id: u64,
    #[serde(default)]
    pub positive_prompt: String,
    #[serde(default)]
    pub negative_prompt: String,
    #[serde(default)]
    pub seed: i64,
    #[serde(default = "clip_stop_default")]
    pub clip_stop_at_last_layers: usize,
    pub high_resolution: Option<HighResolutionFixer>,
}

fn clip_stop_default() -> usize {
    return 2;
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
    pub extensions: Vec<DiffuserExtension>,
}


#[derive(Copy, Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type")]
pub enum DiffuserExtension {
    // #[serde(rename = "Stable Diffusion v1.4")]
    TextEmbedding(TextEmbedding),
    // #[serde(rename = "Stable Diffusion v1.4")]
    Lora(StableDiffusionLora),
    // #[serde(rename = "Stable Diffusion v1.4")]
    Dora,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct TextEmbedding {}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct HighResolutionFixer {}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct StableDiffusionLora {
    pub strength: f32,
}