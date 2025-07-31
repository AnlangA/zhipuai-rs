///! the chat AI api of zhipu
use super::data::*;
use serde::{Deserialize, Serialize};

const API_URL: &str = "https://open.bigmodel.cn/api/paas/v4/chat/completions";

#[derive(Serialize, Deserialize, Debug, Clone)]
#[non_exhaustive]
pub struct ChatApiRequest {
    /// model name, default is "glm-4"
    model: String,
    /// Passed by the client side and needs to be unique. It is a unique identifier used to distinguish each request.
    /// If not provided by the client side, the platform will generate it by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    request_id: Option<String>,
    /// The user ID, if provided, can be used for tracking or personalization.
    #[serde(skip_serializing_if = "Option::is_none")]
    user_id: Option<String>,
    /// When "do_sample" is true, the sampling strategy is enabled. When do_sample is false,
    /// sampling strategy parameters such as [temperature](#temperature) and [top_p](#top_p) will not take effect. The default value is true.
    /// See [do_sample](#do_sample) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    do_sample: Option<bool>,
    /// If set to true, the response will be streamed back in chunks.
    /// 【WARNING】: You cant not enable it when using `function`。
    #[serde(skip_serializing_if = "Option::is_none")]
    stream: Option<bool>,
    /// This parameter configuration is only supported by models of the GLM-4.5 version and above. It controls whether the large model enables chain-of-thought reasoning.
    #[serde(skip_serializing_if = "Option::is_none")]
    thinking: Option<Thinking>,
    /// The temperature parameter controls the randomness of the model's output. Higher values result in more random outputs.
    /// This parameter is only effective when [do_sample](#do_sample) is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    /// The top_p parameter controls the nucleus sampling strategy. This parameter is only effective when [do_sample](#do_sample) is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
    /// The maximum number of tokens to generate in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    /// When calling a language model, the current conversation message list is provided as the prompt input of the model in the form of a JSON array.
    /// For example, `{"role": "user", "content": "Hello"}`. Possible message types include system messages, user messages, assistant messages, and tool messages.
    messages: Vec<Message>,
    /// A list of strings that indicate the model should stop generating further tokens when any of these strings is encountered.
    #[serde(skip_serializing_if = "Option::is_none")]
    stop: Option<Vec<String>>,
    /// A list of tools that the model can use during the conversation.
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<Tool>>,
    /// Specifies the tool to be used. This can be a specific tool name or "auto" to let the model decide.
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_choice: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "extra")]
    code_context: Option<Extra>,
}

impl ChatApiRequest {
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

pub struct ChatApiRequestBuilder {
    /// model name, default is "glm-4"
    model: String,
    /// Passed by the client side and needs to be unique. It is a unique identifier used to distinguish each request.
    /// If not provided by the client side, the platform will generate it by default.
    request_id: Option<String>,
    /// The user ID, if provided, can be used for tracking or personalization.
    user_id: Option<String>,
    /// When "do_sample" is true, the sampling strategy is enabled. When do_sample is false,
    /// sampling strategy parameters such as [temperature](#temperature) and [top_p](#top_p) will not take effect. The default value is true.
    /// See [do_sample](#do_sample) for more details.
    do_sample: Option<bool>,
    /// If set to true, the response will be streamed back in chunks.
    stream: Option<bool>,
    /// This parameter configuration is only supported by models of the GLM-4.5 version and above. It controls whether the large model enables chain-of-thought reasoning.
    thinking: Option<Thinking>,
    /// The temperature parameter controls the randomness of the model's output. Higher values result in more random outputs.
    /// This parameter is only effective when [do_sample](#do_sample) is true.
    temperature: Option<f32>,
    /// The top_p parameter controls the nucleus sampling strategy. This parameter is only effective when [do_sample](#do_sample) is true.
    top_p: Option<f32>,
    /// The maximum number of tokens to generate in the response.
    max_tokens: Option<u32>,
    /// When calling a language model, the current conversation message list is provided as the prompt input of the model in the form of a JSON array.
    /// For example, `{"role": "user", "content": "Hello"}`. Possible message types include system messages, user messages, assistant messages, and tool messages.
    messages: Vec<Message>,
    /// A list of strings that indicate the model should stop generating further tokens when any of these strings is encountered.
    stop: Option<Vec<String>>,
    /// A list of tools that the model can use during the conversation.
    tools: Option<Vec<Tool>>,
    /// Specifies the tool to be used. This can be a specific tool name or "auto" to let the model decide.
    tool_choice: Option<String>,
    /// codegeex model information.
    code_context: Option<Extra>,
}

impl ChatApiRequestBuilder {
    pub fn new(model: &str) -> Self {
        Self {
            model: model.to_string(),
            request_id: None,
            user_id: None,
            do_sample: None,
            stream: None,
            thinking: None,
            temperature: None,
            top_p: None,
            max_tokens: None,
            messages: vec![],
            stop: None,
            tools: None,
            tool_choice: None,
            code_context: None,
        }
    }

