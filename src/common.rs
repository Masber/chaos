use futures::{Stream, stream};

pub fn stream_i32() -> impl Stream<Item = i32> {
    stream::iter(1..=3)
}

pub fn stream_string() -> impl Stream<Item = String> {
    stream::iter(vec!["A".to_string(), "B".to_string(), "C".to_string()])
}
