//! # CogView model name

use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
#[non_exhaustive]
pub enum VideosModelName {
    Cogvideox2,
    CogvideoxFlash,
}


impl fmt::Display for VideosModelName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Cogvideox2 => write!(f, "cogvideox-2"),
            Self::CogvideoxFlash => write!(f, "cogvideox-flash"),
        }
    }
}

impl From<VideosModelName> for &'static str {
    fn from(model: VideosModelName) -> &'static str {
        match model {
            VideosModelName::Cogvideox2 => "cogvideox-2",
            VideosModelName::CogvideoxFlash => "cogvideox-flash",
        }
    }
}