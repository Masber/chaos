use futures::{Stream, stream};

use crate::logger_trait::LoggerService;

pub struct LoggerI32Static;

impl LoggerService for LoggerI32Static {
    type Item = i32;

    async fn get_log(&self) -> impl Stream<Item = Self::Item> {
        stream::iter(0..=3)
    }
}
