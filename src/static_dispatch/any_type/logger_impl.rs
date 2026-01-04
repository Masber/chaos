use futures::{Stream, stream};

use crate::static_dispatch::any_type::logger_trait::LoggerAnyType;

pub struct Logger;

impl LoggerAnyType<i32> for Logger {
    async fn get_log(&self) -> impl Stream<Item = i32> {
        stream::iter(1..=3)
    }
}

impl LoggerAnyType<String> for Logger {
    async fn get_log(&self) -> impl Stream<Item = String> {
        stream::iter(vec![
            "Log entry A".to_string(),
            "Log entry B".to_string(),
            "Log entry C".to_string(),
        ])
    }
}