    /// set model name。
    /// example: "glm-4"
    /// ```ignore
    /// let mut builder = ChatApiRequestBuilder::new("glm-4");
    /// ```
    pub fn model_name(&mut self, model: &str) -> &mut Self {
        self.model = model.to_string();
        self
    }

    /// set request_id
    /// example: "1234567890"
    /// ```ignore
    /// let mut builder = ChatApiRequestBuilder::new("glm-4")
    ///                  .request_id("1234567890");
    /// ```
    pub fn request_id(&mut self, request_id: &str) -> &mut Self {
        self.request_id = Some(request_id.to_string());
        self
    }

    /// set user_id
    /// example: "1234567890"
    /// ```ignore
    /// let mut builder = ChatApiRequestBuilder::new("glm-4")
    ///                  .request_id("1234567890")
    ///                  .user_id("1234567890");
    /// ```
    pub fn user_id(&mut self, user_id: &str) -> &mut Self {
        self.user_id = Some(user_id.to_string());
        self
    }

    /// set do_sample
    /// example: true
    /// ```ignore
    /// let mut builder = ChatApiRequestBuilder::new("glm-4")
    ///                  .request_id("1234567890")
    ///                  .user_id("1234567890")
    ///                  .do_sample_enable(true);
    /// ```
    pub fn do_sample_enable(&mut self, do_sample: bool) -> &mut Self {
        self.do_sample = Some(do_sample);
        self
    }

    /// set stream enable。
    /// default: false
    /// WARNING: 1. if `stream` is true, the response will be a stream response.
    ///          2. if `stream` is true, the `max_tokens` parameter will be ignored.
    ///          3. If `stream` is true and the `function` is used, a `stream` response cannot be obtained.
    /// example: true
    /// ```ignore
    /// let mut builder = ChatApiRequestBuilder::new("glm-4")
    ///                  .request_id("1234567890")
    ///                  .user_id("1234567890")
    ///                  .do_sample_enable(true)
    ///                  .stream_enable(true);
    /// ```
    pub fn stream_enable(&mut self, stream: bool) -> &mut Self {
        self.stream = Some(stream);
        self
    }
    /// Enable chain-of-thought reasoning for the model.
    /// This parameter configuration is only supported by models of the GLM-4.5 version and above.
    /// When enabled, the model will show its reasoning process before providing the final answer.
    /// default: Some(Thinking::Enable)
    /// example:
    /// ```ignore
    /// let mut builder = ChatApiRequestBuilder::new("glm-4.5")
    ///                  .request_id("1234567890")
    ///                  .user_id("1234567890")
    ///                  .do_sample_enable(true)
    ///                  .stream_enable(true)
    ///                  .thinking_enable();
    /// ```
    pub fn thinking_enable(&mut self) -> &mut Self {
        self.thinking = Some(Thinking::enable());
        self
    }
    /// Disable chain-of-thought reasoning for the model.
    /// This parameter configuration is only supported by models of the GLM-4.5 version and above.
    /// When disabled, the model will provide direct answers without showing the reasoning process.
    /// default: Some(Thinking::Enable)
    /// example:
    /// ```ignore
    /// let mut builder = ChatApiRequestBuilder::new("glm-4.5")
    ///                  .request_id("1234567890")
    ///                  .user_id("1234567890")
    ///                  .do_sample_enable(true)
    ///                  .stream_enable(true)
    ///                  .thinking_disable();
    /// ```
    pub fn thinking_disable(&mut self) -> &mut Self {
        self.thinking = Some(Thinking::disable());
        self
    }
    /// set temperature
    /// default: None。**when it is `None`, the model will use the default value : 0.95**
    /// example: 0.95
    /// ```ignore
    /// let mut builder = ChatApiRequestBuilder::new("glm-4")
    ///                  .request_id("1234567890")
    ///                  .user_id("1234567890")
    ///                  .do_sample_enable(true)
    ///                  .stream_enable(true)
    ///                  .temperature(0.95);
    /// ```
    pub fn temperature(&mut self, mut temperature: f32) -> &mut Self {
        if temperature <= 0.0 {
            self.do_sample = Some(false);
            temperature = 0.1;
        } else if temperature >= 1.0 {
            self.do_sample = Some(false);
            temperature = 1.0;
        }
        self.temperature = Some(temperature);
        self
    }

