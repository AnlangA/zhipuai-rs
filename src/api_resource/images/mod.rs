pub mod api;
pub mod data;
pub mod model;
pub mod response;

pub use api::*;
pub use data::*;
pub use model::*;
pub use response::*;

use super::builder::Builder;

pub type Images = ImagesApiRequestBuilder;

impl Builder for Images {
    type Item = Images;
    fn new(model_name: &str) -> Self::Item {
        Images::new(model_name)
    }
}
