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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(into = "u32", from = "u32")]
pub enum VideoFPS {
    VideoFps30,
    VideoFps60,
}

impl From<VideoFPS> for u32 {
    fn from(fps: VideoFPS) -> Self {
        match fps {
            VideoFPS::VideoFps30 => 30,
            VideoFPS::VideoFps60 => 60,
        }
    }
}

impl From<u32> for VideoFPS {
    fn from(value: u32) -> Self {
        match value {
            30 => VideoFPS::VideoFps30,
            60 => VideoFPS::VideoFps60,
            _ => VideoFPS::VideoFps30, // 默认值
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(into = "u32", from = "u32")]
pub enum VideoDuration {
    VideoDuration5,
    VideoDuration10,
}

impl From<VideoDuration> for u32 {
    fn from(duration: VideoDuration) -> Self {
        match duration {
            VideoDuration::VideoDuration5 => 5,
            VideoDuration::VideoDuration10 => 10,
        }
    }
}

impl From<u32> for VideoDuration {
    fn from(value: u32) -> Self {
        match value {
            5 => VideoDuration::VideoDuration5,
            10 => VideoDuration::VideoDuration10,
            _ => VideoDuration::VideoDuration5, // 默认值
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use log::info;
    use serde_json;

    #[test]
    fn test_video_fps_serialization() {
        let fps30 = VideoFPS::VideoFps30;
        let fps60 = VideoFPS::VideoFps60;

        let json30 = serde_json::to_string(&fps30).unwrap();
        let json60 = serde_json::to_string(&fps60).unwrap();

        assert_eq!(json30, "30");
        assert_eq!(json60, "60");
    }

    #[test]
    fn test_video_fps_deserialization() {
        let fps30: VideoFPS = serde_json::from_str("30").unwrap();
        let fps60: VideoFPS = serde_json::from_str("60").unwrap();

        assert!(matches!(fps30, VideoFPS::VideoFps30));
        assert!(matches!(fps60, VideoFPS::VideoFps60));
    }

    #[test]
    fn test_video_fps_round_trip() {
        let original_fps30 = VideoFPS::VideoFps30;
        let original_fps60 = VideoFPS::VideoFps60;

        // 序列化然后反序列化
        let json30 = serde_json::to_string(&original_fps30).unwrap();
        let json60 = serde_json::to_string(&original_fps60).unwrap();

        let deserialized30: VideoFPS = serde_json::from_str(&json30).unwrap();
        let deserialized60: VideoFPS = serde_json::from_str(&json60).unwrap();

        assert!(matches!(deserialized30, VideoFPS::VideoFps30));
        assert!(matches!(deserialized60, VideoFPS::VideoFps60));
    }

    #[test]
    fn test_video_fps_invalid_value_defaults() {
        // 测试无效值是否默认为 VideoFps30
        let invalid_fps: VideoFPS = serde_json::from_str("120").unwrap();
        assert!(matches!(invalid_fps, VideoFPS::VideoFps30));

        let another_invalid: VideoFPS = serde_json::from_str("0").unwrap();
        assert!(matches!(another_invalid, VideoFPS::VideoFps30));
    }

    #[test]
    fn test_video_fps_in_struct() {
        #[derive(Serialize, Deserialize, Debug)]
        struct VideoConfig {
            fps: VideoFPS,
            name: String,
        }

        let config = VideoConfig {
            fps: VideoFPS::VideoFps60,
            name: "test_video".to_string(),
        };

        let json = serde_json::to_string(&config).unwrap();
        info!("Serialized config: {}", json);

        // 验证 fps 字段被序列化为数字
        assert!(json.contains("\"fps\":60"));

        // 测试反序列化
        let deserialized: VideoConfig = serde_json::from_str(&json).unwrap();
        assert!(matches!(deserialized.fps, VideoFPS::VideoFps60));
        assert_eq!(deserialized.name, "test_video");
    }
}
