use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

mod stable_diffusion;

pub use self::stable_diffusion::{
    StableDiffusion14Task, StableDiffusion15Task, StableDiffusion20Task, StableDiffusion21Task, StableDiffusion30Task,
    StableDiffusionCommon,
};

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type")]
pub enum PangaeaTaskType {
    #[serde(rename = "Stable Diffusion v1.4")]
    SD14(StableDiffusion14Task),
    #[serde(rename = "Stable Diffusion v1.5")]
    SD15(StableDiffusion15Task),
    #[serde(rename = "Stable Diffusion v2.0")]
    SD20(StableDiffusion20Task),
    #[serde(rename = "Stable Diffusion v2.1")]
    SD21(StableDiffusion21Task),
    #[serde(rename = "Stable Diffusion XL v1.0")]
    SDXL(StableDiffusion30Task),
    #[serde(rename = "Stable Diffusion XL PonyV6")]
    Pony(StableDiffusion30Task),
    #[serde(rename = "Stable Diffusion 3.0")]
    SD30(StableDiffusion30Task),
}
