use reqwest::Error;
use zhipuai_rs::api_resource::chat::{api::*, data::*, response::*};
use zhipuai_rs::http::*;
use zhipuai_rs::prelude::*;
use anyhow::Result;
use std::io::{self, Write};
#[tokio::main]
async fn main() -> Result<(), Error> {
    let api_key = user_key().unwrap();
    let (api_url, request_json) = ApiRequestBuilder::new(Model::GLM4Flash.into())
        .add_massage(Message::new(
            Role::System.into(),
            Some(Context::SimpleContexts(
                "你是全球最顶尖的童话专家".to_string(),
            )),
            None,
        ))
        .add_massage(Message::new(
            Role::User.into(),
            Some(Context::SimpleContexts(
                "讲个童话，至少2000个字".to_string(),
            )),
            None,
        ))
        .stream_enable(true)
        .max_tokens(4096)
        .build();

    let response = post(api_url, api_key, request_json.to_json()).await?;

    let stream = response_context_stream(response);
    tokio::pin!(stream);

    while let Some(result) = stream.next().await {
        match result {
            Ok(data) => print!("{}", data),
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    Ok(())
}
fn user_key() -> Result<String, > {
    let mut input = String::new();
    print!("输入你的key: ");
    io::stdout().flush()?; // 刷新标准输出，确保提示文字立即显示
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string()) // 去除输入内容的首尾空白字符
}