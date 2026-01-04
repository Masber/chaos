use std::pin::Pin;

use futures::Stream;

use crate::{
    common::stream_i32, dynamic_dispatch::concrete_type::logger_trait::LoggerConcreteType,
};

pub struct LoggerI32;

impl LoggerConcreteType for LoggerI32 {
    type Item = i32;

    // NOTE: `common::stream_string` returns `impl Stream<Item = String>` which does not comply
    // with `get_log` signature, however rust compiler is able to infer the correct type here
    // through `coertion` ref: https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility.
    async fn get_log(&self) -> Pin<Box<dyn Stream<Item = Self::Item>>> {
        Box::pin(stream_i32())
    }
}
