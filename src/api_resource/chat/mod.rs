pub mod api;
pub mod data;
pub mod iner_macro;
pub mod model;
pub mod response;

pub use api::*;
pub use data::*;
pub use model::*;
pub use response::*;

use super::builder::Builder;

pub type Chat = ChatApiRequestBuilder;

impl Builder for Chat {
    type Item = Chat;
    fn new(model_name: &str) -> Self::Item {
        Chat::new(model_name)
    }
}
