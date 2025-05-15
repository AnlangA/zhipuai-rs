pub mod api;
pub mod data;
pub mod model;
pub mod response;

pub use api::*;
pub use data::*;
pub use model::*;
pub use response::*;

use super::builder::Builder;

pub type Videos = VideosApiRequestBuilder;

impl Builder for Videos {
    type Item = Videos;
    fn new(model_name: &str) -> Self::Item {
        Videos::new(model_name)
    }
}
