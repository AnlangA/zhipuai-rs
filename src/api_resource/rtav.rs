//! https://www.bigmodel.cn/dev/api/rtav/GLM-Realtime
//! GLM-Realtime API 能够提供实时的视频通话功能，具有跨文本、音频和视频进行实时推理的能力，AI可以进行流畅的通话，人可以实时打断AI。
//! 除了实时音频交互外，Realtime还可通过手机或AIPC的摄像头与人互动，通过共享电脑屏幕阅读页面信息，通过视频流理解对话当前的环境。

mod event;
mod value;

use crate::error::ZhipuApiError;
use futures::{
    stream::{SplitSink, SplitStream},
    Sink, SinkExt, Stream, StreamExt,
};
use pin_project::pin_project;
use std::{
    pin::Pin,
    task::{Context, Poll},
};
use tokio::net::TcpStream;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{client::IntoClientRequest, http::StatusCode, Message},
    MaybeTlsStream, WebSocketStream,
};
pub use {event::*, value::*};

#[pin_project]
pub struct SessionSink {
    inner: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
}

impl SessionSink {
    /// 通过此事件更新会话的默认配置，默认为音频通话，并且会使用参数的默认值，比如output_audio_format为pcm。
    /// • 特殊说明：当session.update切换chat_mode通话模式时，会有系统默认的对话历史处理策略：
    /// ◦ 从 video 到 audio，对话历史会被丢弃；
    /// ◦ 从 audio 到 video ，对话历史会保留；
    ///
    /// # 参数
    /// * `session`: 会话配置。
    pub async fn session_update(&mut self, session: &Session) -> Result<(), ZhipuApiError> {
        self.send(Event::new_session_update(session)?).await
    }

    /// 此事件用于上传音频至缓冲区。
    /// • 当使用Server VAD模式时，将由模型自动检测语音并决定何时提交。
    /// • 使用ClientVAD模式时，需要手动上传并提交音频。上传时可以自行决定音频长度，音频越短响应时间越快，最长可上传；
    ///
    /// # 参数
    /// * `audio`: 仅支持wav格式，默认采样率为16000；
    /// 如需自定义采样率，可在参数中标注，wav48表示48000hz采样率；
    /// 建议使用16000、24000、48000hz；
    pub async fn input_audio_buffer_append(&mut self, audio: &[u8]) -> Result<(), ZhipuApiError> {
        self.send(Event::new_input_audio_buffer_append(audio)?)
            .await
    }

    /// 此事件用于上传视频帧数频至缓冲区。当前版本下，chat_mode为video_passive视频帧均随音频同时发送，ServerVAD模式下会自动跟随音频上传，ClientVAD模式下需要按照指定的fps向服务端推送jpg图片。
    ///
    /// # 参数
    /// * `video_frame`: jpg格式图片，不符合 imageSize 的图片，会在服务端被重新 resize 到支持的尺寸
    pub async fn input_audio_buffer_append_video_frame(
        &mut self,
        video_frame: &[u8],
    ) -> Result<(), ZhipuApiError> {
        self.send(Event::new_input_audio_buffer_append_video_frame(
            video_frame,
        )?)
        .await
    }

    /// 提交已经上传的音频文件，此事件前必须进行 input_audio_buffer.append，且必须上传一个有效音频或视频文件，否则提交事件会报错。ServerVAD模式下不需要发送此事件，模型将自动上传并提交音频
    /// 调用 input_audio_buffer.commit 时，如果缓冲区内发过 video_frame，会一起打包提交调用模型推理。
    pub async fn input_audio_buffer_commit(&mut self) -> Result<(), ZhipuApiError> {
        self.send(Event::new_input_audio_buffer_commit()?).await
    }

    /// 向对话上下文中添加一个item，包含消息、函数调用响应结果，可以讲此部分结果放入对话历史（session context/history）。如果传入文本为空或function.call.item为空时，会发送一个错误事件；
    ///
    /// # 参数
    /// * `item`: 对话项目。
    pub async fn conversation_item_create(
        &mut self,
        item: &ConversationItem,
    ) -> Result<(), ZhipuApiError> {
        self.send(Event::new_conversation_item_create(item)?).await
    }

    /// 此事件为创建服务器响应，同时也表示触发模型推理。ServerVAD模式服务器会自动创建响应，ClientVAD模式进行视频通话时，需以这个时间点的视频帧和音频传给模型；
    /// 当chat_mode为video时，提交事件之前必须通过input_audio_buffer.append_video_frame事件上传至少一张图片，否则无法创建模型回复，会返回错误事件video_model_query_error；
    pub async fn response_create(&mut self) -> Result<(), ZhipuApiError> {
        self.send(Event::new_response_create()?).await
    }

    /// 取消模型调用
    pub async fn response_cancel(&mut self) -> Result<(), ZhipuApiError> {
        self.send(Event::new_response_cancel()?).await
    }
}

impl Sink<Event> for SessionSink {
    type Error = ZhipuApiError;

    fn poll_ready(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Pin::new(&mut self.project().inner)
            .poll_ready(cx)
            .map_err(|e| e.into())
    }

    fn start_send(self: Pin<&mut Self>, item: Event) -> Result<(), Self::Error> {
        Pin::new(&mut self.project().inner)
            .start_send(item.into_message()?)
            .map_err(|e| e.into())
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Pin::new(&mut self.project().inner)
            .poll_flush(cx)
            .map_err(|e| e.into())
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Pin::new(&mut self.project().inner)
            .poll_close(cx)
            .map_err(|e| e.into())
    }
}

#[pin_project]
pub struct SessionStream {
    inner: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
}

impl SessionStream {
    /// 获取下一个事件数据
    pub async fn next_data(&mut self) -> Result<Option<EventData>, ZhipuApiError> {
        self.next()
            .await
            .map_or_else(|| Ok(None), |e| e.map_or_else(Err, |e| e.data().map(Some)))
    }
}

impl Stream for SessionStream {
    type Item = Result<Event, ZhipuApiError>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Pin::new(&mut self.project().inner)
            .poll_next(cx)
            .map(|o| match o {
                None => None,
                Some(o) => o.map_or_else(
                    |e| Some(Err(e.into())),
                    |m| {
                        if m.is_close() {
                            None
                        } else {
                            Some(Event::parse(m))
                        }
                    },
                ),
            })
    }
}

const URL: &str = "wss://open.bigmodel.cn/api/paas/v4/realtime";

pub async fn start_realtime_session(
    api_key: &str,
) -> Result<(SessionSink, SessionStream), ZhipuApiError> {
    let mut req = URL.into_client_request()?;
    req.headers_mut()
        .insert("Authorization", format!("Bearer {}", api_key).parse()?);

    // 连接到WebSocket服务器
    let (stream, response) = connect_async(req).await?;

    // 这通常是HTTP升级响应
    let status = response.status();
    if StatusCode::SWITCHING_PROTOCOLS != status && !status.is_success() {
        return Err(ZhipuApiError::StatusCode(status.to_string()));
    }

    let (send, recv) = stream.split();
    Ok((SessionSink { inner: send }, SessionStream { inner: recv }))
}
