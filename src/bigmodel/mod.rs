pub mod glm_4;
pub mod builder;

pub use std::marker::PhantomData;
pub use builder::Builder;
pub use glm_4::*;

pub struct BigModel<T> {
    _marker: PhantomData<T>,
}

impl<T> BigModel<T> 
where T: Builder
{
    pub fn new(name: &str) -> T::Item {
        T::new(name)
    }
}