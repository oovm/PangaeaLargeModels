use super::*;

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct StableDiffusionCommon {
    /// The unique id of the task
    pub task_id: u64,
    /// The token owner will pay the fee
    pub api_token: u64,
    /// The positive prompt
    #[serde(default)]
    pub positive_prompt: String,
    /// The negative prompt
    #[serde(default)]
    pub negative_prompt: String,
    pub steps: u8,
    #[serde(default = "default_cfg_scale")]
    #[schemars(range(min = 1, max = 10))]
    pub cfg_scale: f32,
    /// The random seed
    #[serde(default)]
    pub seed: i64,
    #[serde(default)]
    pub high_resolution: Option<HighResolutionFixer>,
}

fn default_clip_skip() -> usize {
    return 2;
}
fn default_cfg_scale() -> f32 {
    return 7.0;
}

/// A struct that holds the prompts for the image generation.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct StableDiffusion14Task {
    #[serde(flatten)]
    pub common: StableDiffusionCommon,
    /// The CLIP will stop at the last layers
    #[serde(default = "default_clip_skip")]
    pub clip_skip: usize,
    pub extension: Vec<DiffuserExtension>,
}

/// A struct that holds the prompts for the image generation.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct StableDiffusion15Task {
    #[serde(flatten)]
    pub common: StableDiffusionCommon,
    /// The CLIP will stop at the last layers
    #[serde(default = "default_clip_skip")]
    pub clip_skip: usize,
    pub extension: Vec<DiffuserExtension>,
}

/// A struct that holds the prompts for the image generation.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct StableDiffusion20Task {
    #[serde(flatten)]
    pub common: StableDiffusionCommon,
    /// The CLIP will stop at the last layers
    #[serde(default = "default_clip_skip")]
    pub clip_skip: usize,
    pub extension: Vec<DiffuserExtension>,
}

/// A struct that holds the prompts for the image generation.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct StableDiffusion21Task {
    #[serde(flatten)]
    pub common: StableDiffusionCommon,
    /// The CLIP will stop at the last layers
    #[serde(default = "default_clip_skip")]
    pub clip_skip: usize,
    pub extension: Vec<DiffuserExtension>,
}

/// A struct that holds the prompts for the image generation.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct StableDiffusion30Task {
    #[serde(flatten)]
    pub common: StableDiffusionCommon,
    /// The CLIP will stop at the last layers
    #[serde(default = "default_clip_skip")]
    pub clip_skip: usize,
    pub extensions: Vec<DiffuserExtension>,
}


#[derive(Copy, Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type")]
pub enum DiffuserExtension {
    // #[serde(rename = "Stable Diffusion v1.4")]
    TextEmbedding(TextEmbedding),
    #[serde(rename = "LoRA")]
    Lora(StableDiffusionLora),
    // #[serde(rename = "Stable Diffusion v1.4")]
    Dora,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct TextEmbedding {
    pub model_id: u64,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct HighResolutionFixer {
    #[serde(default = "default_scale")]
    pub scale: f32,
    #[serde(default)]
    pub method: HighResolutionFixMethod,
}

#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize, JsonSchema)]
pub enum HighResolutionFixMethod {
    #[default]
    EsrGan,
}

fn default_scale() -> f32 {
    2.0
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct StableDiffusionLora {
    pub model_id: u64,
    #[serde(default = "default_strength")]
    pub strength: f32,
}

fn default_strength() -> f32 {
    0.8
}