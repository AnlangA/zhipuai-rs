use zhipuai_rs::api_resource::images;
use zhipuai_rs::prelude::*;
use std::io::{self, Write};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let api_key = user_key().unwrap();
    
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

fn user_key() -> anyhow::Result<String> {
    let mut input = String::new();
    print!("输入你的key: ");
    io::stdout().flush()?; // 刷新标准输出，确保提示文字立即显示
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string()) // 去除输入内容的首尾空白字符
}