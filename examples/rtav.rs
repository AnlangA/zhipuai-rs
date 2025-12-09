//! 质朴AI实时音视频API示例

use bytes::Buf;
use rodio::{OutputStreamBuilder, Sink, buffer::SamplesBuffer};
use std::io::{self, Write};
use tokio::fs::read;
use zhipuai_rs::prelude::*;

//noinspection SpellCheckingInspection
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let api_key = user_key()?;
    let output_stream = OutputStreamBuilder::from_default_device()?.open_stream()?;
    let player = Sink::connect_new(output_stream.mixer());

    // 演示server_vad模式下的音频和视频发送
    let (mut sink, mut stream) = start_realtime_session(&api_key).await?;
    sink.session_update(
        RealtimeSession::new()
            .with_input_audio_format("wav")
            .with_output_audio_format("pcm")
            .with_instructions("结合图中场景说说你的看法。")
            .with_turn_detection(RealtimeTurnDetection::new().with_server_vad())
            .with_voice("lovely_girl")
            .with_temperature(0.6)
            .with_max_response_output_tokens("80")
            .with_modalities(&["text", "audio"])
            .with_beta_fields(
                RealtimeBetaFields::new()
                    .with_chat_mode(RealtimeChatMode::VideoPassive)
                    .with_tts_source("e2e")
                    .with_auto_search(false),
            ),
    )
    .await?;
    sink.input_audio_buffer_append(&read("examples/assets/test.wav").await?)
        .await?;
    sink.input_audio_buffer_append_video_frame(&read("examples/assets/video_frame.jpg").await?)
        .await?;
    // sink.input_audio_buffer_clear().await?;
    // sink.conversation_item_delete("1234").await?;

    // `next_data()`不能获取到event_id和timestamp信息，如果需要可以使用`next()`
    while let Some(event) = stream.next_data().await? {
        let event = event.resolve()?;
        if event.is_heartbeat() {
            continue;
        }

        if let RealtimeEventData::ResponseAudioDelta { delta, .. } = event {
            println!("Audio data: {}", delta.len());
            player.append(samples::<1, 24000>(delta));
            continue;
        } else if let RealtimeEventData::ResponseAudioDone { .. } = event {
            break;
        } else if let RealtimeEventData::ResponseTextDone { text, .. } = event {
            println!("{}", text);
            break;
        }
        println!("{:?}", event);
    }

    // 演示tools调用
    let (mut sink, mut stream) = start_realtime_session(&api_key).await?;
    sink.session_update(
        RealtimeSession::new()
            .with_input_audio_format("wav")
            .with_output_audio_format("pcm")
            .with_turn_detection(RealtimeTurnDetection::new().with_client_vad())
            .with_modalities(&["text", "audio"])
            .with_tools(&[Function::new(
                "get_location",
                "获取当前的位置",
                Parameters::new(Default::default()),
            )])
            .with_beta_fields(
                RealtimeBetaFields::new()
                    // 截止到2025-05-15，仅仅支持音频模式调用自定义函数
                    .with_chat_mode(RealtimeChatMode::Audio)
                    .with_tts_source("e2e")
                    .with_auto_search(false),
            ),
    )
    .await?;
    sink.input_audio_buffer_append(&read("examples/assets/get_location.wav").await?)
        .await?;
    sink.input_audio_buffer_commit().await?;
    sink.response_create().await?;
    // sink.response_cancel().await?;
    let mut is_function_called = false;
    while let Some(event) = stream.next_data().await? {
        let event = event.resolve()?;
        if event.is_heartbeat() {
            continue;
        }

        if let RealtimeEventData::ResponseAudioDelta { delta, .. } = event {
            println!("Audio data: {}", delta.len());
            player.append(samples::<1, 24000>(delta));
            continue;
        } else if let RealtimeEventData::ResponseFunctionCallArgumentsDone { ref name, .. } = event
        {
            is_function_called = name == "get_location";
        } else if let RealtimeEventData::ResponseDone { .. } = event {
            if is_function_called {
                is_function_called = false;
                // 请在response-done到达之后再上传函数调用结果
                sink.conversation_item_create(
                    RealtimeConversationItem::new()
                        .with_role(Role::User)
                        .with_function_call_output("    杭州"),
                )
                .await?;
                sink.response_create().await?;
            } else {
                break;
            }
        }
        println!("{:?}", event);
    }

    Ok(player.sleep_until_end())
}

#[inline]
fn samples<const C: u16, const SR: u32>(data: Vec<u8>) -> SamplesBuffer {
    let out = data
        .chunks(size_of::<i16>())
        .map(|mut i| i.get_i16_le() as f32 / 32768f32)
        .collect::<Vec<_>>();
    SamplesBuffer::new(C, SR, out)
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
