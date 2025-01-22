use anyhow::Result;
use std::io::{self, Write};
use zhipuai_rs::prelude::*;
use zhipuai_rs::chat_simple_message;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let api_key = user_key().unwrap();

    let mut messages = Messages::new()
        .add_message(chat_simple_message!(
            Role::System,
            "你是中英语翻译专家，请准我为我提供文本翻译服务"
        ))
        .add_message(chat_simple_message!(Role::User, "准备为我提供文本翻译"))
        .add_message(chat_simple_message!(
            Role::Assistant,
            "然可以。请提供您希望翻译的文本，并告诉我您需要将其翻译成哪种语言"
        ))
        .add_message(chat_simple_message!(Role::User, "专家你好"));

    loop {
        let (api_url, request_json) = BigModel::<Chat>::new(ChatModelName::GLM4Flash.into())
            .add_messages(messages.clone())
            .build();
        println!("{:?}", request_json.to_json());
        let response = post(&api_url, &api_key, request_json.to_json()).await?;

        match response_context(response).await {
            Ok(context) => {
                if let Some(choices) = context.get_choices() {
                    for choice in choices {
                        println!("{}", choice.message());
                        let (role, message) = choice.message().simple_context().unwrap();
                        messages = messages.add_message(chat_simple_message!(role, message));
                    }
                }
            }
            Err(err) => {
                println!("{:?}", err);
            }
        }
        print!("输入: ");
        let mut input = String::new();
        io::stdout().flush().unwrap(); // 刷新标准输出，确保提示文字立即显示
        io::stdin().read_line(&mut input).unwrap();
        messages = messages.add_message(chat_simple_message!("user", input));
    }
}

// 用于从终端读取用户输入的函数
fn user_key() -> Result<String> {
    let mut input = String::new();
    print!("输入你的key: ");
    io::stdout().flush()?; // 刷新标准输出，确保提示文字立即显示
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string()) // 去除输入内容的首尾空白字符
}
