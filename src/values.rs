//! # LLM name
//!
//! | 模型 | 描述 | 上下文 | 最大输出 |
//! |------|------|--------|----------|
//! | GLM-4-Plus New | 高智能旗舰: 性能全面提升，长文本和复杂任务能力显著增强 | 128K | 4K |
//! | GLM-4-0520 | 高智能模型: 适用于处理高度复杂和多样化的任务 | 128K | 4K |
//! | GLM-4-Long | 超长输入: 专为处理超长文本和记忆型任务设计 | 1M | 4K |
//! | GLM-4-AirX | 极速推理: 具有超快的推理速度和强大的推理效果 | 8K | 4K |
//! | GLM-4-Air | 高性价比: 推理能力和价格之间最平衡的模型 | 128K | 4K |
//! | GLM-4-FlashX | 高速低价: Flash增强版本，超快推理速度 | 128K | 4K |
//! | GLM-4-Flash | 免费调用: 智谱AI首个免费API，零成本调用大模型 | 128K | 4K |
//! | GLM-4V | 图像理解: 具备图像理解能力和推理能力 | 2K | 1k |
//! | GLM-4-AllTools | Agent模型: 自主规划和执行复杂任务 | 128K | 4K |
//! | GLM-4 | 旧版旗舰: 发布于2024年1月16日，目前已被GLM-4-0520取代 | 128K | 4K |
//!
//! # 多模态模型
//!
//! | 模型 | 描述 | 最大输入 | 输出分辨率 |
//! |------|------|----------|------------|
//! | GLM-4V-Plus New | 视频和图像理解: 具备视频内容和多图片的理解能力 | 8K | - |
//! | GLM-4V | 图像理解: 具备图像理解能力和推理能力 | 2K | - |
//! | CogVideoX | 视频生成: 输入文本或图片即可轻松制作视频 | 0.5K | 1440x960 |
//! | CogView-3.5 New | 图片生成: 根据用户文字描述生成高质量图像，支持多图片尺寸 | 1k | 1024x1024<br>768x1344<br>864x1152 等 |
//! | CogView-3 | 图片生成: 根据用户文字描述快速、精准生成图像 | 1k | 1024x1024 |
//!
//! # 向量模型
//!
//! | 模型 | 描述 | 最大输入 | 向量维度 |
//! |------|------|----------|----------|
//! | Embedding-3 | 最新模型: 支持自定义向量维度 | 8K | 2048 |
//! | Embedding-2 | 旧版模型: 目前已被Embedding-3取代 | 8K | 1024 |
//!
//! # 其他模型
//!
//! | 模型 | 描述 | 上下文 | 最大输出 |
//! |------|------|--------|----------|
//! | ChatGLM-3 | 拟人模型: 适用于情感陪伴和虚拟角色 | 4K | 2K |
//! | Emohaa | 心理模型: 具备专业咨询能力，帮助用户理解情感并应对精神问题 | 8K | 4k |
//! | CodeGeeX-4 | 代码模型: 适用于代码自动补全任务 | 128K | 4k |

use std::fmt;
use std::convert::From;
use std::convert::AsRef;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Role {
    System,
    User,
    Assistant,
}

impl From<Role> for &str {
    fn from(s: Role) -> Self {
        match s {
            Role::System => "System",
            Role::User => "User",
            Role::Assistant => "Assistant",
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
            Role::System => "System",
            Role::User => "User",
            Role::Assistant => "Assistant",
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
            Model::GLM4Plus => "glm-4-plus",
            Model::GLM40520 => "glm-4-0520",
            Model::GLM4Long => "glm-4-long",
            Model::GLM4Air => "glm-4-air",
            Model::GLM4AirX => "glm-4-airX",
            Model::GLM4FlashX => "glm-4-flashX",
            Model::GLM4Flash => "glm-4-flash",
            Model::GLM4VPlus => "glm-4V-plus",
            Model::GLM4V => "glm-4V",
            Model::GLM4AllTools => "glm-4-alltools",
            Model::CogVideoX => "cogvideoX",
            Model::CogView35 => "cogview-3.5",
            Model::CogView3 => "cogview-3",
            Model::Embedded3 => "embedded-3",
            Model::ChatGLM3 => "chatglm-3",
            Model::Emohaa => "emohaa",
            Model::CodeGeex4 => "codegeex-4",
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
            Model::GLM4VPlus => "glm-4V-plus",
            Model::GLM4V => "glm-4V",
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
            Model::GLM4Plus => "glm-4-plus",
            Model::GLM40520 => "glm-4-0520",
            Model::GLM4Long => "glm-4-long",
            Model::GLM4Air => "glm-4-air",
            Model::GLM4AirX => "glm-4-airX",
            Model::GLM4FlashX => "glm-4-flashX",
            Model::GLM4Flash => "glm-4-flash",
            Model::GLM4VPlus => "glm-4V-plus",
            Model::GLM4V => "glm-4V",
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

