//! # LLM role
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
            Self::System => "system",
            Self::User => "user",
            Self::Assistant => "assistant",
        }
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let role_str = match self {
            Self::System => "system",
            Self::User => "user",
            Self::Assistant => "assistant",
        };
        write!(f, "{}", role_str)
    }
}

impl From<&str> for Role {
    fn from(value: &str) -> Self {
        match value {
            "user" => Self::User,
            "assistant" => Self::Assistant,
            "system" => Self::System,
            _ => unimplemented!(),
        }
    }
}
