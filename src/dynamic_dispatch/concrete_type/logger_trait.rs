use std::pin::Pin;

use futures::Stream;

pub trait LoggerConcreteType {
    type Item;

    fn get_log(&self) -> impl Future<Output = Pin<Box<dyn Stream<Item = Self::Item>>>> + Send;
}
