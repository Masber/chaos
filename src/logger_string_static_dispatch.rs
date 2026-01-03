use futures::{Stream, stream};

use crate::logger_trait::LoggerService;

pub struct LoggerStringStatic;

impl LoggerService for LoggerStringStatic {
    type Item = String;
    async fn get_log(&self) -> impl Stream<Item = Self::Item> {
        stream::iter(vec!["A".to_string(), "B".to_string(), "C".to_string()])
    }
}
