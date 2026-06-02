use fluvio::{
    consumer::Record as NativeRecord
};
use fluvio_future::task::run_block_on;
use futures_util::stream::StreamExt;
use futures_util::stream::Stream;
use std::pin::Pin;
use fluvio::dataplane::link::ErrorCode;

pub struct Record { pub inner: NativeRecord }

type ConsumerStreamInner = Pin<Box<dyn Stream<Item = Result<NativeRecord, ErrorCode>> + Send>>;
pub struct FluvioStream { pub inner: ConsumerStreamInner }

impl FluvioStream {
    pub fn next(self: &mut Self) -> Result<Box<Record>, String> {
        match run_block_on(self.inner.next()) {
            Some(Ok(rec)) => Ok(Box::new(Record { inner: rec })),
            Some(Err(e)) => Err(e.to_string()),
            None => Err("EOF".to_string()),
        }
    }
}

impl Record {
    pub fn value(self: &Self) -> Vec<u8> {
        self.inner.value().iter().cloned().collect()
    }
    pub fn key(self: &Self) -> Vec<u8> {
        self.inner.key().map(|k| k.iter().cloned().collect()).unwrap_or_default()
    }
    pub fn offset(self: &Self) -> i64 {
        self.inner.offset()
    }
}




