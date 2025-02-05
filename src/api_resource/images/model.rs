//! # CogView model name

use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
#[non_exhaustive]
pub enum ImagesModelName {
    Cogview4,
    CogView3Flash,
}


impl fmt::Display for ImagesModelName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Cogview4 => write!(f, "cogview-4"),
            Self::CogView3Flash => write!(f, "cogview-3-flash"),
        }
    }
}

impl From<ImagesModelName> for &'static str {
    fn from(model: ImagesModelName) -> &'static str {
        match model {
            ImagesModelName::Cogview4 => "cogview-4",
            ImagesModelName::CogView3Flash => "cogview-3-flash",
        }
    }
}