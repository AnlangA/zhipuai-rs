pub mod builder;
pub mod chat;
pub mod images;
pub mod rtav;
pub mod videos;

use builder::Builder;
use std::marker::PhantomData;

pub struct BigModel<T> {
    _marker: PhantomData<T>,
}

impl<T> BigModel<T>
where
    T: Builder,
{
    pub fn new(name: &str) -> T::Item {
        T::new(name)
    }
}
