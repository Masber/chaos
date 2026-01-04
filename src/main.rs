mod dynamic_dispatch;
mod static_dispatch_generic_type_logger_impl;
mod static_dispatch_generic_type_logger_trait;
mod static_dispatch_specific_type_i32_logger_impl;
mod static_dispatch_specific_type_logger_trait;
mod static_dispatch_specific_type_string_logger_impl;

use futures::{StreamExt, executor};

use crate::{
    dynamic_dispatch::{LogServiceDynamic, LoggerI32Dynamic, LoggerStringDynamic},
    static_dispatch_generic_type_logger_impl::LoggerStaticTypeGeneric,
    static_dispatch_generic_type_logger_trait::LogServiceTypeGeneric,
    static_dispatch_specific_type_i32_logger_impl::LoggerI32Static,
    static_dispatch_specific_type_logger_trait::LoggerServiceTypeSpecific,
    static_dispatch_specific_type_string_logger_impl::LoggerStringStatic,
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

    let logger = LoggerStaticTypeGeneric;

    let fut_logger = async { LogServiceTypeGeneric::<String>::get_log(&logger).await };

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
