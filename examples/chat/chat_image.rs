use base64::prelude::*;
use std::io::{self, Write};
use tokio::{fs::File, io::AsyncReadExt};
use zhipuai_rs::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let api_key = user_key().unwrap();
    let mut data = Default::default();
    File::open("examples/assets/video_frame.jpg")
        .await
        .unwrap()
        .read_to_end(&mut data)
        .await
        .unwrap();
    let image_url = BASE64_STANDARD.encode(&data);
    let (api_url, request_json) = BigModel::<Chat>::new(ChatModelName::GLM4VFlash.into())
        .add_message(Message::new(
            Role::User.into(),
            Some(
                Context::rich_contexts(RichContent::image_url(
                    // "https://sfile.chatglm.cn/testpath/8b01b0b4-51fd-5b51-90a1-3ad8fec8b00d_0.png",
                    &image_url,
                ))
                .rich_content(RichContent::text("图里面有什么")),
            ),
            None,
        ))
        .build();

    let response = post(api_url, api_key, request_json.to_json()).await?;

    match response_context(response).await {
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

fn user_key() -> anyhow::Result<String> {
    let mut input = String::new();
    print!("输入你的key: ");
    io::stdout().flush()?; // 刷新标准输出，确保提示文字立即显示
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string()) // 去除输入内容的首尾空白字符
}
