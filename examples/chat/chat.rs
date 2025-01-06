use reqwest::Error;
use zhipuai_rs::api_resource::chat::{api::*, data::*, response::*};
use zhipuai_rs::http::*;
use zhipuai_rs::values::{Role, Model};
use anyhow::Result;
use std::io::{self ,Write};
#[tokio::main]
async fn main() -> Result<(), Error> {
    let api_key = user_key().unwrap();
    let (api_url, request_json) = ApiRequestBuilder::new(Model::GLM4Flash.into())
        .add_massage(Message::new(
            Role::System.into(),
            Some(Context::SimpleContexts(
                "你是专业的中译英翻译专家，将user发给你的中文翻译成英文".to_string(),
            )),
            None,
        ))
        .add_massage(Message::new(
            Role::User.into(),
            Some(Context::SimpleContexts(
                "将我发给你的中文翻译成英文，只回复翻译内容".to_string(),
            )),
            None,
        ))
        .add_massage(Message::new(
            Role::Assistant.into(),
            Some(Context::SimpleContexts(
                "好的，把要翻译的中文给我吧".to_string(),
            )),
            None,
        ))
        .add_massage(Message::new(
            Role::User.into(),
            Some(Context::SimpleContexts("今天天气真好".to_string())),
            None,
        ))
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

// 用于从终端读取用户输入的函数
fn user_key() -> Result<String> {
    let mut input = String::new();
    print!("输入你的key: ");
    io::stdout().flush()?; // 刷新标准输出，确保提示文字立即显示
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string()) // 去除输入内容的首尾空白字符
}