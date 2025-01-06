///! response of chat api
use super::data::*;
use async_stream::try_stream;
use bytes::{Buf, BufMut, BytesMut};
use futures::StreamExt;
use reqwest::Response;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse {
    id: Option<String>,
    created: Option<u64>,
    model: Option<String>,
    choices: Option<Vec<Choice>>,
    request_id: Option<String>,
    usage: Option<Usage>,
    web_search: Option<Vec<WebSearchResponse>>,
}
impl ApiResponse {
    pub fn get_choices(&self) -> Option<&Vec<Choice>> {
        self.choices.as_ref()
    }
    pub fn get_web_search(&self) -> &Option<Vec<WebSearchResponse>> {
        &self.web_search
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Delta {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChoiceStream {
    index: u32,
    delta: Delta,
}
impl ChoiceStream {
    pub fn get_content(&self) -> &String {
        &self.delta.content
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiResponseStream {
    id: Option<String>,
    created: Option<u64>,
    model: Option<String>,
    choices: Option<Vec<ChoiceStream>>,
    usage: Option<Usage>,
    web_search: Option<Vec<WebSearchResponse>>,
}
impl ApiResponseStream {
    pub fn get_choices(&self) -> Option<&Vec<ChoiceStream>> {
        self.choices.as_ref()
    }
    pub fn get_web_search(&self) -> &Option<Vec<WebSearchResponse>> {
        &self.web_search
    }
}

type Result<T> = std::result::Result<T, Box<dyn Error>>;

/// Fetches the entire response body and returns it.
pub async fn response_all(response: Response) -> Result<String> {
    if response.status().is_success() {
        let response_text = response.text().await?;
        Ok(response_text)
    } else {
        Err(format!("Failed to fetch data: {}", response.status()).into())
    }
}

/// Parses and returns specific fields from the response body.
pub async fn response_context(response: Response) -> Result<ApiResponse> {
    if response.status().is_success() {
        let response_text = response.text().await?;
        let api_response: ApiResponse = serde_json::from_str(&response_text)?;
        Ok(api_response)
    } else {
        Err(format!("Failed to fetch data: {}", response.status()).into())
    }
}

/// Processes the response body as a stream, parsing chunks and yielding results.
pub fn response_context_stream(response: Response) -> impl futures::Stream<Item = Result<String>> {
    try_stream! {
        if !response.status().is_success() {
            Err(format!("Failed to fetch data: {}", response.status()))?;
        }
        let mut response_text = response.bytes_stream();
        let mut buffer = BytesMut::new();
        let mut string_buffer = String::new();
        while let Some(chunk) = response_text.next().await {
            let bytes = chunk?;
            buffer.put(bytes);
            // Handle UTF-8 decoding
            decode_utf8(&mut buffer, &mut string_buffer)?;
            // Process JSON objects
            let processed_data = process_json_objects(&mut string_buffer)?;
            for data in processed_data {
                yield data;
            }
        }
        // Handle any remaining data
        if !string_buffer.is_empty() {
            yield format!("Remaining unprocessed data: {}", string_buffer);
        }
    }
}

/// Decodes UTF-8 encoded bytes into a string buffer.
fn decode_utf8(buffer: &mut BytesMut, string_buffer: &mut String) -> Result<()> {
    loop {
        match std::str::from_utf8(buffer) {
            Ok(s) => {
                string_buffer.push_str(s);
                buffer.clear();
                break;
            }
            Err(e) => {
                let valid_up_to = e.valid_up_to();
                if valid_up_to == 0 {
                    break;
                }
                let valid_str = unsafe { std::str::from_utf8_unchecked(&buffer[..valid_up_to]) };
                string_buffer.push_str(valid_str);
                buffer.advance(valid_up_to);
            }
        }
    }
    Ok(())
}

/// Processes JSON objects from a string buffer and returns a vector of processed data.
fn process_json_objects(string_buffer: &mut String) -> Result<Vec<String>> {
    let mut processed_data = Vec::new();
    while let Some(end) = string_buffer.find('\n') {
        let json_str = string_buffer[..end].trim();
        if !json_str.is_empty() && json_str != "data: [DONE]" {
            if let Some(json_str) = json_str.strip_prefix("data: ") {
                match serde_json::from_str::<ApiResponseStream>(json_str) {
                    Ok(api_response) => {
                        if let Some(choices) = api_response.get_choices() {
                            for message in choices {
                                processed_data.push(message.get_content().to_string());
                            }
                        }
                    }
                    Err(e) => {
                        match serde_json::from_str::<Value>(json_str) {
                           Ok(_) => processed_data.push(format!(
                               "JSON format is correct, but does not match ApiResponseStream structure: {}",
                               json_str
                           )),
                           Err(_) => processed_data.push(format!("Invalid JSON data: {}", json_str)),
                       }
                        processed_data.push(format!("Failed to parse API response: {}", e));
                    }
                }
            }
        }
        string_buffer.drain(..=end);
    }
    Ok(processed_data)
}
