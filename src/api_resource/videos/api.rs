use super::data::*;
use serde::{Deserialize, Serialize};

const API_URL: &str = "https://open.bigmodel.cn/api/paas/v4/videos/generations";
const API_URL_ASYNC: &str = "https://open.bigmodel.cn/api/paas/v4/async-result";

#[derive(Serialize, Deserialize, Debug, Clone)]
#[non_exhaustive]
pub struct VideosApiRequest {
    /// model name
    model: String,
    /// prompt text
    prompt: String,
    /// quality
    #[serde(skip_serializing_if = "Option::is_none")]
    quality: Option<VideoQuality>,
    /// with_audio
    #[serde(skip_serializing_if = "Option::is_none")]
    with_audio: Option<bool>,
    /// image_url
    #[serde(skip_serializing_if = "Option::is_none")]
    image_url: Option<String>,
    /// size
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<VideoSize>,
    /// fps
    #[serde(skip_serializing_if = "Option::is_none")]
    fps: Option<VideoFPS>,
    /// request_id
    #[serde(skip_serializing_if = "Option::is_none")]
    request_id: Option<String>,
    /// user_id
    #[serde(skip_serializing_if = "Option::is_none")]
    user_id: Option<String>,
}
impl VideosApiRequest {
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

pub struct VideosApiRequestBuilder {
    /// model name
    model: String,
    /// prompt text
    prompt: String,
    /// quality
    quality: Option<VideoQuality>,
    /// with_audio
    with_audio: Option<bool>,
    /// image_url
    image_url: Option<String>,
    /// size
    size: Option<VideoSize>,
    /// fps
    fps: Option<VideoFPS>,
    /// request_id
    request_id: Option<String>,
    /// user_id
    user_id: Option<String>,
    /// response_id
    response_id: String,
}

impl VideosApiRequestBuilder {
    // Public function `new` that creates a new instance of the struct.
    // It takes a string reference `model` as a parameter and returns an instance of `Self`.
    pub fn new(model: &str) -> Self {
        // Create a new instance of `Self` (the struct this function is defined in).
        // Initialize the struct fields as follows:
        Self {
            // Set the `model` field to the value of `model` converted to a `String`.
            model: model.to_string(),
            // Initialize the `prompt` field as an empty `String`.
            prompt: String::new(),
            // Initialize the `quality` field as `None`, indicating no quality is set initially.
            quality: None,
            // Initialize the `with_audio` field as `None`, indicating no audio is set initially.
            with_audio: None,
            // Initialize the `image_url` field as `None`, indicating no image URL is set initially.
            image_url: None,
            // Initialize the `size` field as `None`, indicating no size is set initially.
            size: None,
            // Initialize the `fps` field as `None`, indicating no fps is set initially.
            fps: None,
            // Initialize the `request_id` field as `None`, indicating no request ID is set initially.
            request_id: None,
            // Initialize the `user_id` field as `None`, indicating no user ID is set initially.
            user_id: None,
            // Initialize the `response_id` field as `None`, indicating no response ID is set initially.
            response_id: String::new(),
        }
    }

    pub fn prompt(mut self, prompt: &str) -> Self {
        // Assign the provided `prompt` to the struct's `prompt` field.
        // Convert the string slice to a `String` type and wrap it in a `Some` variant of the `Option` enum.
        self.prompt = prompt.to_string();
        // Return the modified instance of the struct.
        self
    }

    pub fn quality(mut self, quality: VideoQuality) -> Self {
        // Assign the provided `quality` to the struct's `quality` field.
        // Convert the string slice to a `String` type and wrap it in a `Some` variant of the `Option` enum.
        self.quality = Some(quality);
        // Return the modified instance of the struct.
        self
    }

    pub fn with_audio(mut self, with_audio: bool) -> Self {
        // Assign the provided `with_audio` to the struct's `with_audio` field.
        // Convert the string slice to a `String` type and wrap it in a `Some` variant of the `Option` enum.
        self.with_audio = Some(with_audio);
        // Return the modified instance of the struct.
        self
    }

    pub fn image_url(mut self, image_url: &str) -> Self {
        // Assign the provided `image_url` to the struct's `image_url` field.
        // Convert the string slice to a `String` type and wrap it in a `Some` variant of the `Option` enum.
        self.image_url = Some(image_url.to_string());
        // Return the modified instance of the struct.
        self
    }

    pub fn size(mut self, size: VideoSize) -> Self {
        // Assign the provided `size` to the struct's `size` field.
        // Convert the string slice to a `String` type and wrap it in a `Some` variant of the `Option` enum.
        self.size = Some(size);
        // Return the modified instance of the struct.
        self
    }

    pub fn fps(mut self, fps: VideoFPS) -> Self {
        // Assign the provided `fps` to the struct's `fps` field.
        // Convert the string slice to a `String` type and wrap it in a `Some` variant of the `Option` enum.
        self.fps = Some(fps);
        // Return the modified instance of the struct.
        self
    }

    pub fn request_id(mut self, request_id: &str) -> Self {
        // Assign the provided `request_id` to the struct's `request_id` field.
        // Convert the string slice to a `String` type and wrap it in a `Some` variant of the `Option` enum.
        self.request_id = Some(request_id.to_string());
        // Return the modified instance of the struct.
        self
    }

    pub fn user_id(mut self, user_id: &str) -> Self {
        // Assign the provided `user_id` to the struct's `user_id` field.
        // Convert the string slice to a `String` type and wrap it in a `Some` variant of the `Option` enum.
        self.user_id = Some(user_id.to_string());
        // Return the modified instance of the struct.
        self
    }

    pub fn response_id(mut self, response_id: &str) -> Self {
        // Assign the provided `response_id` to the struct's `response_id` field.
        // Convert the string slice to a `String` type and wrap it in a `Some` variant of the `Option` enum.
        self.response_id = response_id.to_string();
        // Return the modified instance of the struct.
        self
    }

    pub fn build(self) -> (String, VideosApiRequest) {
        // Return a tuple containing two elements:
        // 1. A new `String` created from the constant `API_URL`.
        // 2. The current instance (`self`) of the struct, cast to `VideosApiRequest`.
        (
            API_URL.to_string(), // Convert the constant to a `String` type.
            VideosApiRequest {
                model: self.model,
                prompt: self.prompt,
                quality: self.quality,
                with_audio: self.with_audio,
                image_url: self.image_url,
                size: self.size,
                fps: self.fps,
                request_id: self.request_id,
                user_id: self.user_id,
            },
        )
    }

    pub fn build_response(self) -> (String, VideosApiAsynRequest) {
        let api_url = format!("{}/{}", API_URL_ASYNC, self.response_id);
        (
            api_url,
            VideosApiAsynRequest {
                id: self.response_id,
            },
        )
    }
} // Implement the `Build` trait for the `VideosApiRequestBuilder` struct.

#[derive(Serialize, Deserialize, Debug, Clone)]
#[non_exhaustive]
pub struct VideosApiAsynRequest {
    // Define a struct named `VideosApiAsynRequest` with the following fields:
    id: String,
}

impl VideosApiAsynRequest {
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
