use std::pin::Pin;

use futures::Stream;

/// Using type parameters ref: https://doc.rust-lang.org/reference/types/parameters.html
pub trait LoggerAnyType<Item> {
    fn get_log(&self) -> impl Future<Output = Pin<Box<dyn Stream<Item = Item>>>> + Send;
}
