use reqwest::Error;
use zhipuapi_rs::api_resource::chat::{api::*, data::*, response::*};
use zhipuapi_rs::http::*;
use zhipuapi_rs::values::{Role, Model};
use anyhow::Result;
use std::io::{self, Write};
#[tokio::main]
async fn main() -> Result<(), Error> {
    let api_key = user_key().unwrap();
    let (api_url, request_json) = ApiRequestBuilder::new(CODEGEEX_4)
        .add_code_context(Extra::new(Target::new(
            Some("main.rs".to_string()),
            Some("fn main() {\n    println!(\"Hell".to_string()),
            Some("".to_string()),
        )))
        .max_tokens(4096)
        .build();

    let response = post(api_url, api_key, request_json.to_json()).await?;

    match response_context(response).await {
        Ok(context) => {
            if let Some(choices) = context.get_choices() {
                for choice in choices {
                    println!("{}", choice.get_message());
                }
            }
        }
        Err(err) => {
            println!("{:?}", err);
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