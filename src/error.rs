use crate::api_resource::rtav::Error as RealtimeError;
use base64::DecodeError;
use reqwest::{Error as ReqwestError, header::InvalidHeaderValue};
use serde_json::Error as JsonError;
use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
    io::Error as IoError,
    time::SystemTimeError,
};
use tokio_tungstenite::tungstenite::Error as TungsteniteError;

/// 通用的API错误类型
#[derive(Debug)]
pub enum ZhipuApiError {
    Decode(DecodeError),
    InvalidHeader(InvalidHeaderValue),
    Io(IoError),
    Json(JsonError),
    Realtime(RealtimeError),
    Reqwest(ReqwestError),
    StatusCode(String),
    SystemTime(SystemTimeError),
    TungsteniteError(TungsteniteError),
}

impl Display for ZhipuApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ZhipuApiError: ")?;
        match self {
            Self::Decode(e) => Display::fmt(e, f),
            Self::InvalidHeader(e) => Display::fmt(e, f),
            Self::Io(e) => Display::fmt(e, f),
            Self::Json(e) => Display::fmt(e, f),
            Self::Realtime(e) => Display::fmt(e, f),
            Self::Reqwest(e) => Display::fmt(e, f),
            Self::StatusCode(e) => Display::fmt(e, f),
            Self::SystemTime(e) => Display::fmt(e, f),
            Self::TungsteniteError(e) => Display::fmt(e, f),
        }
    }
}

impl Error for ZhipuApiError {}

impl From<IoError> for ZhipuApiError {
    fn from(value: IoError) -> Self {
        Self::Io(value)
    }
}

impl From<JsonError> for ZhipuApiError {
    fn from(value: JsonError) -> Self {
        Self::Json(value)
    }
}

impl From<SystemTimeError> for ZhipuApiError {
    fn from(value: SystemTimeError) -> Self {
        Self::SystemTime(value)
    }
}

impl From<TungsteniteError> for ZhipuApiError {
    fn from(value: TungsteniteError) -> Self {
        Self::TungsteniteError(value)
    }
}

impl From<InvalidHeaderValue> for ZhipuApiError {
    fn from(value: InvalidHeaderValue) -> Self {
        Self::InvalidHeader(value)
    }
}

impl From<DecodeError> for ZhipuApiError {
    fn from(value: DecodeError) -> Self {
        Self::Decode(value)
    }
}

impl From<RealtimeError> for ZhipuApiError {
    fn from(value: RealtimeError) -> Self {
        Self::Realtime(value)
    }
}

impl From<ReqwestError> for ZhipuApiError {
    fn from(value: ReqwestError) -> Self {
        Self::Reqwest(value)
    }
}
