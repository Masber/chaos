use futures::{Stream, stream};

pub trait LogService<Item> {
    fn get_log(&self) -> impl Future<Output = impl Stream<Item = Item>> + Send;
}

pub struct LoggerGenericStatic;

impl LogService<i32> for LoggerGenericStatic {
    async fn get_log(&self) -> impl Stream<Item = i32> {
        stream::iter(1..=3)
    }
}

impl LogService<String> for LoggerGenericStatic {
    async fn get_log(&self) -> impl Stream<Item = String> {
        stream::iter(vec![
            "Log entry A".to_string(),
            "Log entry B".to_string(),
            "Log entry C".to_string(),
        ])
    }
}
