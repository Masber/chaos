use futures::Stream;

use crate::{
    common::stream_string, static_dispatch::concrete_type::logger_trait::LoggerConcreteType,
};

pub struct LoggerString;

impl LoggerConcreteType for LoggerString {
    type Item = String;

    async fn get_log(&self) -> impl Stream<Item = Self::Item> {
        stream_string()
    }
}
