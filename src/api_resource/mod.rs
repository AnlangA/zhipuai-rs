pub mod chat;
pub mod builder;

pub use std::marker::PhantomData;
pub use builder::Builder;

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