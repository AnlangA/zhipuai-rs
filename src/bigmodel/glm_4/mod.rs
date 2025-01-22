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

pub type Glm4Mod = ApiRequestBuilder;

impl Builder for Glm4Mod {
    type Item = Glm4Mod;
    fn new(model_name: &str) -> Self::Item {
        Glm4Mod::new(model_name)
    }
}