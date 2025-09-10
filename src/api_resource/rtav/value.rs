use crate::role::Role;
use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{Error as SerDeError, SeqAccess, Visitor},
    ser::SerializeSeq,
};
use serde_json::Value;
use std::{
    collections::HashMap,
    fmt::{Display, Formatter, Result as FmtResult},
};

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
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
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
pub struct SimpleBrowser {
    /// 搜索前的拖延话术，也会合成语音返回
    pub description: String,
    pub meta: String,
    pub search_meta: String,
    pub text_citation: String,
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub struct GreetingConfig {
    /// 是否开启开场白
    enable: bool,
}

impl GreetingConfig {
    /// 开启
    pub fn enable() -> Self {
        Self { enable: true }
    }

    /// 关闭
    pub fn disable() -> Self {
        Self { enable: false }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BetaFields {
    /// 是否开启内置的自动搜索（为true,会在服务端内置搜索引擎,无需传入），开关仅在audio模式下生效，video模式由模型控制自动补充搜索内容 默认为true
    auto_search: Option<bool>,
    /// 必填，通话模式：video_passive、audio（默认）
    #[serde(default)]
    pub chat_mode: ChatMode,
    /// 文本转语音的方式，支持：e2e。已不再支持修改该字段。
    pub tts_source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_last_text: Option<bool>,
    pub simple_browser: Option<SimpleBrowser>,
    /// 开场白内容
    pub greeting_config: Option<GreetingConfig>,
    /// 开场白内容
    pub content: Option<String>,
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

    /// 提供文本转语音的方式。
    /// 已不再支持修改该字段。
    ///
    /// # 参数
    /// * `tts_source`: 支持：e2e。
    pub fn with_tts_source(&mut self, tts_source: &str) -> &mut Self {
        self.tts_source = Some(tts_source.to_owned());
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

    /// 提供开场白设置
    ///
    /// # 参数
    /// * `greeting_config`: 开场白设置
    pub fn with_greeting_config(&mut self, greeting_config: GreetingConfig) -> &mut Self {
        self.greeting_config = Some(greeting_config);
        self
    }

    /// 提供开场白内容
    ///
    /// # 参数
    /// * `content`: 开场白内容
    pub fn with_content(&mut self, content: &str) -> &mut Self {
        self.content = Some(content.to_owned());
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
            simple_browser: None,
            greeting_config: None,
            content: Default::default(),
        }
    }
}

/// VAD检测的类型，支持client_vad（默认），server_vad
#[derive(Copy, Clone, Debug, Default, Deserialize, Serialize)]
pub(super) enum VadType {
    #[default]
    #[serde(rename = "client_vad")]
    ClientVad,
    #[serde(rename = "server_vad")]
    ServerVad,
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub struct TurnDetection {
    create_response: bool,
    interrupt_response: bool,
    prefix_padding_ms: i32,
    silence_duration_ms: i32,
    r#type: VadType,
}

impl TurnDetection {
    pub fn new() -> Self {
        Self {
            create_response: true,
            interrupt_response: true,
            prefix_padding_ms: 0,
            silence_duration_ms: 0,
            r#type: Default::default(),
        }
    }

    /// 使用client_vad（客户端语音检测，默认）
    pub fn with_client_vad(&mut self) -> &mut Self {
        self.r#type = VadType::ClientVad;
        self
    }

    /// 使用server_vad（服务端语音自动检测）
    pub fn with_server_vad(&mut self) -> &mut Self {
        self.r#type = VadType::ServerVad;
        self
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InputAudioNoiseReduction {
    /// 降噪类型。near_field 适用于近距离说话的麦克风，如耳机；far_field 适用于远距离麦克风，如笔记本电脑或会议室麦克风。
    r#type: String,
}

impl InputAudioNoiseReduction {
    pub fn near() -> Self {
        Self {
            r#type: "near_field".to_owned(),
        }
    }

    pub fn far() -> Self {
        Self {
            r#type: "far_field".to_owned(),
        }
    }
}

//noinspection SpellCheckingInspection
#[derive(Debug, Deserialize, Serialize)]
pub struct Session {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub object: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub id: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub model: String,
    /// [“text”, “audio”] (默认值)：输出文本和音频
    /// 1.会收到response.audio_transcript.delta/done、response.audio.delta/done;
    /// 2.会收到response.text.delta/done
    ///
    /// [“text”]：只输出文本
    /// 1.不会收到response.audio_transcript.delta/done、response.audio.delta/done;
    /// 2.会收到response.text.delta/done
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modalities: Vec<String>,
    /// 系统指令，用于引导模型生成期望的响应。
    pub instructions: Option<String>,
    /// 降噪模式，目前支持语音通话下的降噪
    pub input_audio_noise_reduction: Option<InputAudioNoiseReduction>,
    /// 默认女声:tongtong.
    /// 甜美女性：female-tianmei
    /// 青年大学生：male-qn-daxuesheng.
    /// 精英青年：male-qn-jingying.
    /// 萌萌女童：lovely_girl.
    /// 少女：female-shaonv
    pub voice: Option<String>,
    /// 音频输入格式，支持wav（wav48表示48K采样率）；
    #[serde(default)]
    pub input_audio_format: String,
    /// 音频输出格式，支持pcm、mp3，默认pcm
    #[serde(default)]
    pub output_audio_format: String,
    /// 回复的最大长度，对应文本 token计数， “0” <max_response_output_tokens<= “1024”，超过这个长度回复会被截断 。
    pub max_response_output_tokens: Option<String>,
    pub tool_choice: Option<String>,
    /// ServerVAD 时，更新tools要同时传入turn_detection。当前仅audio模式支持tools调用
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
            instructions: Default::default(),
            input_audio_noise_reduction: Default::default(),
            voice: Default::default(),
            input_audio_format: "wav".to_string(),
            output_audio_format: "pcm".to_string(),
            max_response_output_tokens: Default::default(),
            tool_choice: Default::default(),
            tools: Default::default(),
            temperature: None,
            turn_detection: None,
            beta_fields: Default::default(),
        }
    }

    /// glm-realtime-air
    pub fn with_air_model(&mut self) -> &mut Self {
        self.model = "glm-realtime-air".to_owned();
        self
    }

    pub fn with_flash_model(&mut self) -> &mut Self {
        self.model = "glm-realtime-flash".to_owned();
        self
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

    /// 提供降噪模式，目前支持语音通话下的降噪
    ///
    /// * 参数
    /// * `input_audio_noise_reduction`: 音频降噪模式。
    pub fn with_input_audio_noise_reduction(
        &mut self,
        input_audio_noise_reduction: InputAudioNoiseReduction,
    ) -> &mut Self {
        self.input_audio_noise_reduction = Some(input_audio_noise_reduction);
        self
    }

    /// 提供多模态输出类型。
    ///
    /// # 参数
    /// * `modalities`: 输出类型。
    pub fn with_modalities(&mut self, modalities: &[&str]) -> &mut Self {
        self.modalities = modalities.iter().map(|i| i.to_owned().into()).collect();
        self
    }

    /// 提供系统指令，用于引导模型生成期望的响应。
    ///
    /// # 参数
    /// * `instructions`: 系统指令。
    pub fn with_instructions(&mut self, instructions: &str) -> &mut Self {
        self.instructions = Some(instructions.to_string());
        self
    }

    /// 提供发音人。
    ///
    /// # 参数
    /// * `voice`:
    /// 默认女声:tongtong.
    /// 甜美女性：female-tianmei
    /// 青年大学生：male-qn-daxuesheng.
    /// 精英青年：male-qn-jingying.
    /// 萌萌女童：lovely_girl.
    /// 少女：female-shaonv
    pub fn with_voice(&mut self, voice: &str) -> &mut Self {
        self.voice = Some(voice.to_owned());
        self
    }

    /// 提供模型温度。
    ///
    /// # 参数
    /// * `temperature`: 取值范围 [0.0,1.0]。值越大，会使输出更随机，更具创造性；值越小，输出会更加稳定或确定。
    pub fn with_temperature(&mut self, temperature: f64) -> &mut Self {
        self.temperature = Some(temperature);
        self
    }

    /// 提供回复最大长度。
    ///
    /// # 参数
    /// * `max_response_output_tokens`: 对应文本 token计数， “0” <max_response_output_tokens<= “1024”，超过这个长度回复会被截断。
    pub fn with_max_response_output_tokens(
        &mut self,
        max_response_output_tokens: &str,
    ) -> &mut Self {
        self.max_response_output_tokens = Some(max_response_output_tokens.to_owned());
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
    /// * `tools`: ServerVAD 时，更新tools要同时传入turn_detection。当前仅audio模式支持tools调用。
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

/// 定义项的状态 (completed, incomplete)。
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ConversationItemStatus {
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "incomplete")]
    Incomplete,
    #[serde(rename = "in_progress")]
    InProgress,
}

impl ConversationItemStatus {
    fn deserialize<'de, D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let var = String::deserialize(deserializer)?;
        Ok(match var.as_str() {
            "completed" => Self::Completed,
            "incomplete" => Self::Incomplete,
            _ => Self::InProgress,
        })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ContentPart {
    Audio(String),
    Text(String),
    InputAudio,
}

impl ContentPart {
    pub(super) fn from_value<E>(value: &Value) -> Result<Self, E>
    where
        E: SerDeError,
    {
        let map = value
            .as_object()
            .ok_or(E::custom("Value must be object."))?;

        Ok(match map.get("type") {
            Some(Value::String(v)) if v == "text" => ContentPart::Text(
                map.get("text")
                    .map(|v| match v {
                        Value::String(v) => v.to_owned(),
                        _ => Default::default(),
                    })
                    .unwrap_or_default(),
            ),
            Some(Value::String(v)) if v == "audio" => ContentPart::Audio(
                map.get("transcript")
                    .map(|v| match v {
                        Value::String(v) => v.to_owned(),
                        _ => Default::default(),
                    })
                    .unwrap_or_default(),
            ),
            Some(Value::String(v)) if v == "input_audio" => ContentPart::InputAudio,
            Some(Value::String(v)) if !v.is_empty() => {
                return Err(E::custom(format!("Unsupported type {}.", v)));
            }
            _ => ContentPart::Audio(Default::default()),
        })
    }
}

/// 定义对话中的项，可以是消息、函数调用或函数调用响应。
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConversationItem {
    /// 始终为 “realtime.item”。
    #[serde(skip_serializing_if = "String::is_empty")]
    pub object: String,
    /// 项的类型 (message, function_call, function_call_output)。
    pub r#type: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub id: String,
    /// 项的状态 (completed, incomplete)。
    #[serde(deserialize_with = "ConversationItemStatus::deserialize")]
    pub status: ConversationItemStatus,
    /// 消息内容数组。
    #[serde(
        default,
        serialize_with = "ConversationItem::serialize_contents",
        deserialize_with = "ConversationItem::deserialize_contents"
    )]
    pub content: Vec<ContentPart>,
    /// 消息发送者的角色 (user, assistant, system)，仅在 message 类型时适用。
    #[serde(
        default,
        serialize_with = "ConversationItem::serialize_role",
        deserialize_with = "ConversationItem::deserialize_role"
    )]
    pub role: Option<Role>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 函数调用的名称，用于 function_call 类型。
    pub name: Option<String>,
    /// 函数调用的参数，用于 function_call 类型。
    pub arguments: Option<String>,
    /// 函数调用的输出，用于 function_call_output 类型。
    pub output: Option<String>,
}

impl ConversationItem {
    fn serialize_contents<S>(contents: &[ContentPart], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(contents.len()))?;
        for i in contents.iter() {
            let mut map = HashMap::<String, Value>::with_capacity(2);
            match i {
                ContentPart::Audio(v) => {
                    map.insert("type".to_owned(), Value::String("audio".to_owned()));
                    map.insert("transcript".to_owned(), Value::String(v.to_owned()));
                }
                ContentPart::Text(v) => {
                    map.insert("type".to_owned(), Value::String("text".to_owned()));
                    map.insert("text".to_owned(), Value::String(v.to_owned()));
                }
                ContentPart::InputAudio => {
                    map.insert("type".to_owned(), Value::String("input_audio".to_owned()));
                }
            }
            seq.serialize_element(&map)?;
        }

