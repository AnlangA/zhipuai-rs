use super::Error as RealtimeError;
use base64::DecodeError;
use reqwest::header::InvalidHeaderValue;
use serde_json::Error as JsonError;
use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
    io::Error as IoError,
    time::SystemTimeError,
};
use tokio_tungstenite::tungstenite::Error as TungsteniteError;

#[derive(Debug)]
pub enum SessionError {
    Decode(DecodeError),
    InvalidHeader(InvalidHeaderValue),
    Io(IoError),
    Json(JsonError),
    Realtime(RealtimeError),
    StatusCode(String),
    SystemTime(SystemTimeError),
    TungsteniteError(TungsteniteError),
}

impl Display for SessionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "RealtimeApiError: ")?;
        match self {
            Self::Decode(e) => Display::fmt(e, f),
            Self::InvalidHeader(e) => Display::fmt(e, f),
            Self::Io(e) => Display::fmt(e, f),
            Self::Json(e) => Display::fmt(e, f),
            Self::Realtime(e) => Display::fmt(e, f),
            Self::StatusCode(e) => Display::fmt(e, f),
            Self::SystemTime(e) => Display::fmt(e, f),
            Self::TungsteniteError(e) => Display::fmt(e, f),
        }
    }
}

impl Error for SessionError {}

impl From<IoError> for SessionError {
    fn from(value: IoError) -> Self {
        Self::Io(value)
    }
}

impl From<JsonError> for SessionError {
    fn from(value: JsonError) -> Self {
        Self::Json(value)
    }
}

impl From<SystemTimeError> for SessionError {
    fn from(value: SystemTimeError) -> Self {
        Self::SystemTime(value)
    }
}

impl From<TungsteniteError> for SessionError {
    fn from(value: TungsteniteError) -> Self {
        Self::TungsteniteError(value)
    }
}

impl From<InvalidHeaderValue> for SessionError {
    fn from(value: InvalidHeaderValue) -> Self {
        Self::InvalidHeader(value)
    }
}

impl From<DecodeError> for SessionError {
    fn from(value: DecodeError) -> Self {
        Self::Decode(value)
    }
}

impl From<RealtimeError> for SessionError {
    fn from(value: RealtimeError) -> Self {
        Self::Realtime(value)
    }
}
