use std::io::{self, Write};
use zhipuai_rs::prelude::*;
use zhipuai_rs::chat_simple_message;
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let api_key = user_key()?;
    let messages = Messages::new().add_message(chat_simple_message!("system", "Please think deeply before your response.")).
        add_message(chat_simple_message!("user", "一个袋子中有5个红球和3个蓝球,随机抽取2个球,抽到至少1个红球的概率为:"));
    
    let (api_url, request_json) = BigModel::<Chat>::new(ChatModelName::GlmZ1Flash.into())
        .add_messages(messages)
        .stream_enable(true)
        .build();

    let response = post(api_url, api_key, request_json.to_json()).await?;

    let stream = response_context_stream(response);
    tokio::pin!(stream);

    while let Some(result) = stream.next().await {
        match result {
            Ok(data) => print!("{}", data),
            Err(e) => eprint!("Error: {}", e),
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
