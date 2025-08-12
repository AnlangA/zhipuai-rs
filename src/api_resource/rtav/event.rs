use super::value::{
    ContentPart, Conversation, ConversationItem, Error, RateLimit, Response, Session,
};

use crate::error::ZhipuApiError;
use base64::prelude::*;
use serde::{Serialize, de::DeserializeOwned};
use serde_json::{Error as JsonError, Value, from_str, from_value, to_string, value::Serializer};
use std::{
    collections::HashMap,
    io::{Error as IoError, ErrorKind},
    time::{SystemTime, UNIX_EPOCH},
};
use tokio_tungstenite::tungstenite::Message;

fn get_timestamp() -> Result<String, ZhipuApiError> {
    Ok(SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .as_millis()
        .to_string())
}

pub struct Event {
    data: HashMap<String, Value>,
}

impl Event {
    pub(crate) fn into_message(self) -> Result<Message, ZhipuApiError> {
        Ok(Message::Text(to_string(&self.data)?.into()))
    }

    /// 获取事件数据
    pub fn data(&self) -> Result<EventData, ZhipuApiError> {
        EventData::parse(&self.data)
    }

    pub(super) fn parse(src: Message) -> Result<Self, ZhipuApiError> {
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
    ) -> Result<Self, ZhipuApiError> {
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
    pub fn new_input_audio_buffer_append(audio: &[u8]) -> Result<Self, ZhipuApiError> {
        let mut data = HashMap::new();
        let audio = BASE64_STANDARD.encode(audio);
        data.insert("audio".to_string(), Value::String(audio));
        Self::build_client_event(&mut data, "input_audio_buffer.append")
    }

    /// 提交已经上传的音频文件，此事件前必须进行 input_audio_buffer.append，且必须上传一个有效音频或视频文件，否则提交事件会报错。ServerVAD模式下不需要发送此事件，模型将自动上传并提交音频
    /// 调用 input_audio_buffer.commit 时，如果缓冲区内发过 video_frame，会一起打包提交调用模型推理。
    pub fn new_input_audio_buffer_commit() -> Result<Self, ZhipuApiError> {
        Self::build_client_event(&mut Default::default(), "input_audio_buffer.commit")
    }

    /// 客户端发送 input_audio_buffer.clear 事件用于清除缓冲区中的音频数据。
    /// 服务端使用 input_audio_buffer.cleared 事件进行响应。
    pub fn new_input_audio_buffer_clear() -> Result<Self, ZhipuApiError> {
        Self::build_client_event(&mut Default::default(), "input_audio_buffer.clear")
    }

    /// 此事件用于上传视频帧数频至缓冲区。当前版本下，chat_mode为video_passive视频帧均随音频同时发送，ServerVAD模式下会自动跟随音频上传，ClientVAD模式下需要按照指定的fps向服务端推送jpg图片。
    ///
    /// # 参数
    /// * `video_frame`: jpg格式图片，不符合 imageSize 的图片，会在服务端被重新 resize 到支持的尺寸
    pub fn new_input_audio_buffer_append_video_frame(
        video_frame: &[u8],
    ) -> Result<Self, ZhipuApiError> {
        let mut data = HashMap::new();
        let video_frame = BASE64_STANDARD.encode(video_frame);
        data.insert("video_frame".to_string(), Value::String(video_frame));
        Self::build_client_event(&mut data, "input_audio_buffer.append_video_frame")
    }

    /// 向对话上下文中添加一个item，包含消息、函数调用响应结果，可以讲此部分结果放入对话历史（session context/history）。如果传入文本为空或function.call.item为空时，会发送一个错误事件；
    ///
    /// # 参数
    /// * `item`: 对话项目。
    pub fn new_conversation_item_create(item: &ConversationItem) -> Result<Self, ZhipuApiError> {
        let mut data = HashMap::new();
        let v = item.serialize(Serializer)?;
        data.insert("item".to_string(), v);
        Self::build_client_event(&mut data, "conversation.item.create")
    }

    /// 删除会话历史中的一轮次会话 conversation.item.delete
    ///
    /// # 参数
    /// * `item_id` 对话项目ID
    pub fn new_conversation_item_delete(item_id: &str) -> Result<Self, ZhipuApiError> {
        let mut data = HashMap::new();
        let v = item_id.serialize(Serializer)?;
        data.insert("item_id".to_string(), v);
        Self::build_client_event(&mut data, "conversation.item.delete")
    }

    /// 此事件为创建服务器响应，同时也表示触发模型推理。ServerVAD模式服务器会自动创建响应，ClientVAD模式进行视频通话时，需以这个时间点的视频帧和音频传给模型；
    /// 当chat_mode为video时，提交事件之前必须通过input_audio_buffer.append_video_frame事件上传至少一张图片，否则无法创建模型回复，会返回错误事件video_model_query_error；
    pub fn new_response_create() -> Result<Self, ZhipuApiError> {
        Self::build_client_event(&mut Default::default(), "response.create")
    }

    /// 取消模型调用
    pub fn new_response_cancel() -> Result<Self, ZhipuApiError> {
        Self::build_client_event(&mut Default::default(), "response.cancel")
    }

    /// 通过此事件更新会话的默认配置，默认为音频通话，并且会使用参数的默认值，比如output_audio_format为pcm。
    /// • 特殊说明：当session.update切换chat_mode通话模式时，会有系统默认的对话历史处理策略：
    /// ◦ 从 video 到 audio，对话历史会被丢弃；
    /// ◦ 从 audio 到 video ，对话历史会保留；
    ///
    /// # 参数
    /// * `session`: 会话配置。
    pub fn new_session_update(session: &Session) -> Result<Self, ZhipuApiError> {
        let mut data = HashMap::new();
        let v = session.serialize(Serializer)?;
        data.insert("session".to_string(), v);
        Self::build_client_event(&mut data, "session.update")
    }
}

/// 事件数据（服务端）
#[derive(Debug)]
pub enum EventData {
    /// 未知的事件
    None,
    /// 在创建会话后会立即返回服务器 conversation.created 事件。 每个会话创建一个对话。
    ConversationCreated(Conversation),
    /// 创建对话项时，将返回服务器 conversation.item.created 事件。
    ConversationItemCreated(ConversationItem),
    /// 写入音频缓冲区的语音转文本的结果。语音转文本与响应创建异步运行，该事件可能发生在响应事件之前或者之后；
    ConversationItemInputAudioTranscriptionCompleted {
        content_index: u32,
        /// 包含音频的用户消息项的 ID。
        item_id: String,
        transcript: String,
    },
    /// 配置了输入音频听录并且用户消息的听录请求失败时，系统会返回服务器 conversation.item.input_audio_transcription.failed 事件。 此事件是与其他 error 事分开的，以便客户端能够识别相关项。
    ConversationItemInputAudioTranscriptionFailed {
        /// 包含音频的内容部分的索引
        content_index: u32,
        /// 包含音频的用户消息项的ID
        item_id: String,
        error: Error,
    },
    /// 客户端使用 conversation.item.delete 事件删除对话项目时，系统会返回服务器 conversation.item.deleted。
    ConversationItemDeleted { item_id: String },
    /// 发生错误时，系统会返回服务器 error 事件（可能是客户端问题，也可能是服务器问题，具体可查看错误码文档）。 大多数错误都是可恢复的，并且会话将保持打开状态。
    Error(Error),
    /// 当会话创建/更新是时会返回，后续每30s返回一次，Heartbeat表示对话当前是活跃的链接状态；
    Heartbeat,
    /// 输入音频缓冲区由客户端提交或在服务器 VAD 模式下自动提交时，系统会返回服务器 input_audio_buffer.committed 事件。
    InputAudioBufferCommitted { item_id: String },
    /// 客户端使用 input_audio_buffer.clear 事件清除输入音频缓冲区时，系统会返回服务器 input_audio_buffer.cleared事件。
    InputAudioBufferCleared,
    /// 在音频缓冲区中检测到语音时，系统会以 server_vad 模式返回服务器 input_audio_buffer.speech_started 事件。
    InputAudioBufferSpeechStarted {
        audio_start_ms: u32,
        item_id: String,
    },
    /// server_vad 模式下服务器在音频缓冲区中检测到语音结束时，系统会返回服务器 input_audio_buffer.speech_stopped 事件。
    /// 服务器还发送一个 conversation.item.created 事件，其中包含从音频缓冲区创建的用户消息项。
    InputAudioBufferSpeechStopped { audio_end_ms: u32, item_id: String },
    /// 速率限制发生变化时通知
    RateLimitsUpdated(Vec<RateLimit>),
    /// 创建新响应时，系统会返回服务器 response.done 事件。
    ResponseCreated(Response),
    /// 对客户端事件response.cancel的响应，如果有正在进行中的response的话。
    ResponseCancelled(Response),
    /// 更新模型生成的音频时，系统将返回服务器response.audio.delta 事件。delta 是一个 mp3或pcm 格式base64 编码的音频块，根据output audio format决定。
    ResponseAudioDelta {
        delta: Vec<u8>,
        content_index: u32,
        output_index: u32,
        response_id: String,
    },
    /// 模型生成完音频后，系统将返回服务器 response.audio.done 事件。
    /// 当响应中断、不完整或取消时，系统也会返回此事件。
    ResponseAudioDone {
        content_index: u32,
        output_index: u32,
        item_id: String,
        response_id: String,
    },
    /// 更新模型生成的音频输出语音转文本时，系统会返回服务器response.audio_transcript.delta 事件。
    ResponseAudioTranscriptDelta {
        delta: String,
        content_index: u32,
        output_index: u32,
        item_id: String,
        response_id: String,
    },
    /// 模型生成的音频输出听录完成流式处理时，系统会返回服务器 response.audio_transcript.done 事件。
    /// 当响应中断、不完整或取消时，系统也会返回此事件。
    ResponseAudioTranscriptDone {
        transcript: String,
        content_index: u32,
        output_index: u32,
        item_id: String,
        response_id: String,
    },
    /// 当响应完成流式处理时，系统会返回服务器 response.done 事件。 无论最终状态如何，始终发出此事件。
    /// 消耗的tokens，会在在response.done事件中返回；包含完整的input、output token信息；
    ResponseDone(Response),
    /// 模型生成的函数调用时，系统会返回服务器 response.function_call_arguments.done 事件。
    /// 当发给模型的query需要调用多次function call时，可能会返回多个调用，比如提问“帮我搜一下北京、上海的天气”，模型会返回2次function call的结果，系统也会返回两次 response.function_call_arguments.done 事件。
    /// 当前仅支持响应成功时返回此事件，中断、不完整或取消时正在支持中。
    ResponseFunctionCallArgumentsDone {
        /// 函数的名称
        name: String,
        /// 函数调用参数, json字符串,需自行解析
        arguments: String,
        output_index: u32,
        response_id: String,
    },
    /// video模型内置了搜索的工具，当识别到用户的提问需要通过搜索获取外部数据时，会返回此事件。服务内部会自动调用搜索接口获取数据，获取搜索结果后会再次调用模型，获取到模型回复后继续流式返回数据。
    /// 此事件在response.created事件之后，在response.audio_transcript.delta之前，如搜索结果报错，会返回错误事件
    /// video_model_query_error。
    /// 当前视频链路我们还未支持开关搜索工具，将在后续的版本中支持。
    ResponseFunctionCallSimpleBrowser { name: String, session: Session },
    /// 在响应生成过程中创建新项时，系统会返回服务器 response.output_item.added 事件。
    ResponseOutputItemAdded {
        item: ConversationItem,
        output_index: u32,
        response_id: String,
    },
    /// 当项完成流式处理时，系统会返回服务器 response.output_item.done 事件。
    /// 当响应中断、不完整或取消时，系统也会返回此事件。
    ResponseOutputItemDone {
        item: ConversationItem,
        output_index: u32,
        response_id: String,
    },
    /// 在响应生成期间将新的内容部分添加到助手消息项时，系统会返回服务器 response.content_part.added 事件。
    ResponseContentPartAdded {
        part: ContentPart,
        content_index: u32,
        output_index: u32,
        item_id: String,
        response_id: String,
    },
    /// 当内容部分在助手消息项中完成流式处理时，系统会返回服务器 response.content_part.done 事件。
    ResponseContentPartDone {
        part: ContentPart,
        content_index: u32,
        output_index: u32,
        item_id: String,
        response_id: String,
    },
    /// 更新模型生成的文本时，系统会返回服务器response.text.delta 事件。 文本对应于助手消息项的 text 内容部分。
    ResponseTextDelta {
        delta: String,
        content_index: u32,
        output_index: u32,
        item_id: String,
        response_id: String,
    },
    /// 当模型生成的文本完成流式处理时，系统会返回服务器 response.text.done 事件。 文本对应于助手消息项的 text 内容部分。
    /// 当响应中断、不完整或取消时，系统也会返回此事件。
    ResponseTextDone {
        text: String,
        content_index: u32,
        output_index: u32,
        item_id: String,
        response_id: String,
    },
    /// 在创建会话后会立即返回服务器session.created 事件；
    SessionCreated(Session),
    /// 在更新会话后会立即返回服务器session.updated 事件；
    SessionUpdated(Session),
}

impl EventData {
    /// 判断是否是None
    pub fn is_none(&self) -> bool {
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
    pub fn resolve(self) -> Result<Self, ZhipuApiError> {
        if let Self::Error(e) = self {
            return Err(e.into());
        }
        Ok(self)
    }

    fn parse(data: &HashMap<String, Value>) -> Result<Self, ZhipuApiError> {
        Ok(match data.get("type") {
            Some(Value::String(r)) => match r.as_str() {
                "conversation.created" => {
                    Self::ConversationCreated(Self::parse_value("conversation", data)?)
                }
                "conversation.item.created" => {
                    Self::ConversationItemCreated(Self::parse_value("item", data)?)
                }
                "conversation.item.input_audio_transcription.completed" => {
                    Self::ConversationItemInputAudioTranscriptionCompleted {
                        content_index: Self::parse_value("content_index", data)?,
                        item_id: Self::parse_value("item_id", data)?,
                        transcript: Self::parse_value("transcript", data)?,
                    }
                }
                "conversation.item.input_audio_transcription.failed" => {
                    Self::ConversationItemInputAudioTranscriptionFailed {
                        content_index: Self::parse_value("content_index", data)?,
                        item_id: Self::parse_value("item_id", data)?,
                        error: Self::parse_value("error", data)?,
                    }
                }
                "conversation.item.deleted" => Self::ConversationItemDeleted {
                    item_id: Self::parse_value("item_id", data)?,
                },
                "error" => Self::Error(Self::parse_value("error", data)?),
                "heartbeat" => Self::Heartbeat,
                "input_audio_buffer.committed" => Self::InputAudioBufferCommitted {
                    item_id: Self::parse_value("item_id", data)?,
                },
                "input_audio_buffer.cleared" => Self::InputAudioBufferCleared,
                "input_audio_buffer.speech_started" => Self::InputAudioBufferSpeechStarted {
                    audio_start_ms: Self::parse_value("audio_start_ms", data)?,
                    item_id: Self::parse_value("item_id", data)?,
                },
                "input_audio_buffer.speech_stopped" => Self::InputAudioBufferSpeechStopped {
                    audio_end_ms: Self::parse_value("audio_end_ms", data)?,
                    item_id: Self::parse_value("item_id", data)?,
                },
                "rate_limits.updated" => {
                    Self::RateLimitsUpdated(Self::parse_value("rate_limits", data)?)
                }
                "response.audio.delta" => {
                    let delta: String = Self::parse_value("delta", data)?;
                    Self::ResponseAudioDelta {
                        delta: BASE64_STANDARD.decode(delta)?,
                        content_index: Self::parse_value("content_index", data)?,
                        output_index: Self::parse_value("output_index", data)?,
                        response_id: Self::parse_value("response_id", data)?,
                    }
                }
                "response.audio.done" => Self::ResponseAudioDone {
                    content_index: Self::parse_value("content_index", data)?,
                    output_index: Self::parse_value("output_index", data)?,
                    item_id: Self::parse_value("item_id", data)?,
                    response_id: Self::parse_value("response_id", data)?,
                },
                "response.audio_transcript.delta" => Self::ResponseAudioTranscriptDelta {
                    delta: Self::parse_value("delta", data)?,
                    content_index: Self::parse_value("content_index", data)?,
                    output_index: Self::parse_value("output_index", data)?,
                    item_id: Self::parse_value("item_id", data)?,
                    response_id: Self::parse_value("response_id", data)?,
                },
                "response.audio_transcript.done" => Self::ResponseAudioTranscriptDone {
                    transcript: Self::parse_value("transcript", data)?,
                    content_index: Self::parse_value("content_index", data)?,
                    output_index: Self::parse_value("output_index", data)?,
                    item_id: Self::parse_value("item_id", data)?,
                    response_id: Self::parse_value("response_id", data)?,
                },
                "response.created" => Self::ResponseCreated(Self::parse_value("response", data)?),
                "response.cancelled" => {
                    Self::ResponseCancelled(Self::parse_value("response", data)?)
                }
                "response.done" => Self::ResponseDone(Self::parse_value("response", data)?),
                "response.function_call_arguments.done" => {
                    Self::ResponseFunctionCallArgumentsDone {
                        name: Self::parse_value("name", data)?,
                        arguments: Self::parse_value("arguments", data)?,
                        output_index: Self::parse_value("output_index", data)?,
                        response_id: Self::parse_value("response_id", data)?,
                    }
                }
                "response.function_call.simple_browser" => {
                    Self::ResponseFunctionCallSimpleBrowser {
                        name: Self::parse_value("name", data)?,
                        session: Self::parse_value("session", data)?,
                    }
                }
                "response.output_item.added" => Self::ResponseOutputItemAdded {
                    item: Self::parse_value("item", data)?,
                    output_index: Self::parse_value("output_index", data)?,
                    response_id: Self::parse_value("response_id", data).unwrap_or_default(),
                },
                "response.output_item.done" => Self::ResponseOutputItemDone {
                    item: Self::parse_value("item", data)?,
                    output_index: Self::parse_value("output_index", data)?,
                    response_id: Self::parse_value("response_id", data)?,
                },
                "response.content_part.added" => Self::ResponseContentPartAdded {
                    part: ContentPart::from_value::<JsonError>(
                        data.get("part").unwrap_or(&Value::Null),
                    )?,
                    content_index: Self::parse_value("content_index", data)?,
                    output_index: Self::parse_value("output_index", data)?,
                    item_id: Self::parse_value("item_id", data)?,
                    response_id: Self::parse_value("response_id", data)?,
                },
                "response.content_part.done" => Self::ResponseContentPartDone {
                    part: ContentPart::from_value::<JsonError>(
                        data.get("part").unwrap_or(&Value::Null),
                    )?,
                    content_index: Self::parse_value("content_index", data)?,
                    output_index: Self::parse_value("output_index", data)?,
                    item_id: Self::parse_value("item_id", data)?,
                    response_id: Self::parse_value("response_id", data).unwrap_or_default(),
                },
                "response.text.delta" => Self::ResponseTextDelta {
                    delta: Self::parse_value("delta", data)?,
                    content_index: Self::parse_value("content_index", data)?,
                    output_index: Self::parse_value("output_index", data)?,
                    item_id: Self::parse_value("item_id", data)?,
                    response_id: Self::parse_value("response_id", data)?,
                },
                "response.text.done" => Self::ResponseTextDone {
                    text: Self::parse_value("text", data)?,
                    content_index: Self::parse_value("content_index", data)?,
                    output_index: Self::parse_value("output_index", data)?,
                    item_id: Self::parse_value("item_id", data)?,
                    response_id: Self::parse_value("response_id", data)?,
                },
                "session.created" => Self::SessionCreated(Self::parse_value("session", data)?),
                "session.updated" => Self::SessionUpdated(Self::parse_value("session", data)?),
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
            Some(e) => from_value(e.clone()),
            _ => Err(serde_json::Error::io(IoError::new(
                ErrorKind::InvalidData,
                key,
            ))),
        }
    }
}