        seq.end()
    }

    fn deserialize_contents<'de, D>(deserializer: D) -> Result<Vec<ContentPart>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Vis;
        impl<'a> Visitor<'a> for Vis {
            type Value = Vec<ContentPart>;

            fn expecting(&self, formatter: &mut Formatter) -> FmtResult {
                write!(formatter, "Expect json array.")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'a>,
            {
                let mut res = if let Some(len) = seq.size_hint() {
                    Vec::with_capacity(len)
                } else {
                    Default::default()
                };
                while let Some(v) = seq.next_element::<Value>()? {
                    res.push(ContentPart::from_value(&v)?);
                }

                Ok(res)
            }
        }
        deserializer.deserialize_seq(Vis)
    }

    fn serialize_role<S>(role: &Option<Role>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let role = if let Some(role) = role {
            role.to_string()
        } else {
            Default::default()
        };
        serializer.serialize_str(&role)
    }

    fn deserialize_role<'de, D>(deserializer: D) -> Result<Option<Role>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let role = Option::<String>::deserialize(deserializer)?;
        Ok(role.map_or(None, |i| Some(i.as_str().into())))
    }

    pub fn new() -> Self {
        Self {
            object: Default::default(),
            r#type: Default::default(),
            id: Default::default(),
            status: ConversationItemStatus::Incomplete,
            content: Default::default(),
            role: Default::default(),
            name: None,
            arguments: Default::default(),
            output: None,
        }
    }

    /// 提供消息发送者的角色 (user, assistant, system)，仅在 message 类型时适用。
    ///
    /// # 参数
    /// * `role`: 角色类型。
    pub fn with_role(&mut self, role: Role) -> &mut Self {
        self.role = Some(role);
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

/// 定义响应的状态 (completed, cancelled, failed, incomplete)。
#[derive(Debug, Deserialize, Serialize)]
pub enum ResponseStatus {
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "incomplete")]
    Incomplete,
    #[serde(rename = "in_progress")]
    InProgress,
}

/// 定义服务器返回的响应对象结构。
#[derive(Debug, Deserialize, Serialize)]
pub struct Response {
    /// 始终为 “realtime.response”。
    pub object: Option<String>,
    /// 响应的唯一 ID。
    pub id: String,
    /// 响应的状态 (completed, cancelled, failed, incomplete)。
    pub status: ResponseStatus,
    pub usage: Option<Usage>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RateLimit {
    limit: u8,
    remaining: u8,
    name: String,
    reset_seconds: f32,
}
