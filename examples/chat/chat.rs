use std::io::{self, Write};
use zhipuai_rs::{chat_simple_message, prelude::*};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let api_key = user_key()?;

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
        let (api_url, request_json) = BigModel::<Chat>::new(ChatModelName::Glm4Flash250414.into())
            .add_messages(messages.clone())
            .build();
        println!("{:?}", request_json.to_json());
        let response = post(&api_url, &api_key, request_json.to_json()).await?;

        match chat_response_context(response).await {
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
        io::stdout().flush()?; // 刷新标准输出，确保提示文字立即显示
        io::stdin().read_line(&mut input)?;
        messages = messages.add_message(chat_simple_message!("user", input));
    }
}

//noinspection SpellCheckingInspection
// 用于从终端读取用户输入的函数
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
