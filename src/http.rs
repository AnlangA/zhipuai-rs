//! # LLM information post interface

use std::borrow::Cow;

pub async fn post<'a>(
    api_url: impl Into<Cow<'a, str>>,
    api_key: impl Into<Cow<'a, str>>,
    request_json: impl Into<Cow<'a, str>>,
) -> Result<reqwest::Response, reqwest::Error> {
    let api_url = api_url.into();
    let api_key = api_key.into();
    let request_json = request_json.into();

    let client = reqwest::Client::new();
    let response = client
        .post(api_url.as_ref())
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .body(request_json.into_owned())
        .send()
        .await;
    response
}

pub async fn get<'a>(
    api_url: impl Into<Cow<'a, str>>,
    api_key: impl Into<Cow<'a, str>>,
) -> Result<reqwest::Response, reqwest::Error> {
    let api_url = api_url.into();
    let api_key = api_key.into();

    let client = reqwest::Client::new();
    let response = client
        .get(api_url.as_ref()) // 使用 GET 方法
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .send()
        .await;
    response
}
