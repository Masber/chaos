use futures::Stream;

pub trait LoggerAnyType<Item> {
    fn get_log(&self) -> impl Future<Output = impl Stream<Item = Item>> + Send;
}
