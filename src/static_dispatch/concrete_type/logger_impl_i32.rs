use futures::{Stream, stream};

use crate::static_dispatch::concrete_type::logger_trait::LoggerConcreteType;

pub struct LoggerI32;

impl LoggerConcreteType for LoggerI32 {
    type Item = i32;

    async fn get_log(&self) -> impl Stream<Item = Self::Item> {
        stream::iter(0..=3)
    }
}
