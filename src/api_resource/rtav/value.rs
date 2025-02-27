use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Deserialize)]
pub struct Error {
    /// 错误的类型。例如，“invalid_request_error” 和 “server_error” 是错误类型。
    pub r#type: Option<String>,
    /// 用户可读的错误消息。
    pub message: Option<String>,
    /// 错误代码（如果有）。
    pub code: String,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(ref m) = self.message {
            write!(f, "{} (Error code: {})", m, self.code)
        } else if let Some(ref t) = self.r#type {
            write!(f, "{} (Error code: {})", t, self.code)
        } else {
            write!(f, "Other (Error code: {})", self.code)
        }
    }
}

impl std::error::Error for Error {}

#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize)]
pub enum ChatMode {
    #[default]
    #[serde(rename = "audio")]
    Audio,
    #[serde(rename = "video_passive")]
    VideoPassive,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BetaFields {
    /// 是否开启内置的自动搜索（为true,会在服务端内置搜索引擎,无需传入），开关仅在audio模式下生效，video模式由模型控制自动补充搜索内容 默认为true
    auto_search: Option<bool>,
    /// 必填，通话模式：video_passive、audio（默认）
    pub chat_mode: ChatMode,
    /// 语音转文字的方式，支持：e2e
    pub tts_source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_last_text: Option<bool>,
}

impl BetaFields {
    pub fn new() -> Self {
        Default::default()
    }

    /// 提供通话模式
    ///
    /// # 参数
    /// * `chat_mode`: video_passive、audio（默认）
    pub fn with_chat_mode(&mut self, chat_mode: ChatMode) -> &mut Self {
        self.chat_mode = chat_mode;
        self
    }

    /// 提供语音转文字的方式。
    ///
    /// # 参数
    /// * `tts_source`: 支持：e2e。
    pub fn with_tts_source(&mut self, tts_source: &str) -> &mut Self {
        self.tts_source = Some(tts_source.to_string());
        self
    }

    /// 提供内置的自动搜索
    ///
    /// # 参数
    /// * `auto_search`: 为true,会在服务端内置搜索引擎,无需传入，开关仅在audio模式下生效，video模式由模型控制自动补充搜索内容（默认为true）
    pub fn with_auto_search(&mut self, auto_search: bool) -> &mut Self {
        self.auto_search = Some(auto_search);
        self
    }
}

impl Default for BetaFields {
    fn default() -> Self {
        Self {
            auto_search: None,
            chat_mode: Default::default(),
            tts_source: Some("e2e".to_string()),
            is_last_text: None,
        }
    }
}

/// VAD检测的类型，支持client_vad（默认），server_vad
#[derive(Copy, Clone, Debug, Default, Deserialize, Serialize)]
pub enum VadType {
    #[default]
    #[serde(rename = "client_vad")]
    ClientVad,
    #[serde(rename = "server_vad")]
    ServerVad,
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub struct TurnDetection {
    r#type: VadType,
}

impl TurnDetection {
    pub fn new() -> Self {
        Self {
            r#type: Default::default(),
        }
    }

    /// 提供语音激活类型。
    ///
    /// # 参数
    /// * `vad_type`: VAD检测的类型，支持client_vad（默认），server_vad。
    pub fn with_vad_type(&mut self, vad_type: VadType) -> &mut Self {
        self.r#type = vad_type;
        self
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Session {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub object: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub id: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub model: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub modalities: Vec<String>,
    /// 系统指令，用于引导模型生成期望的响应。
    pub instructions: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub voice: String,
    /// 音频输入格式，支持wav；
    pub input_audio_format: String,
    /// 音频输出格式，支持pcm、mp3，默认pcm
    pub output_audio_format: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub tool_choice: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<super::super::chat::Function>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    pub turn_detection: Option<TurnDetection>,
    pub beta_fields: BetaFields,
}

impl Session {
    /// 创建事件会话
    pub fn new() -> Self {
        Self {
            object: Default::default(),
            id: Default::default(),
            model: Default::default(),
            modalities: Default::default(),
            instructions: "".to_string(),
            voice: Default::default(),
            input_audio_format: "wav".to_string(),
            output_audio_format: "pcm".to_string(),
            tool_choice: Default::default(),
            tools: Default::default(),
            temperature: None,
            turn_detection: None,
            beta_fields: Default::default(),
        }
    }

    /// 提供音频输入格式，支持wav；
    ///
    /// * 参数
    /// * `input_audio_format`: 音频格式。
    pub fn with_input_audio_format(&mut self, input_audio_format: &str) -> &mut Self {
        self.input_audio_format = input_audio_format.to_string();
        self
    }

    /// 提供音频输出格式，支持pcm、mp3，默认pcm
    ///
    /// * 参数
    /// * `output_audio_format`: 音频格式。
    pub fn with_output_audio_format(&mut self, output_audio_format: &str) -> &mut Self {
        self.output_audio_format = output_audio_format.to_string();
        self
    }

    /// 提供系统指令，用于引导模型生成期望的响应。
    ///
    /// # 参数
    /// * `instructions`: 系统指令。
    pub fn with_instructions(&mut self, instructions: &str) -> &mut Self {
        self.instructions = instructions.to_string();
        self
    }

    /// 提供检测模式
    ///
    /// # 参数
    /// * `turn_detection`: 检测模式。
    pub fn with_turn_detection(&mut self, turn_detection: &TurnDetection) -> &mut Self {
        self.turn_detection = Some(*turn_detection);
        self
    }

    /// 提供beta_fields。
    pub fn with_beta_fields(&mut self, beta_fields: &BetaFields) -> &mut Self {
        self.beta_fields = beta_fields.clone();
        self
    }

    /// 提供工具函数。
    ///
    /// # 参数
    /// * `tools`: ServerVAD 时，更新tools要同时传入turn_detection，防止误设置回客户端VAD
    pub fn with_tools(&mut self, tools: &[super::super::chat::Function]) -> &mut Self {
        self.tools = Some(tools.to_vec());
        self
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Conversation {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub object: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConversationItem {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub object: String,
    pub r#type: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
}

impl ConversationItem {
    pub fn new() -> Self {
        Self {
            object: Default::default(),
            r#type: Default::default(),
            id: Default::default(),
            text: None,
            output: None,
        }
    }

    /// 提供文字消息内容。
    ///
    /// # 参数
    /// * `text`: 消息。
    pub fn with_text(&mut self, text: &str) -> &mut Self {
        self.r#type = "text".to_string();
        self.text = Some(text.to_string());
        self
    }

    /// 提供函数调用的结果输入，适用于function_call_output的类型。
    ///
    /// # 参数
    /// * `output`: 消息。
    pub fn with_function_call_output(&mut self, output: &str) -> &mut Self {
        self.r#type = "function_call_output".to_string();
        self.output = Some(output.to_string());
        self
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Usage {
    pub total_tokens: u32,
    pub input_tokens: u32,
    pub output_tokens: u32,
    pub input_token_details: InputTokenDetails,
    pub output_token_details: OutputTokenDetails,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InputTokenDetails {
    pub text_tokens: u32,
    pub audio_tokens: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OutputTokenDetails {
    pub text_tokens: u32,
    pub audio_tokens: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Response {
    pub object: Option<String>,
    pub id: String,
    pub status: String,
    pub usage: Usage,
}
