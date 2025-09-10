use base64::prelude::*;
use std::io::{self, Write};
use tokio::{fs::File, io::AsyncReadExt};
use zhipuai_rs::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let api_key = user_key()?;
    let mut data = Default::default();
    File::open("examples/assets/数学几何题.png")
        .await?
        .read_to_end(&mut data)
        .await?;
    let image_url = BASE64_STANDARD.encode(&data);
    let (api_url, request_json) = BigModel::<Chat>::new(ChatModelName::Glm4p1VThinkingFlashX.into())
        .add_message(Message::new(
            Role::User.into(),
            Some(
                Context::rich_contexts(RichContent::image_url(
                    // "https://sfile.chatglm.cn/testpath/8b01b0b4-51fd-5b51-90a1-3ad8fec8b00d_0.png",
                    &image_url,
                ))
                .rich_content(RichContent::text("理解题目，解决题目中的所有问题")),
            ),
            None,
        ))
        .build();

    let response = post(api_url, api_key, request_json.to_json()).await?;

    match chat_response_context(response).await {
        Ok(context) => {
            if let Some(choices) = context.get_choices() {
                for choice in choices {
                    println!("{}", choice.message());
                }
            }
        }
        Err(err) => {
            println!("{:?}", err);
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
