//! # glm4 model name
use std::fmt;

//noinspection SpellCheckingInspection
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ChatModelName {
    Glm4Plus,
    Glm4Air250414,
    Glm4AirX,
    Glm4Long,
    Glm4FlashX,
    Glm4Flash,
    Glm4Flash250414,
    Glm4VPlus0111,
    Glm4VPlus,
    Glm4V,
    Glm4VFlash,
    Glm41VThinkingFlash,
    Glm41VThinkingFlashX,
    GlmZeroPreview,
    GlmZ1Air,
    GlmZ1AirX,
    GlmZ1Flash,
    Glm4Voice,
    Glm4AllTools,
    CodeGeeX,
    CharGlm4,
    Emohaa,
}

impl fmt::Display for ChatModelName {
    //noinspection SpellCheckingInspection
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Glm4Plus => write!(f, "glm-4-plus"),
            Self::Glm4Air250414 => write!(f, "glm-4-air-250414"),
            Self::Glm4AirX => write!(f, "glm-4-airx"),
            Self::Glm4Long => write!(f, "glm-4-long"),
            Self::Glm4FlashX => write!(f, "glm-4-flashx"),
            Self::Glm4Flash => write!(f, "glm-4-flash"),
            Self::Glm4Flash250414 => write!(f, "glm-4-flash-250414"),
            Self::Glm4VPlus0111 => write!(f, "glm-4v-plus-0111"),
            Self::Glm4VPlus => write!(f, "glm-4v-plus"),
            Self::Glm4V => write!(f, "glm-4v"),
            Self::Glm4VFlash => write!(f, "glm-4v-flash"),
            Self::Glm41VThinkingFlash => write!(f, "GLM-4.1V-Thinking-Flash"),
            Self::Glm41VThinkingFlashX => write!(f, "GLM-4.1V-Thinking-FlashX"),
            Self::GlmZeroPreview => write!(f, "glm-zero-preview"),
            Self::GlmZ1Air => write!(f, "glm-z1-air"),
            Self::GlmZ1AirX => write!(f, "glm-z1-airx"),
            Self::GlmZ1Flash => write!(f, "glm-z1-flash"),
            Self::Glm4Voice => write!(f, "glm-4-voice"),
            Self::Glm4AllTools => write!(f, "glm-4-alltools"),
            Self::CodeGeeX => write!(f, "codegeex-4"),
            Self::CharGlm4 => write!(f, "char-glm-4"),
            Self::Emohaa => write!(f, "emohaa"),
        }
    }
}

impl From<ChatModelName> for &'static str {
    //noinspection SpellCheckingInspection
    fn from(model: ChatModelName) -> Self {
        match model {
            ChatModelName::Glm4Plus => "glm-4-plus",
            ChatModelName::Glm4Air250414 => "glm-4-air-250414",
            ChatModelName::Glm4AirX => "glm-4-air-x",
            ChatModelName::Glm4Long => "glm-4-long",
            ChatModelName::Glm4FlashX => "glm-4-flash-x",
            ChatModelName::Glm4Flash => "glm-4-flash",
            ChatModelName::Glm4Flash250414 => "glm-4-flash-250414",
            ChatModelName::Glm4VPlus0111 => "glm-4v-plus-0111",
            ChatModelName::Glm4VPlus => "glm-4v-plus",
            ChatModelName::Glm4V => "glm-4v",
            ChatModelName::Glm4VFlash => "glm-4v-flash",
            ChatModelName::Glm41VThinkingFlash => "GLM-4.1V-Thinking-Flash",
            ChatModelName::Glm41VThinkingFlashX => "GLM-4.1V-Thinking-FlashX",
            ChatModelName::GlmZeroPreview => "glm-zero-preview",
            ChatModelName::GlmZ1Air => "glm-z1-air",
            ChatModelName::GlmZ1AirX => "glm-z1-airx",
            ChatModelName::GlmZ1Flash => "glm-z1-flash",
            ChatModelName::Glm4Voice => "glm-4-voice",
            ChatModelName::Glm4AllTools => "glm-4-alltools",
            ChatModelName::CodeGeeX => "codegeex-4",
            ChatModelName::CharGlm4 => "char-glm-4",
            ChatModelName::Emohaa => "emohaa",
        }
    }
}
