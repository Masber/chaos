use std::pin::Pin;

use futures::Stream;

use crate::{
    common::{stream_i32, stream_string},
    dynamic_dispatch::any_type::logger_trait::LoggerAnyType,
};

pub struct LoggerAnyTypeDynamic;

impl LoggerAnyType<i32> for LoggerAnyTypeDynamic {
    // NOTE: `common::stream_string` returns `impl Stream<Item = String>` which does not comply
    // with `get_log` signature, however rust compiler is able to infer the correct type here
    // through `coertion` ref: https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility.
    async fn get_log(&self) -> Pin<Box<dyn Stream<Item = i32>>> {
        Box::pin(stream_i32())
    }
}

impl LoggerAnyType<String> for LoggerAnyTypeDynamic {
    // NOTE: `common::stream_string` returns `impl Stream<Item = String>` which does not comply
    // with `get_log` signature, however rust compiler is able to infer the correct type here
    // through `coertion` ref: https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility.
    async fn get_log(&self) -> Pin<Box<dyn Stream<Item = String>>> {
        Box::pin(stream_string())
    }
}
