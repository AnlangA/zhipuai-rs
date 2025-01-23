//! # a trait for creating Ai model builder
pub trait Builder {
    type Item;
    fn new(model_name: &str) -> Self::Item;
}
