use futures::{Stream, stream};

use crate::static_dispatch::concrete_type::logger_trait::LoggerServiceTypeSpecific;

pub struct LoggerI32Static;

impl LoggerServiceTypeSpecific for LoggerI32Static {
    type Item = i32;

    async fn get_log(&self) -> impl Stream<Item = Self::Item> {
        stream::iter(0..=3)
    }
}
