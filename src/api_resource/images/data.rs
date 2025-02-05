//！#images model data

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ImageSize {
    #[serde(rename = "1024x1024")]
    Size1024x1024,
    #[serde(rename = "768x1344")]
    Size768x1344,
    #[serde(rename = "864x1152")]
    Size864x1152,
    #[serde(rename = "1344x768")]
    Size1344x768,
    #[serde(rename = "1152x864")]
    Size1152x864,
    #[serde(rename = "1440x720")]
    Size1440x720,
    #[serde(rename = "720x1440")]
    Size720x1440,
}