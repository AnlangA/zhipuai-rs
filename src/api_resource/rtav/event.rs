use super::{
    error::SessionError,
    value::{ConversationItem, Error, Response, Session},
};

use base64::prelude::*;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{from_str, to_string, value::Serializer, Value};
use std::{
    collections::HashMap,
    io::{Error as IoError, ErrorKind},
    time::{SystemTime, UNIX_EPOCH},
};
use tokio_tungstenite::tungstenite::Message;

fn get_timestamp() -> Result<String, SessionError> {
    Ok(SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .as_millis()
        .to_string())
}

pub struct Event {
    data: HashMap<String, Value>,
}

impl Event {
    pub(crate) fn into_message(self) -> Result<Message, SessionError> {
        Ok(Message::Text(to_string(&self.data)?.into()))
    }

    /// 获取事件数据
    pub fn data(&self) -> Result<EventData, SessionError> {
        EventData::parse(&self.data)
    }

    pub(super) fn parse(src: Message) -> Result<Self, SessionError> {
        Ok(Self {
            data: from_str(src.to_text()?)?,
        })
    }

    fn from_data(data: &HashMap<String, Value>) -> Self {
        Self { data: data.clone() }
    }

    /// 获取事件ID
    /// 由客户端生成的id, 用于标识此事件
    pub fn id(&self) -> Option<&str> {
        match self.data.get("event_id") {
            None => None,
            Some(id) => id.as_str(),
        }
    }

    /// 获取客户端时间戳
    /// 调用端发起调用的时间戳，毫秒
    pub fn client_timestamp(&self) -> u64 {
        match self.data.get("client_timestamp") {
            None => Default::default(),
            Some(ts) => ts.as_u64().unwrap_or_default(),
        }
    }

    fn build_client_event(
        data: &mut HashMap<String, Value>,
        r#type: &str,
    ) -> Result<Self, SessionError> {
        let ts = get_timestamp()?;
        data.insert("event_id".to_string(), Value::String(format!("evt-{}", ts)));
        data.insert("type".to_string(), r#Value::String(r#type.to_string()));
        data.insert("client_timestamp".to_string(), Value::String(ts));
        Ok(Event::from_data(data))
    }

    /// 此事件用于上传音频至缓冲区。
    /// • 当使用Server VAD模式时，将由模型自动检测语音并决定何时提交。
    /// • 使用ClientVAD模式时，需要手动上传并提交音频。上传时可以自行决定音频长度，音频越短响应时间越快，最长可上传；
    ///
    /// # 参数
    /// * `audio`: 仅支持wav格式，默认采样率为16000；
    /// 如需自定义采样率，可在参数中标注，wav48表示48000hz采样率；
    /// 建议使用16000、24000、48000hz；
    pub fn new_input_audio_buffer_append(audio: &[u8]) -> Result<Self, SessionError> {
        let mut data = HashMap::new();
        let audio = BASE64_STANDARD.encode(audio);
        data.insert("audio".to_string(), Value::String(audio));
        Self::build_client_event(&mut data, "input_audio_buffer.append")
    }

    /// 提交已经上传的音频文件，此事件前必须进行 input_audio_buffer.append，且必须上传一个有效音频或视频文件，否则提交事件会报错。ServerVAD模式下不需要发送此事件，模型将自动上传并提交音频
    /// 调用 input_audio_buffer.commit 时，如果缓冲区内发过 video_frame，会一起打包提交调用模型推理。
    pub fn new_input_audio_buffer_commit() -> Result<Self, SessionError> {
        Self::build_client_event(&mut Default::default(), "input_audio_buffer.commit")
    }

    /// 此事件用于上传视频帧数频至缓冲区。当前版本下，chat_mode为video_passive视频帧均随音频同时发送，ServerVAD模式下会自动跟随音频上传，ClientVAD模式下需要按照指定的fps向服务端推送jpg图片。
    ///
    /// # 参数
    /// * `video_frame`: jpg格式图片，不符合 imageSize 的图片，会在服务端被重新 resize 到支持的尺寸
    pub fn new_input_audio_buffer_append_video_frame(
        video_frame: &[u8],
    ) -> Result<Self, SessionError> {
        let mut data = HashMap::new();
        let video_frame = BASE64_STANDARD.encode(video_frame);
        data.insert("video_frame".to_string(), Value::String(video_frame));
        Self::build_client_event(&mut data, "input_audio_buffer.append_video_frame")
    }

    /// 向对话上下文中添加一个item，包含消息、函数调用响应结果，可以讲此部分结果放入对话历史（session context/history）。如果传入文本为空或function.call.item为空时，会发送一个错误事件；
    ///
    /// # 参数
    /// * `item`: 对话项目。
    pub fn new_conversation_item_create(item: &ConversationItem) -> Result<Self, SessionError> {
        let mut data = HashMap::new();
        let v = item.serialize(Serializer)?;
        data.insert("item".to_string(), v);
        Self::build_client_event(&mut data, "conversation.item.create")
    }

    /// 此事件为创建服务器响应，同时也表示触发模型推理。ServerVAD模式服务器会自动创建响应，ClientVAD模式进行视频通话时，需以这个时间点的视频帧和音频传给模型；
    /// 当chat_mode为video时，提交事件之前必须通过input_audio_buffer.append_video_frame事件上传至少一张图片，否则无法创建模型回复，会返回错误事件video_model_query_error；
    pub fn new_response_create() -> Result<Self, SessionError> {
        Self::build_client_event(&mut Default::default(), "response.create")
    }

    /// 取消模型调用
    pub fn new_response_cancel() -> Result<Self, SessionError> {
        Self::build_client_event(&mut Default::default(), "response.cancel")
    }

    /// 通过此事件更新会话的默认配置，默认为音频通话，并且会使用参数的默认值，比如output_audio_format为pcm。
    /// • 特殊说明：当session.update切换chat_mode通话模式时，会有系统默认的对话历史处理策略：
    /// ◦ 从 video 到 audio，对话历史会被丢弃；
    /// ◦ 从 audio 到 video ，对话历史会保留；
    ///
    /// # 参数
    /// * `session`: 会话配置。
    pub fn new_session_update(session: &Session) -> Result<Self, SessionError> {
        let mut data = HashMap::new();
        let v = session.serialize(Serializer)?;
        data.insert("session".to_string(), v);
        Self::build_client_event(&mut data, "session.update")
    }
}

/// 事件数据（服务端）
#[derive(Debug)]
pub enum EventData {
    None,
    InputAudioBufferCommitted {
        item_id: String,
    },
    ConversationItemCreated(ConversationItem),
    Heartbeat,
    SessionCreated(Session),
    SessionUpdated(Session),
    ResponseCreated(Response),
    ResponseAudioTranscriptDelta {
        delta: String,
        content_index: u32,
        output_index: u32,
        response_id: String,
    },
    ResponseDone(Response),
    ConversationItemInputAudioTranscriptionCompleted {
        content_index: u32,
        transcript: String,
    },
    ResponseAudioDelta {
        content_index: u32,
        output_index: u32,
        delta: Vec<u8>,
    },
    Error(Error),
}

impl EventData {
    /// 判断是否是None
    pub fn is_none(&mut self) -> bool {
        if let Self::None = self {
            return true;
        }
        false
    }

