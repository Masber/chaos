mod dynamic_dispatch;
mod logger_generic_static_dispatch;
mod logger_i32_static_dispatch;
mod logger_string_static_dispatch;
mod logger_trait;

use futures::{StreamExt, executor};

use crate::{
    dynamic_dispatch::{LogServiceDynamic, LoggerI32Dynamic, LoggerStringDynamic},
    logger_generic_static_dispatch::{LogService, LoggerGenericStatic},
    logger_i32_static_dispatch::LoggerI32Static,
    logger_string_static_dispatch::LoggerStringStatic,
    logger_trait::LoggerService,
};

fn main() {
    println!("Static Dispatch i32 Logger:");

    let logger = LoggerI32Static;

    let fut_journald_logger = async { logger.get_log().await };

    let mut stream_values = executor::block_on(fut_journald_logger);

    let fut_logger_values = async {
        while let Some(value) = stream_values.next().await {
            println!("Value: {:?}", value);
        }
    };

    executor::block_on(fut_logger_values);

    println!("Static Dispatch String Logger:");

    let logger = LoggerStringStatic;

    let fut_logger = async { logger.get_log().await };

    let mut stream_values = executor::block_on(fut_logger);

    let fut_logger_values = async {
        while let Some(value) = stream_values.next().await {
            println!("Value: {:?}", value);
        }
    };

    executor::block_on(fut_logger_values);

    println!("Static Dispatch Generic Logger:");

    let logger = LoggerGenericStatic;

    let fut_logger = async { LogService::<String>::get_log(&logger).await };

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
