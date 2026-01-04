use std::pin::Pin;

use futures::{Stream, stream};

use crate::dynamic_dispatch::concrete_type::logger_trait::LoggerConcreteType;

pub struct LoggerString;

impl LoggerConcreteType for LoggerString {
    type Item = String;

    async fn get_log(&self) -> Pin<Box<dyn Stream<Item = Self::Item>>> {
        Box::pin(stream::iter(vec![
            "A".to_string(),
            "B".to_string(),
            "C".to_string(),
        ]))
    }
}
