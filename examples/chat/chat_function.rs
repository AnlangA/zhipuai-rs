use anyhow::Result;
use std::{
    collections::HashMap,
    io::{self, Write},
};
use zhipuai_rs::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let api_key = user_key().unwrap();
    let mut hash = HashMap::new();
    hash.insert(
        "起始地".to_string(),
        Property::new("string", "出发城市或车站"),
    );
    hash.insert(
        "目的地".to_string(),
        Property::new("string", "目的地城市或车站"),
    );
    hash.insert(
        "日期".to_string(),
        Property::new("string", "要查询的火车日期"),
    );
    hash.insert(
        "类型".to_string(),
        Property::new("string", "类型。火车、高铁、动车"),
    );
    let parameters = Parameters::new(hash);

    let (api_url, request_json) = BigModel::<Chat>::new(ChatModelName::GLM4Flash.into())
        .add_message(Message::new(
            Role::User.into(),
            Some(Context::SimpleContexts(
                "6月1日深圳到郴州的高铁".to_string(),
            )),
            None,
        ))
        .add_tools(Tool::new().function(Function::new(
            "query_train_info",
            "根据用户提供的信息进行处理",
            parameters,
        )))
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

fn user_key() -> Result<String> {
    let mut input = String::new();
    print!("输入你的key: ");
    io::stdout().flush()?; // 刷新标准输出，确保提示文字立即显示
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string()) // 去除输入内容的首尾空白字符
}
