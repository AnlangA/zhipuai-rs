///! response of chat api

use crate::error::ZhipuApiError;
use reqwest::Response;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ImagesChatApiResponse {
    created: usize,
    data: Vec<Url>,
    content_filter: Option<Vec<ContentFilter>>
}

impl ImagesChatApiResponse {
    pub fn urls(&self) -> Vec<String> {
        self.data.iter().map(|x| x.url.clone()).collect()
    }
    pub fn content_filter(&self) -> &[ContentFilter] {
        self.content_filter.as_deref().unwrap_or(&[])
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Url {
    url: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContentFilter {
    role: String,
    level: usize
}

pub async fn images_response_context(response: Response) -> Result<ImagesChatApiResponse, ZhipuApiError> {
    if response.status().is_success() {
        let response = response.text().await?;
        let response: ImagesChatApiResponse = serde_json::from_str(&response)?;
        Ok(response)
    }else {
        Err(ZhipuApiError::StatusCode(format!(
            "Failed to fetch data: {}",
            response.status()
        )))
    }
}

