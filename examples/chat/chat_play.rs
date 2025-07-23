use std::io::{self, Write};
use zhipuai_rs::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let api_key = user_key()?;
    let tool = DrawingTool;
    let (api_url, request_json) = BigModel::<Chat>::new(ChatModelName::CharGlm4.into())
        .add_message(Message::new(
            Role::User.into(),
            Some(Context::SimpleContexts("你好呀，我今天很开心".to_string())),
            None,
        ))
        .add_tools(Tool::new().drawing_tool(tool))
        .stream_enable(true)
        .build();

    println!("{:?}", request_json.to_json());

    let response = post(api_url, api_key, request_json.to_json()).await?;

    let stream = response_context_stream(response);
    tokio::pin!(stream);

    while let Some(result) = stream.next().await {
        match result {
            Ok(data) => println!("{}", data),
            Err(e) => eprintln!("Error: {}", e),
        }
    }

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
