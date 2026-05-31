use fluvio::TopicProducerPool as TopicProducerPoolNative;
use fluvio_future::task::run_block_on;
use crate::produce_output::ProduceOutput;

pub struct TopicProducerPool { pub inner: TopicProducerPoolNative }

impl TopicProducerPool {
    pub fn send(self: &Self, key: &[u8], value: &[u8]) -> Result<Box<ProduceOutput>, String> {
        run_block_on(self.inner.send(key, value))
            .map(|out| Box::new(ProduceOutput { inner: Some(out) }))
            .map_err(|e| e.to_string())
    }
    pub fn flush(self: &Self) -> Result<(), String> {
        run_block_on(self.inner.flush())
            .map(|_| ())
            .map_err(|e| e.to_string())
    }

}



