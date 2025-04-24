//! 实时音视频API示例

use rodio::{buffer::SamplesBuffer, OutputStream, Sink};
use std::io::{self, Write};
use tokio::{fs::File, io::AsyncReadExt};
use zhipuai_rs::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let api_key = user_key().unwrap();
    let (mut sink, mut stream) = start_realtime_session(&api_key).await?;
    sink.session_update(
        RealtimeSession::new()
            .with_input_audio_format("wav")
            .with_output_audio_format("pcm")
            .with_instructions("结合图中场景说说你的看法。")
            .with_turn_detection(
                RealtimeTurnDetection::new().with_vad_type(RealtimeVadType::ClientVad),
            )
            .with_beta_fields(
                RealtimeBetaFields::new()
                    .with_chat_mode(RealtimeChatMode::VideoPassive)
                    .with_tts_source("e2e")
                    .with_auto_search(false),
            ),
    )
    .await?;

    let mut audio = Default::default();
    File::open("examples/assets/audio.wav")
        .await?
        .read_to_end(&mut audio)
        .await?;
    let mut video_frame = Default::default();
    File::open("examples/assets/video_frame.jpg")
        .await?
        .read_to_end(&mut video_frame)
        .await?;

    sink.input_audio_buffer_append(&audio).await?;
    sink.input_audio_buffer_append_video_frame(&video_frame)
        .await?;
    sink.input_audio_buffer_commit().await?;
    sink.conversation_item_create(RealtimeConversationItem::new().with_text("以梦想为主题。"))
        .await?;
    // sink.conversation_item_create(RealtimeConversationItem::new().with_function_call_output("梦想 (dream)")).await?;
    sink.response_create().await?;
    // sink.response_cancel().await?;

    let (_stream, handle) = OutputStream::try_default().unwrap();
    let player = Sink::try_new(&handle).unwrap();

    while let Some(event) = stream.next_data().await? {
        let event = event.resolve()?;
        if event.is_heartbeat() {
            continue;
        }

        if let RealtimeEventData::ResponseAudioDelta { delta, .. } = event {
            println!("Audio data: {}", delta.len());
            player.append(samples::<1, 24000>(delta));
            continue;
        }
        println!("{:?}", event);
    }

    Ok(())
}

#[inline]
fn samples<const C: u16, const SR: u32>(data: Vec<u8>) -> SamplesBuffer<i16> {
    let mut out = Vec::with_capacity(data.len() / size_of::<i16>());
    let mut i = 0;
    while i < data.len() {
        out.push(i16::from_le_bytes([data[i], data[i + 1]]));
        i += size_of::<i16>();
    }

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
