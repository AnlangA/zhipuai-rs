///! response of chat api
use super::data::*;
use crate::error::ZhipuApiError;
use async_stream::try_stream;
use bytes::{Buf, BufMut, BytesMut};
use futures::StreamExt;
use reqwest::Response;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatApiResponse {
    id: Option<String>,
    created: Option<u64>,
    model: Option<String>,
    choices: Option<Vec<Choice>>,
    request_id: Option<String>,
    usage: Option<Usage>,
    web_search: Option<Vec<WebSearchResponse>>,
}
impl ChatApiResponse {
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
    content: Option<String>,
    tool_calls: Option<Vec<ToolCall>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChoiceStream {
    index: u32,
    delta: Delta,
}

impl ChoiceStream {
    pub fn get_content(&self) -> String {
        self.delta.content.clone().unwrap_or_default().clone()
    }

    pub fn get_tool_calls(&self) -> String {
        let data = self.delta.tool_calls.clone().unwrap_or_default();
        let mut string_data = String::new();
        for item in data {
            let item_data = format!("{}", item);
            string_data = format!("{}{}", string_data, item_data);
        }
        string_data
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChatApiResponseStream {
    id: Option<String>,
    created: Option<u64>,
    model: Option<String>,
    choices: Option<Vec<ChoiceStream>>,
    usage: Option<Usage>,
    web_search: Option<Vec<WebSearchResponse>>,
}

impl ChatApiResponseStream {
    pub fn get_choices(&self) -> Option<&Vec<ChoiceStream>> {
        self.choices.as_ref()
    }
    pub fn get_web_search(&self) -> &Option<Vec<WebSearchResponse>> {
        &self.web_search
    }
}

/// Fetches the entire response body and returns it.
pub async fn response_all(response: Response) -> Result<String, ZhipuApiError> {
    if response.status().is_success() {
        let response_text = response.text().await?;
        Ok(response_text)
    } else {
        Err(ZhipuApiError::StatusCode(format!(
            "Failed to fetch data: {}",
            response.status()
        )))
    }
}

/// Parses and returns specific fields from the response body.
pub async fn chat_response_context(response: Response) -> Result<ChatApiResponse, ZhipuApiError> {
    if response.status().is_success() {
        let response_text = response.text().await?;
        let api_response: ChatApiResponse = serde_json::from_str(&response_text)?;
        Ok(api_response)
    } else {
        Err(ZhipuApiError::StatusCode(format!(
            "Failed to fetch data: {}",
            response.status()
        )))
    }
}

/// Processes the response body as a stream, parsing chunks and yielding results.
pub fn response_context_stream(
    response: Response,
) -> impl futures::Stream<Item = Result<String, ZhipuApiError>> {
    try_stream! {
        if !response.status().is_success() {
            Err(ZhipuApiError::StatusCode(format!("Failed to fetch data: {}", response.status())))?;
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
fn decode_utf8(buffer: &mut BytesMut, string_buffer: &mut String) -> Result<(), ZhipuApiError> {
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
fn process_json_objects(string_buffer: &mut String) -> Result<Vec<String>, ZhipuApiError> {
    let mut processed_data = Vec::new();
    while let Some(end) = string_buffer.find('\n') {
        let json_str = string_buffer[..end].trim();
        if !json_str.is_empty() && json_str != "data: [DONE]" {
            if let Some(json_str) = json_str.strip_prefix("data: ") {
                match serde_json::from_str::<ChatApiResponseStream>(json_str) {
                    Ok(api_response) => {
                        if let Some(choices) = api_response.get_choices() {
                            for message in choices {
                                if !message.get_content().to_string().is_empty() {
                                    processed_data.push(message.get_content().to_string());
                                } else if !message.get_tool_calls().to_string().is_empty() {
                                    processed_data.push(message.get_tool_calls().to_string());
                                }
                            }
                        }
                    }
                    Err(e) => {
                        match serde_json::from_str::<Value>(json_str) {
                           Ok(_) => processed_data.push(format!(
                               "JSON format is correct, but does not match ChatApiResponseStream structure: {}",
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
