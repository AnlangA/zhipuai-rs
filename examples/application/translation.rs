use anyhow::{anyhow, Result};
use regex::RegexBuilder;
use std::io::{self, BufRead, Write};
use zhipuai_rs::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let api_key = user_key().unwrap();

    let stdin = io::stdin();
    let mut handle = stdin.lock();
    println!("输入 'exit' 退出程序。连续按两下'enter'开始翻译");
    loop {
        // 提示用户输入中文文本
        println!("请输入文本:");

        // 读取用户输入的多行文本
        let mut input_lines = Vec::new();
        let mut line = String::new();
        while let Ok(_) = handle.read_line(&mut line) {
            let trimmed_line = line.trim();
            if trimmed_line.eq_ignore_ascii_case("exit") {
                println!("退出程序...");
                return Ok(());
            }
            if trimmed_line.is_empty() {
                // 如果用户输入了只包含换行符的行，则认为输入结束
                break;
            }
            input_lines.push(trimmed_line.to_string());
            line.clear(); // 清空line以便下一次输入
        }

        if input_lines.is_empty() {
            println!("未输入任何文本，请重新输入。");
            continue;
        }

        let input = input_lines.join("\n");

        match translate_text(api_key.as_str(), &input).await {
            Ok(translated) => {
                let translation_result = extract_content(&translated).expect("无法提取内容");
                println!("翻译结果:\n{}\n", translation_result);
            }
            Err(err) => println!("翻译失败: {:?}", err),
        }
    }
}

async fn translate_text(api_key: &str, text: &str) -> Result<String> {
    let (api_url, request_json) = BigModel::<Chat>::new(ChatModelName::GLM4Flash.into())
        .add_message(Message::new(
            Role::System.into(),
            Some(Context::SimpleContexts(
                "你是专业的中译英翻译专家，将user发给你的中文翻译成英文，给你的英文翻译成中文"
                    .to_string(),
            )),
            None,
        ))
        .add_message(Message::new(
            Role::User.into(),
            Some(Context::SimpleContexts(
                "将我发给你的中文翻译成英文，给你的英文翻译成中文，只回复翻译内容".to_string(),
            )),
            None,
        ))
        .add_message(Message::new(
            Role::Assistant.into(),
            Some(Context::SimpleContexts(
                "接下来不管你发什么内容，我都只会进行翻译".to_string(),
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
    let context = response_context(response).await.expect("无法获取上下文");

    match context.get_choices() {
        Some(choices) if !choices.is_empty() => {
            let translations: Vec<String> = choices
                .iter()
                .map(|choice| format!("{}", choice.message()))
                .collect();
            Ok(translations.join("\n"))
        }
        _ => Err(anyhow!("未找到翻译")),
    }
}

fn extract_content(input: &str) -> Result<String> {
    let re = RegexBuilder::new(r"(?m)Content:\s*(.*)")
        .dot_matches_new_line(true)
        .build()
        .map_err(|e| anyhow!("无法创建正则表达式: {:?}", e))?;

    if let Some(caps) = re.captures(input) {
        if let Some(content) = caps.get(1) {
            return Ok(content.as_str().to_string());
        }
    }

    Err(anyhow!("在 'Content:' 后未找到内容"))
}

// 用于从终端读取用户输入的函数
fn user_key() -> Result<String> {
    let mut input = String::new();
    print!("输入你的key: ");
    io::stdout().flush()?; // 刷新标准输出，确保提示文字立即显示
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string()) // 去除输入内容的首尾空白字符
}
