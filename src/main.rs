mod dynamic_dispatch;
mod static_dispatch;

use futures::{StreamExt, executor};

use crate::{
    dynamic_dispatch::{LogServiceDynamic, LoggerI32Dynamic, LoggerStringDynamic},
    static_dispatch::{
        any_type::{logger_impl::Logger, logger_trait::LoggerAnyType},
        concrete_type::{
            logger_impl_i32::LoggerI32, logger_impl_string::LoggerString,
            logger_trait::LoggerConcreteType,
        },
    },
};

fn main() {
    println!("Static Dispatch i32 Logger:");

    let logger = LoggerI32;

    let fut_journald_logger = async { logger.get_log().await };

    let mut stream_values = executor::block_on(fut_journald_logger);

    let fut_logger_values = async {
        while let Some(value) = stream_values.next().await {
            println!("Value: {:?}", value);
        }
    };

    executor::block_on(fut_logger_values);

    println!("Static Dispatch String Logger:");

    let logger = LoggerString;

    let fut_logger = async { logger.get_log().await };

    let mut stream_values = executor::block_on(fut_logger);

    let fut_logger_values = async {
        while let Some(value) = stream_values.next().await {
            println!("Value: {:?}", value);
        }
    };

    executor::block_on(fut_logger_values);

    println!("Static Dispatch Generic Logger:");

    let logger = Logger;

    let fut_logger = async { LoggerAnyType::<String>::get_log(&logger).await };

    let mut stream_values = executor::block_on(fut_logger);

    let fut_logger_values = async {
        while let Some(value) = stream_values.next().await {
            println!("Value: {:?}", value);
        }
    };

    executor::block_on(fut_logger_values);

    println!("Dynamic Dispatch Logger:");

    let logger = LoggerI32Dynamic;

    let fut_journald_logger = async { logger.get_log().await };

    let mut stream_values = executor::block_on(fut_journald_logger);

    let fut_logger_values = async {
        while let Some(value) = stream_values.next().await {
            println!("Value: {:?}", value);
        }
    };

    executor::block_on(fut_logger_values);

    let logger = LoggerStringDynamic;

    let fut_values_syslog = async { logger.get_log().await };

    let mut stream_values = executor::block_on(fut_values_syslog);

    let fut_logger_values = async {
        while let Some(value) = stream_values.next().await {
            println!("Value: {:?}", value);
        }
    };

    executor::block_on(fut_logger_values);
}
