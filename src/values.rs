//! # LLM name
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Role {
    System,
    User,
    Assistant,
}

impl From<Role> for &str {
    fn from(s: Role) -> Self {
        match s {
            Role::System => "system",
            Role::User => "user",
            Role::Assistant => "assistant",
        }
    }
}

impl From<Role> for String {
    fn from(s: Role) -> Self {
        s.to_string()
    }
}

// Optionally, add AsRef<str> for &str references
impl AsRef<str> for Role {
    fn as_ref(&self) -> &str {
        match self {
            Role::System => "system",
            Role::User => "user",
            Role::Assistant => "assistant",
        }
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let role_str = match self {
            Role::System => "system",
            Role::User => "user",
            Role::Assistant => "assistant",
        };
        write!(f, "{}", role_str)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Model {
    GLM4Plus,
    GLM40520,
    GLM4Long,
    GLM4Air,
    GLM4AirX,
    GLM4FlashX,
    GLM4Flash,
    GLM4VPlus,
    GLM4V,
    GLM4VFlash,
    GLM4AllTools,
    CogVideoX,
    CogView35,
    CogView3,
    Embedded3,
    ChatGLM3,
    Emohaa,
    CodeGeex4,
}

impl fmt::Display for Model {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let model_str = match self {
            Self::GLM4Plus => "glm-4-plus",
            Self::GLM40520 => "glm-4-0520",
            Self::GLM4Long => "glm-4-long",
            Self::GLM4Air => "glm-4-air",
            Self::GLM4AirX => "glm-4-airX",
            Self::GLM4FlashX => "glm-4-flashX",
            Self::GLM4Flash => "glm-4-flash",
            Self::GLM4VPlus => "glm-4v-plus",
            Self::GLM4V => "glm-4v",
            Self::GLM4VFlash => "glm-4v-flash",
            Self::GLM4AllTools => "glm-4-alltools",
            Self::CogVideoX => "cogvideoX",
            Self::CogView35 => "cogview-3.5",
            Self::CogView3 => "cogview-3",
            Self::Embedded3 => "embedded-3",
            Self::ChatGLM3 => "chatglm-3",
            Self::Emohaa => "emohaa",
            Self::CodeGeex4 => "codegeex-4",
        };
        write!(f, "{}", model_str)
    }
}

impl From<Model> for &'static str {
    fn from(model: Model) -> Self {
        match model {
            Model::GLM4Plus => "glm-4-plus",
            Model::GLM40520 => "glm-4-0520",
            Model::GLM4Long => "glm-4-long",
            Model::GLM4Air => "glm-4-air",
            Model::GLM4AirX => "glm-4-airX",
            Model::GLM4FlashX => "glm-4-flashX",
            Model::GLM4Flash => "glm-4-flash",
            Model::GLM4VPlus => "glm-4v-plus",
            Model::GLM4V => "glm-4v",
            Model::GLM4VFlash => "glm-4v-flash",
            Model::GLM4AllTools => "glm-4-alltools",
            Model::CogVideoX => "cogvideoX",
            Model::CogView35 => "cogview-3.5",
            Model::CogView3 => "cogview-3",
            Model::Embedded3 => "embedded-3",
            Model::ChatGLM3 => "chatglm-3",
            Model::Emohaa => "emohaa",
            Model::CodeGeex4 => "codegeex-4",
        }
    }
}

impl Into<String> for Model {
    fn into(self) -> String {
        self.to_string()
    }
}

impl AsRef<str> for Model {
    fn as_ref(&self) -> &str {
        match self {
            Self::GLM4Plus => "glm-4-plus",
            Self::GLM40520 => "glm-4-0520",
            Self::GLM4Long => "glm-4-long",
            Self::GLM4Air => "glm-4-air",
            Self::GLM4AirX => "glm-4-airX",
            Self::GLM4FlashX => "glm-4-flashX",
            Self::GLM4Flash => "glm-4-flash",
            Self::GLM4VPlus => "glm-4v-plus",
            Self::GLM4V => "glm-4v",
            Self::GLM4VFlash => "glm-4v-flash",
            Self::GLM4AllTools => "glm-4-alltools",
            Self::CogVideoX => "cogvideoX",
            Self::CogView35 => "cogview-3.5",
            Self::CogView3 => "cogview-3",
            Self::Embedded3 => "embedded-3",
            Self::ChatGLM3 => "chatglm-3",
            Self::Emohaa => "emohaa",
            Self::CodeGeex4 => "codegeex-4",
        }
    }
}

