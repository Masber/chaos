use futures::Stream;

use crate::{
    common::{stream_i32, stream_string},
    static_dispatch::any_type::logger_trait::LoggerAnyType,
};

pub struct Logger;

impl LoggerAnyType<i32> for Logger {
    async fn get_log(&self) -> impl Stream<Item = i32> {
        stream_i32()
    }
}

impl LoggerAnyType<String> for Logger {
    async fn get_log(&self) -> impl Stream<Item = String> {
        stream_string()
    }
}