    /// 判断是否是错误
    pub fn is_error(&self) -> bool {
        if let Self::Error(_) = self {
            return true;
        }
        false
    }

    /// 判断是否是心跳包
    pub fn is_heartbeat(&self) -> bool {
        if let Self::Heartbeat = self {
            return true;
        }
        false
    }

    /// 错误转换
    pub fn resolve(self) -> Result<Self, SessionError> {
        if let Self::Error(e) = self {
            return Err(e.into());
        }
        Ok(self)
    }

    fn parse(data: &HashMap<String, Value>) -> Result<Self, SessionError> {
        Ok(match data.get("type") {
            Some(Value::String(r)) => match r.as_str() {
                "input_audio_buffer.committed" => Self::InputAudioBufferCommitted {
                    item_id: Self::parse_value("item_id", data)?,
                },
                "conversation.item.created" => {
                    Self::ConversationItemCreated(Self::parse_value("item", data)?)
                }
                "heartbeat" => Self::Heartbeat,
                "session.created" => Self::SessionCreated(Self::parse_value("session", data)?),
                "session.updated" => Self::SessionUpdated(Self::parse_value("session", data)?),
                "error" => Self::Error(Self::parse_value("error", data)?),
                "response.created" => Self::ResponseCreated(Self::parse_value("response", data)?),
                "response.done" => Self::ResponseDone(Self::parse_value("response", data)?),
                "response.audio_transcript.delta" => Self::ResponseAudioTranscriptDelta {
                    delta: Self::parse_value("delta", data)?,
                    content_index: Self::parse_value("content_index", data)?,
                    output_index: Self::parse_value("output_index", data)?,
                    response_id: Self::parse_value("response_id", data)?,
                },
                "conversation.item.input_audio_transcription.completed" => {
                    Self::ConversationItemInputAudioTranscriptionCompleted {
                        content_index: Self::parse_value("content_index", data)?,
                        transcript: Self::parse_value("transcript", data)?,
                    }
                }
                "response.audio.delta" => {
                    let delta: String = Self::parse_value("delta", data)?;
                    Self::ResponseAudioDelta {
                        content_index: Self::parse_value("content_index", data)?,
                        output_index: Self::parse_value("output_index", data)?,
                        delta: BASE64_STANDARD.decode(delta)?,
                    }
                }
                _ => Self::None,
            },
            _ => Self::None,
        })
    }

    fn parse_value<T: DeserializeOwned>(
        key: &str,
        data: &HashMap<String, Value>,
    ) -> serde_json::Result<T> {
        match data.get(key) {
            Some(e) => serde_json::from_value(e.clone()),
            _ => Err(serde_json::Error::io(IoError::new(
                ErrorKind::InvalidData,
                key,
            ))),
        }
    }
}
