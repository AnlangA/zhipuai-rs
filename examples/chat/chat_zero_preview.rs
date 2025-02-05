use std::io::{self, Write};
use zhipuai_rs::prelude::*;
use zhipuai_rs::chat_simple_message;
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let api_key = user_key().unwrap();
    let messages = Messages::new().add_message(chat_simple_message!("system", "Please think deeply before your response.")).
        add_message(chat_simple_message!("user", "一个袋子中有5个红球和3个蓝球,随机抽取2个球,抽到至少1个红球的概率为:"));
    
    let (api_url, request_json) = BigModel::<Chat>::new(ChatModelName::GLMZeroPreview.into())
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

fn user_key() -> anyhow::Result<String> {
    let mut input = String::new();
    print!("输入你的key: ");
    io::stdout().flush()?; // 刷新标准输出，确保提示文字立即显示
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string()) // 去除输入内容的首尾空白字符
}