    /// set top_p
    /// default: None。**when it is `None`, the model will use the default value : 0.7**
    /// example: 0.7
    /// ```ignore
    /// let mut builder = ChatApiRequestBuilder::new("glm-4")
    ///                  .request_id("1234567890")
    ///                  .user_id("1234567890")
    ///                  .do_sample_enable(true)
    ///                  .stream_enable(true)
    ///                  .temperature(0.95)
    ///                  .top_p(0.7);
    /// ```
    pub fn top_p(&mut self, mut top_p: f32) -> &mut Self {
        if top_p <= 0.0 {
            top_p = 0.01;
        } else if top_p >= 1.0 {
            top_p = 0.99;
        }
        self.top_p = Some(top_p);
        self
    }

    /// set max_tokens
    /// default: None。**when it is `None`, the model will use the default value : 1024**
    /// example: 1024
    /// ```ignore
    /// let mut builder = ChatApiRequestBuilder::new("glm-4")
    ///                  .request_id("1234567890")
    ///                  .user_id("1234567890")
    ///                  .do_sample_enable(true)
    ///                  .stream_enable(true)
    ///                  .temperature(0.95)
    ///                  .top_p(0.7)
    ///                  .max_tokens(1024);
    /// ```
    pub fn max_tokens(&mut self, max_tokens: u32) -> &mut Self {
        self.max_tokens = Some(max_tokens);
        self
    }

    /// set stop
    /// default: None
    /// example:
    /// ```ignore
    /// let mut builder = ChatApiRequestBuilder::new("glm-4")
    ///                  .request_id("1234567890")
    ///                  .user_id("1234567890")
    ///                  .do_sample_enable(true)
    ///                  .stream_enable(true)
    ///                  .temperature(0.95)
    ///                  .top_p(0.7)
    ///                  .max_tokens(1024)
    ///                  .stop(vec!["\n", "##"]);
    /// ```
    pub fn stop(&mut self, stop: Vec<String>) -> &mut Self {
        self.stop = Some(stop);
        self
    }

    /// set tools
    /// default: None
    /// example: vec![Tool::new("search", "so nice", Some(true))]
    /// ```ignore
    /// let mut builder = ChatApiRequestBuilder::new("glm-4")
    ///                  .add_messages(Message::new("user", Some("hello".to_string()), None))
    ///                  .add_tools(Tool::new("search", "so nice", Some(true)));
    /// ```
    pub fn add_tools(&mut self, tools: Tool) -> &mut Self {
        self.tools.get_or_insert_with(Vec::new).push(tools);
        self
    }

    /// set tool_choice
    /// default: None。
    /// just support `auto`。Do not use it!
    pub fn tool_choice(&mut self, tool_choice: &str) -> &mut Self {
        self.tool_choice = Some(tool_choice.to_string());
        self
    }

    /// add message
    /// default: None
    /// example: vec![Message::new("user", Some("hello".to_string()), None)]
    /// ```ignore
    /// let mut builder = ChatApiRequestBuilder::new("glm-4")
    ///                  .request_id("1234567890")
    ///                  .user_id("1234567890")
    ///                  .do_sample_enable(true)
    ///                  .stream_enable(true)
    ///                  .temperature(0.95)
    ///                  .top_p(0.7)
    ///                  .max_tokens(1024)
    ///                  .stop(vec!["\n", "##"])
    ///                  .add_message(Message::new("user", Some("hello".to_string()), None));
    ///                  .add_message(Message::new("assistant", Some("hello".to_string()), None));
    /// ```
    pub fn add_message(&mut self, message: Message) -> &mut Self {
        self.messages.push(message);
        self
    }

    /// add messages alse see [`add_message`]
    pub fn add_messages(&mut self, messages: Messages) -> &mut Self {
        self.messages.extend(messages.messages);
        self
    }

    /// set code_context for code generation model (CodeGeeX)
    /// default: None
    pub fn add_code_context(&mut self, code_context: Extra) -> &mut Self {
        self.code_context = Some(code_context);
        self
    }

    pub fn build(&self) -> (String, ChatApiRequest) {
        (
            API_URL.to_string(),
            ChatApiRequest {
                model: self.model.clone(),
                request_id: self.request_id.clone(),
                user_id: self.user_id.clone(),
                do_sample: self.do_sample,
                stream: self.stream,
                thinking: self.thinking.clone(),
                temperature: self.temperature,
                top_p: self.top_p,
                max_tokens: self.max_tokens,
                messages: self.messages.clone(),
                stop: self.stop.clone(),
                tools: self.tools.clone(),
                tool_choice: self.tool_choice.clone(),
                code_context: self.code_context.clone(),
            },
        )
    }
}
