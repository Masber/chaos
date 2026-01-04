use futures::{Stream, stream};

use crate::static_dispatch::concrete_type::logger_trait::LoggerConcreteType;

pub struct LoggerString;

impl LoggerConcreteType for LoggerString {
    type Item = String;

    async fn get_log(&self) -> impl Stream<Item = Self::Item> {
        stream::iter(vec!["A".to_string(), "B".to_string(), "C".to_string()])
    }
}
