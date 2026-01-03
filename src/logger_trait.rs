use futures::Stream;

pub trait LoggerService {
    type Item;

    fn get_log(&self) -> impl Future<Output = impl Stream<Item = Self::Item>> + Send;
}
