use std::pin::Pin;

use futures::{Stream, stream};

use crate::dynamic_dispatch::any_type::logger_trait::LoggerAnyType;

pub struct LoggerAnyTypeDynamic;

impl LoggerAnyType<i32> for LoggerAnyTypeDynamic {
    async fn get_log(&self) -> Pin<Box<dyn Stream<Item = i32>>> {
        Box::pin(stream::iter(1..=3))
    }
}

impl LoggerAnyType<String> for LoggerAnyTypeDynamic {
    async fn get_log(&self) -> Pin<Box<dyn Stream<Item = String>>> {
        Box::pin(stream::iter(vec![
            "Log entry A".to_string(),
            "Log entry B".to_string(),
            "Log entry C".to_string(),
        ]))
    }
}
