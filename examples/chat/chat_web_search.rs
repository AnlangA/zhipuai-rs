use zhipuapi_rs::api_resource::chat::{api::*, data::*, response::*};
use zhipuapi_rs::prelude::*;
use anyhow::Result;
use std::io::{self, Write};
#[tokio::main]
async fn main() -> Result<(), Error> {
    let api_key = user_key().unwrap();
    let prompt = "

    # 以下是来自互联网的信息：
    {search_result}
    
    # 当前日期: 2024-XX-XX
    
    # 要求：
    根据最新发布的信息回答用户问题，当回答引用了参考信息时，必须在句末使用对应的[ref_序号]来标明参考信息来源。
    
    ";
    let (api_url, request_json) = ApiRequestBuilder::new(Model::GLM4Flash.into())
        .add_massage(Message::new(
            Role::System.into(),
            Some(Context::SimpleContexts(
                "你是全球最顶尖的电力电子专家".to_string(),
            )),
            None,
        ))
        .add_massage(Message::new(
            Role::User.into(),
            Some(Context::SimpleContexts(
                "我需要国外充电电源模块最新的进展".to_string(),
            )),
            None,
        ))
        .add_massage(Message::new(
            Role::Assistant.into(),
            Some(Context::SimpleContexts(
                "我用超级计算机查找一下，国内国外的最新消息我都会查找".to_string(),
            )),
            None,
        ))
        .add_massage(Message::new(
            Role::User.into(),
            Some(Context::SimpleContexts("介绍一下当前最新情况".to_string())),
            None,
        ))
        .add_tools(
            Tool::new().web_search(WebSearch::new().search_prompt(prompt).search_result(true)),
        )
        .stream_enable(false)
        .build();

    let response = post(api_url, api_key, request_json.to_json()).await?;

    let stream = response_context_stream(response);
    tokio::pin!(stream);

    while let Some(result) = stream.next().await {
        match result {
            Ok(data) => print!("{}", data),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
    Ok(())
}
fn user_key() -> Result<String, > {
    let mut input = String::new();
    print!("输入你的key: ");
    io::stdout().flush()?; // 刷新标准输出，确保提示文字立即显示
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string()) // 去除输入内容的首尾空白字符
}