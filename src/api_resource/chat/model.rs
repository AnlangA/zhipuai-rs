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
    Glm4p1VThinkingFlash,
    Glm4p1VThinkingFlashX,
    GlmZeroPreview,
    GlmZ1Air,
    GlmZ1AirX,
    GlmZ1Flash,
    Glm4Voice,
    Glm4AllTools,
    CodeGeeX,
    CharGlm4,
    Glm4p5,
    Glm4p5Air,
    Glm4p5X,
    Glm4p5AirX,
    Glm4p5Flash,
    Glm4p5V,
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
            Self::Glm4p1VThinkingFlash => write!(f, "GLM-4.1V-Thinking-Flash"),
            Self::Glm4p1VThinkingFlashX => write!(f, "GLM-4.1V-Thinking-FlashX"),
            Self::GlmZeroPreview => write!(f, "glm-zero-preview"),
            Self::GlmZ1Air => write!(f, "glm-z1-air"),
            Self::GlmZ1AirX => write!(f, "glm-z1-airx"),
            Self::GlmZ1Flash => write!(f, "glm-z1-flash"),
            Self::Glm4Voice => write!(f, "glm-4-voice"),
            Self::Glm4AllTools => write!(f, "glm-4-alltools"),
            Self::CodeGeeX => write!(f, "codegeex-4"),
            Self::CharGlm4 => write!(f, "charglm-4"),
            Self::Glm4p5 => write!(f, "glm-4.5"),
            Self::Glm4p5Air => write!(f, "glm-4.5-air"),
            Self::Glm4p5X => write!(f, "glm-4.5-x"),
            Self::Glm4p5AirX => write!(f, "glm-4.5-airx"),
            Self::Glm4p5Flash => write!(f, "glm-4.5-flash"),
            Self::Glm4p5V => write!(f, "glm-4.5v"),
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
            ChatModelName::Glm4p1VThinkingFlash => "GLM-4.1V-Thinking-Flash",
            ChatModelName::Glm4p1VThinkingFlashX => "GLM-4.1V-Thinking-FlashX",
            ChatModelName::GlmZeroPreview => "glm-zero-preview",
            ChatModelName::GlmZ1Air => "glm-z1-air",
            ChatModelName::GlmZ1AirX => "glm-z1-airx",
            ChatModelName::GlmZ1Flash => "glm-z1-flash",
            ChatModelName::Glm4Voice => "glm-4-voice",
            ChatModelName::Glm4AllTools => "glm-4-alltools",
            ChatModelName::CodeGeeX => "codegeex-4",
            ChatModelName::CharGlm4 => "charglm-4",
            ChatModelName::Glm4p5 => "glm-4.5",
            ChatModelName::Glm4p5Air => "glm-4.5-air",
            ChatModelName::Glm4p5X => "glm-4.5-x",
            ChatModelName::Glm4p5AirX => "glm-4.5-airx",
            ChatModelName::Glm4p5Flash => "glm-4.5-flash",
            ChatModelName::Glm4p5V => "glm-4.5v",
        }
    }
}
