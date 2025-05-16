use zhipuai_rs::api_resource::images;
use zhipuai_rs::prelude::*;
use std::io::{self, Write};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let api_key = user_key()?;
    
    let (api_url, request_json) = BigModel::<images::Images>::new(images::model::ImagesModelName::CogView3Flash.into())
        .prompt("一只凶狠的猫咪。")
        .build();
    
    let response = post(api_url, api_key, request_json.to_json()).await?;
    println!("come here post response");
    let api_response = images::response::images_response_context(response).await?;
    println!("come here post response decode");
    let url = api_response.urls();

    println!("图片链接: {:?}", url);

    Ok(())
}

//noinspection SpellCheckingInspection
fn user_key() -> anyhow::Result<String> {
    // 首先尝试从环境变量获取
    if let Ok(key) = std::env::var("ZHIPU_API_KEY") {
        println!("从环境变量:ZHIPU_API_KEY 获取到key");
        return Ok(key);
    }
    // 如果环境变量不存在，则要求用户输入
    let mut input = String::new();
    print!("输入你的key: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}