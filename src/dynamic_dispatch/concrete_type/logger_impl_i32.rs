use std::pin::Pin;

use futures::{Stream, stream};

use crate::dynamic_dispatch::concrete_type::logger_trait::LoggerConcreteType;

pub struct LoggerI32;

impl LoggerConcreteType for LoggerI32 {
    type Item = i32;

    async fn get_log(&self) -> Pin<Box<dyn Stream<Item = Self::Item>>> {
        Box::pin(stream::iter(1..=3))
    }
}
