use futures::{Stream, stream};

use crate::static_dispatch::generic_type::logger_trait::LogServiceTypeGeneric;

pub struct LoggerStaticTypeGeneric;

impl LogServiceTypeGeneric<i32> for LoggerStaticTypeGeneric {
    async fn get_log(&self) -> impl Stream<Item = i32> {
        stream::iter(1..=3)
    }
}

impl LogServiceTypeGeneric<String> for LoggerStaticTypeGeneric {
    async fn get_log(&self) -> impl Stream<Item = String> {
        stream::iter(vec![
            "Log entry A".to_string(),
            "Log entry B".to_string(),
            "Log entry C".to_string(),
        ])
    }
}
