//! 实时音视频API示例

use rodio::{OutputStream, Sink, Source};
use std::{
    io::{self, Write},
    time::Duration,
};
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
            player.append(Pcm::new(delta, 24000));
            continue;
        }
        println!("{:?}", event);
    }

    Ok(())
}

struct Pcm {
    data: Vec<u8>,
    index: usize,
    sample_rate: u32,
}

impl Pcm {
    fn new(data: Vec<u8>, sample_rate: u32) -> Self {
        Self {
            data,
            index: 0,
            sample_rate,
        }
    }
}

impl Iterator for Pcm {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.data.len() {
            return None;
        }

        let data = [self.data[self.index], self.data[self.index + 1]];
        self.index += 2;
        Some(i16::from_le_bytes(data) as f32 / i16::MAX as f32)
    }
}

impl Source for Pcm {
    fn current_frame_len(&self) -> Option<usize> {
        Some(self.data.iter().len() / 2)
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn total_duration(&self) -> Option<Duration> {
        self.current_frame_len()
            .map(|i| Duration::from_secs(i as u64 / self.sample_rate as u64))
    }
}

// 用于从终端读取用户输入的函数
fn user_key() -> anyhow::Result<String> {
    let mut input = String::new();
    print!("输入你的key: ");
    io::stdout().flush()?; // 刷新标准输出，确保提示文字立即显示
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string()) // 去除输入内容的首尾空白字符
}
