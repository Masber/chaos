mod dynamic_dispatch;
mod static_dispatch;

use futures::{StreamExt, executor};

use crate::{
    dynamic_dispatch::{
        any_type::{logger_trait::LoggerAnyType, logget_impl::LoggerAnyTypeDynamic},
        concrete_type::{
            logger_impl_i32::LoggerI32 as LoggerI32Dynamic,
            logger_trait::LoggerConcreteType as LoggerConcreteTypeDynamic,
            logget_impl_string::LoggerString as LoggerStringDynamic,
        },
    },
    static_dispatch::{
        any_type::{
            logger_impl::Logger as LoggerAnyTypeStatic,
            logger_trait::LoggerAnyType as LoggerTraitAnyTypeStatic,
        },
        concrete_type::{
            logger_impl_i32::LoggerI32 as LoggerI32Static,
            logger_impl_string::LoggerString as LoggerStringStatic,
            logger_trait::LoggerConcreteType as LoggerConcreteTypeStatic,
        },
    },
};

fn logger_static_dispatch_i32() {
    println!("Static Dispatch i32 Logger:");

    let logger = LoggerI32Static;

    let fut_logger = async { logger.get_log().await };

    let mut stream_values = executor::block_on(fut_logger);

    let fut_logger_values = async {
        while let Some(value) = stream_values.next().await {
            println!("Value: {value:?}");
        }
    };

    executor::block_on(fut_logger_values);
}

fn logger_static_dispatch_string() {
    println!("Static Dispatch String Logger:");

    let logger = LoggerStringStatic;

    let fut_logger = async { logger.get_log().await };

    let mut stream_values = executor::block_on(fut_logger);

    let fut_logger_values = async {
        while let Some(value) = stream_values.next().await {
            println!("Value: {value:?}");
        }
    };

    executor::block_on(fut_logger_values);
}

fn logger_static_dispatch_any_type() {
    println!("Static Dispatch any type Logger:");

    let logger = LoggerAnyTypeStatic;

    let fut_logger = async { LoggerTraitAnyTypeStatic::<String>::get_log(&logger).await };

    let mut stream_values = executor::block_on(fut_logger);

    let fut_logger_values = async {
        while let Some(value) = stream_values.next().await {
            println!("Value: {value:?}");
        }
    };

    executor::block_on(fut_logger_values);
}

fn logger_dynamic_dispatch_i32() {
    println!("Dynamic Dispatch i32 Logger:");

    let logger = LoggerI32Dynamic;

    let fut_logger = async { logger.get_log().await };

    let mut stream_values = executor::block_on(fut_logger);

    let fut_logger_values = async {
        while let Some(value) = stream_values.next().await {
            println!("Value: {value:?}");
        }
    };

    executor::block_on(fut_logger_values);
}

fn logger_dynamic_dispatch_string() {
    println!("Dynamic Dispatch String Logger:");

    let logger = LoggerStringDynamic;

    let fut_logger = async { logger.get_log().await };

    let mut stream_values = executor::block_on(fut_logger);

    let fut_logger_values = async {
        while let Some(value) = stream_values.next().await {
            println!("Value: {value:?}");
        }
    };

    executor::block_on(fut_logger_values);
}

fn logger_dynamic_dispatch_any_type() {
    println!("Dynamic Dispatch any type Logger:");

    let logger = LoggerAnyTypeDynamic;

    let fut_logger = async { LoggerAnyType::<String>::get_log(&logger).await };

    let mut stream_values = executor::block_on(fut_logger);

    let fut_logger_values = async {
        while let Some(value) = stream_values.next().await {
            println!("Value: {value:?}");
        }
    };

    executor::block_on(fut_logger_values);
}

fn main() {
    logger_static_dispatch_i32();

    logger_static_dispatch_string();

    logger_static_dispatch_any_type();

    logger_dynamic_dispatch_i32();

    logger_dynamic_dispatch_string();

    logger_dynamic_dispatch_any_type();
}
