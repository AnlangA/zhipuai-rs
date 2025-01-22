//! # glm4 model name
use std::fmt::{self, write};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ChatModelName{
    GLM4Plus,
    GLM4Air0111,
    GLM4AirX,
    GLM4Long,
    GLM4FlashX,
    GLM4Flash,
    GLM4VPlus0111,
    GLM4VPlus,
    GLM4V,
    GLM4VFlash,
    GLMZeroPreview,
    GLM4Voice,
    GLM4Alltools,
    CodeGeeX,
    CharGLM4,
    Emohaa,

}

impl fmt::Display for ChatModelName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ChatModelName::GLM4Plus => write!(f, "glm-4-plus"),
            ChatModelName::GLM4Air0111 => write!(f, "glm-4-air-0111"),
            ChatModelName::GLM4AirX => write!(f, "glm-4-air-x"),
            ChatModelName::GLM4Long => write!(f, "glm-4-long"),
            ChatModelName::GLM4FlashX => write!(f, "glm-4-flash-x"),
            ChatModelName::GLM4Flash => write!(f, "glm-4-flash"),
            ChatModelName::GLM4VPlus0111 => write!(f, "glm-4v-plus-0111"),
            ChatModelName::GLM4VPlus => write!(f, "glm-4v-plus"),
            ChatModelName::GLM4V => write!(f, "glm-4v"),
            ChatModelName::GLM4VFlash => write!(f, "glm-4v-flash"),
            ChatModelName::GLMZeroPreview => write!(f, "glm-zero-preview"),
            ChatModelName::GLM4Voice => write!(f, "glm-4-voice"),
            ChatModelName::GLM4Alltools => write!(f, "glm-4-alltools"),
            ChatModelName::CodeGeeX => write!(f, "codegeex-4"),
            ChatModelName::CharGLM4 => write!(f, "char-glm-4"),
            ChatModelName::Emohaa => write!(f, "emohaa"),
        }
    }
}

impl From<ChatModelName> for &'static str {
    fn from(model: ChatModelName) -> Self {
        match model {
            ChatModelName::GLM4Plus => "glm-4-plus",
            ChatModelName::GLM4Air0111 => "glm-4-air-0111",
            ChatModelName::GLM4AirX => "glm-4-air-x",
            ChatModelName::GLM4Long => "glm-4-long",
            ChatModelName::GLM4FlashX => "glm-4-flash-x",
            ChatModelName::GLM4Flash => "glm-4-flash",
            ChatModelName::GLM4VPlus0111 => "glm-4v-plus-0111",
            ChatModelName::GLM4VPlus => "glm-4v-plus",
            ChatModelName::GLM4V => "glm-4v",
            ChatModelName::GLM4VFlash => "glm-4v-flash",
            ChatModelName::GLMZeroPreview => "glm-zero-preview",
            ChatModelName::GLM4Voice => "glm-4-voice",
            ChatModelName::GLM4Alltools => "glm-4-alltools",
            ChatModelName::CodeGeeX => "codegeex",
            ChatModelName::CharGLM4 => "char-glm-4",
            ChatModelName::Emohaa => "emohaa",
        }
    }
}


