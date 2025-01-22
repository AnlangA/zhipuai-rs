//! # glm4 model name
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum GLM4{
    GLM4Plus,
    GLM4Air0111,
    GLM4AirX,
    GLM4Long,
    GLM4FlashX,
    GLM4Flash,
}

impl fmt::Display for GLM4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GLM4::GLM4Plus => write!(f, "glm-4-plus"),
            GLM4::GLM4Air0111 => write!(f, "glm-4-air-0111"),
            GLM4::GLM4AirX => write!(f, "glm-4-air-x"),
            GLM4::GLM4Long => write!(f, "glm-4-long"),
            GLM4::GLM4FlashX => write!(f, "glm-4-flash-x"),
            GLM4::GLM4Flash => write!(f, "glm-4-flash"),
        }
    }
}

impl From<GLM4> for &'static str {
    fn from(model: GLM4) -> Self {
        match model {
            GLM4::GLM4Plus => "glm-4-plus",
            GLM4::GLM4Air0111 => "glm-4-air-0111",
            GLM4::GLM4AirX => "glm-4-air-x",
            GLM4::GLM4Long => "glm-4-long",
            GLM4::GLM4FlashX => "glm-4-flash-x",
            GLM4::GLM4Flash => "glm-4-flash",
        }
    }
}


