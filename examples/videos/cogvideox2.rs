use zhipuai_rs::api_resource::videos;
use zhipuai_rs::prelude::*;
use std::io::{self, Write};
use tokio::time;
use tokio::time::Duration;
use futures::stream::{self, StreamExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 获取 API 密钥
    let api_key = user_key()?;
    
    // 构建视频生成请求
    let (api_url, request_json) = BigModel::<videos::Videos>::new(videos::model::VideosModelName::Cogvideox2.into())
        .prompt("玩具大战")
        .response_id("")
        .build();
    

    // 发送请求并获取响应
    let response = post(api_url.clone(), api_key.clone(), request_json.to_json()).await?;

    // 解析响应并获取任务 ID
    let api_response = videos::response::videos_response_context(response).await?;

    // 等待任务完成
    let task_id = api_response.id();
    
    let mut interval = time::interval(Duration::from_secs(5));

    // 使用流处理轮询任务状态
    let stream = stream::unfold((), move |_| {
        let task_id = task_id.clone();
        let api_key = api_key.clone();
        async move {
            // 构建异步请求
            let (api_url, _) = BigModel::<videos::Videos>::new(videos::model::VideosModelName::Cogvideox2.into())
                .prompt("")
                .response_id(&task_id)
                .build_response();


            // 发送异步请求并获取最终响应
            let final_response = get(api_url.clone(), api_key.clone()).await.ok()?;

            // 解析最终响应
            let api_response = videos::response::videos_asyn_response_context(final_response).await.ok()?;

            // 返回任务状态
            Some((api_response, ()))
        }
    });

    // 将流装箱
    let mut stream = Box::pin(stream);

    // 处理流中的任务状态
    while let Some(api_response) = stream.next().await {
        match api_response.task_status().as_str() {
            "SUCCESS" => {
                // 提取视频链接并打印
                let videos = api_response.video_result();
                if videos.is_empty() {
                    println!("未找到视频链接");
                } else {
                    for video in videos {
                        println!("视频链接: {}", video.url());
                    }
                }
                break; // 任务完成，退出循环
            }
            "FAIL" => {
                println!("任务失败: {}", api_response.task_status());
                break; // 任务失败，退出循环
            }
            _ => {
                println!("任务状态: {}, 继续等待...", api_response.task_status());
                interval.tick().await; // 等待 5 秒后继续轮询
            }
        }
    }
    Ok(())
}

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
