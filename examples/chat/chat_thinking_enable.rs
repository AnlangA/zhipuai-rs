use std::io::{self, Write};
use zhipuai_rs::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let api_key = user_key()?;
    
    // 使用支持 thinking 功能的 GLM-4.5 模型，并启用思维链推理
    let (api_url, request_json) = BigModel::<Chat>::new(ChatModelName::Glm4p5.into())
        .add_message(Message::new(
            Role::System.into(),
            Some(Context::SimpleContexts(
                "你是一个数学专家，请仔细分析问题并展示你的推理过程。".to_string(),
            )),
            None,
        ))
        .add_message(Message::new(
            Role::User.into(),
            Some(Context::SimpleContexts(
                "一个袋子中有5个红球和3个蓝球，随机抽取2个球，抽到至少1个红球的概率是多少？请详细说明计算过程。".to_string(),
            )),
            None,
        ))
        .thinking_enable()  // 启用思维链推理，模型会展示推理过程
        .max_tokens(4000)
        .build();

    println!("=== 启用思维链推理模式 ===");
    println!("请求JSON: {}", request_json.to_json());
    println!("\n=== 模型响应 ===");

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
            println!("错误: {:?}", err);
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
