use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum VideoSize {
    #[serde(rename = "3840x2160")]
    Size3840x2160,
    #[serde(rename = "2048x1080")]
    Size2048x1080,
    #[serde(rename = "1920x1080")]
    Size1920x1080,
    #[serde(rename = "960x1280")]
    Size960x1280,
    #[serde(rename = "1280x960")]
    Size1280x960,
    #[serde(rename = "1024x1024")]
    Size1024x1024,
    #[serde(rename = "720x480")]
    Size720x480,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum VideoQuality {
    #[serde(rename = "speed")]
    VideoSpeed,
    #[serde(rename = "quality")]
    VideoQuality,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum VideoFPS {
    VideoFps30,
    VideoFps60,
}

impl VideoFPS {
    pub fn as_u32(&self) -> u32 {
        match self {
            VideoFPS::VideoFps30 => 30,
            VideoFPS::VideoFps60 => 60,
        }
    }
}
