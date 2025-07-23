///! response of chat api
use crate::error::ZhipuApiError;
use reqwest::Response;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct VideosChatApiResponse {
    request_id: Option<String>,
    id: Option<String>,
    model: Option<String>,
    task_status: Option<String>,
}

impl VideosChatApiResponse {
    pub fn request_id(&self) -> String {
        self.request_id.as_deref().unwrap_or_default().to_string()
    }

    pub fn id(&self) -> String {
        self.id.as_deref().unwrap_or_default().to_string()
    }

    pub fn model(&self) -> String {
        self.model.as_deref().unwrap_or_default().to_string()
    }

    pub fn task_status(&self) -> String {
        self.task_status.as_deref().unwrap_or_default().to_string()
    }
}

pub async fn videos_response_context(
    response: Response,
) -> Result<VideosChatApiResponse, ZhipuApiError> {
    if response.status().is_success() {
        let response = response.text().await?;
        let response: VideosChatApiResponse = serde_json::from_str(&response)?;
        Ok(response)
    } else {
        Err(ZhipuApiError::StatusCode(format!(
            "Failed to fetch data: {}",
            response.status()
        )))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VideosChatApiAsynResponse {
    model: Option<String>,
    video_result: Option<Vec<Video>>,
    request_id: Option<String>,
    task_status: Option<String>,
}

impl VideosChatApiAsynResponse {
    pub fn model(&self) -> String {
        self.model.as_deref().unwrap_or_default().to_string()
    }
    pub fn video_result(&self) -> &[Video] {
        self.video_result.as_deref().unwrap_or(&[])
    }
    pub fn request_id(&self) -> String {
        self.request_id.as_deref().unwrap_or_default().to_string()
    }
    pub fn task_status(&self) -> String {
        self.task_status.as_deref().unwrap_or_default().to_string()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Video {
    url: String,
    cover_image_url: String,
}

impl Video {
    pub fn url(&self) -> &str {
        &self.url
    }
    pub fn cover_image_url(&self) -> &str {
        &self.cover_image_url
    }
}

pub async fn videos_asyn_response_context(
    response: Response,
) -> Result<VideosChatApiAsynResponse, ZhipuApiError> {
    if response.status().is_success() {
        let response = response.text().await?;
        let response: VideosChatApiAsynResponse = serde_json::from_str(&response)?;
        Ok(response)
    } else {
        Err(ZhipuApiError::StatusCode(format!(
            "Failed to fetch data: {}",
            response.status()
        )))
    }
}
