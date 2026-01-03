use std::pin::Pin;

use futures::{Stream, stream};

// Trait definition
pub trait LogServiceDynamic {
    type Item;

    fn get_log(&self) -> impl Future<Output = Pin<Box<dyn Stream<Item = Self::Item>>>> + Send;
}

// Struct implementing the Logger trait
pub struct LoggerI32Dynamic;

impl LogServiceDynamic for LoggerI32Dynamic {
    type Item = i32;

    async fn get_log(&self) -> Pin<Box<dyn Stream<Item = Self::Item>>> {
        Box::pin(stream::iter(1..=3))
    }
}

pub struct LoggerStringDynamic;

impl LogServiceDynamic for LoggerStringDynamic {
    type Item = String;

    async fn get_log(&self) -> Pin<Box<dyn Stream<Item = Self::Item>>> {
        Box::pin(stream::iter(vec![
            "A".to_string(),
            "B".to_string(),
            "C".to_string(),
        ]))
    }
}
