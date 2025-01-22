pub mod api;
pub mod data;
pub mod response;
pub mod iner_macro;
pub mod model;

pub use api::*;
pub use data::*;
pub use response::*;
pub use model::*;

use super::builder::Builder;

pub type Chat = ChatChatApiRequestBuilder;

impl Builder for Chat {
    type Item = Chat;
    fn new(model_name: &str) -> Self::Item {
        Chat::new(model_name)
    }
}