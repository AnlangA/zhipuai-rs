use anyhow::{anyhow, Result};
use regex::RegexBuilder;
use std::io::{self, Write};
use zhipuai_rs::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let api_key = user_key().unwrap();

    // 读取用户输入的文本
    let text_to_translate = read_user_input()?;

    match translate_text(api_key.as_str(), &text_to_translate).await {
        Ok(translated) => {
            let translation_result = extract_content(&translated).expect("Cannot extract content");
            println!("{}", translation_result);
        }
        Err(err) => println!("{:?}", err),
    }
    Ok(())
}

// 用于从终端读取用户输入的函数
fn user_key() -> Result<String> {
    let mut input = String::new();
    print!("输入你的key: ");
    io::stdout().flush()?; // 刷新标准输出，确保提示文字立即显示
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string()) // 去除输入内容的首尾空白字符
}

// 用于从终端读取用户输入的函数
fn read_user_input() -> Result<String> {
    let mut input = String::new();
    print!("输入要润色的文本: ");
    io::stdout().flush()?; // 刷新标准输出，确保提示文字立即显示
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string()) // 去除输入内容的首尾空白字符
}

async fn translate_text(api_key: &str, text: &str) -> Result<String> {
    let (api_url, request_json) = BigModel::<Chat>::new(ChatModelName::GLM4Flash.into())
        .add_message(Message::new(
            Role::System.into(),
            Some(Context::SimpleContexts(
                "你是专业的英文润色高手，你是澳大利亚法律学专业的研究生,但母语是中文，具有相当专业的英语能力。".to_string(),
            )),
            None,
        ))
        .add_message(Message::new(
            Role::User.into(),
            Some(Context::SimpleContexts(
                "我将发送一段不怎么专业的英文，你需要将这些内容使用你的专业技能进行润色。".to_string(),
            )),
            None,
        ))
        .add_message(Message::new(
            Role::Assistant.into(),
            Some(Context::SimpleContexts(
                "好的，我会基于我的身份-澳大利亚法律学专业的研究生，使用我的专业技能进行润色。".to_string(),
            )),
            None,
        ))
        .add_message(Message::new(
            Role::User.into(),
            Some(Context::SimpleContexts(text.to_string())),
            None,
        ))
        .max_tokens(4096)
        .build();

    let response = post(api_url, api_key, request_json.to_json()).await?;
    let context = chat_response_context(response)
        .await
        .expect("Cannot get context");

    match context.get_choices() {
        Some(choices) if !choices.is_empty() => {
            let translations: Vec<String> = choices
                .iter()
                .map(|choice| format!("{}", choice.message())) // 使用 format! 转换为 String
                .collect();
            Ok(translations.join("\n")) // 将翻译结果用换行符连接
        }
        _ => Err(anyhow!("No translation found")),
    }
}

fn extract_content(input: &str) -> Result<String> {
    // 定义一个正则表达式，匹配 "Content:" 后面的所有字符
    let re = RegexBuilder::new(r"(?m)Content:\s*(.*)")
        .dot_matches_new_line(true)
        .build()
        .map_err(|e| anyhow!("Failed to create regex: {:?}", e))?;

    // 使用正则表达式进行匹配，并提取第一个捕获组
    if let Some(caps) = re.captures(input) {
        if let Some(content) = caps.get(1) {
            return Ok(content.as_str().to_string());
        }
    }

    Err(anyhow!("No content found after 'Content:'"))
}
