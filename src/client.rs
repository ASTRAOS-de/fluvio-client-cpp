use fluvio::{
    Fluvio as FluvioNative,
    Offset as OffsetNative,
    consumer::ConsumerConfigExtBuilder as ConsumerConfigExtBuilderNative
};
use fluvio_future::task::run_block_on;
use crate::config::FluvioConfig;
use crate::{FluvioStream, TopicProducerConfigBuilder, TopicProducerPool};

pub struct Fluvio { pub inner: FluvioNative }


impl Fluvio {
    pub fn connect() -> Result<Box<Fluvio>, String> {
        run_block_on(FluvioNative::connect())
            .map(|fluvio| Box::new(Fluvio { inner: fluvio }))
            .map_err(|e| e.to_string())
    }

    pub fn connect_with_config(config: &FluvioConfig) -> Result<Box<Fluvio>, String> {
        run_block_on(FluvioNative::connect_with_config(&config.inner))
            .map(|fluvio| Box::new(Fluvio { inner: fluvio }))
            .map_err(|e| e.to_string())
    }
    pub fn topic_producer(self: &Self, topic: &str) -> Result<Box<TopicProducerPool>, String> {
        run_block_on(self.inner.topic_producer(topic))
            .map(|producer| Box::new(TopicProducerPool { inner: producer }))
            .map_err(|e| e.to_string())
    }

    pub fn topic_producer_with_config(self: &Self, topic: &str, config: &TopicProducerConfigBuilder) -> Result<Box<TopicProducerPool>, String> {
        let built_config = config.inner.build().map_err(|e| e.to_string())?;
        run_block_on(self.inner.topic_producer_with_config(topic, built_config))
            .map(|producer| Box::new(TopicProducerPool { inner: producer }))
            .map_err(|e| e.to_string())
    }
    
    pub fn consumer_stream(self: &Self, topic: &str, partition: u32, offset_index: i64) -> Result<Box<FluvioStream>, String> {
        let offset = if offset_index == -1 { OffsetNative::end() } else if offset_index == 0 { OffsetNative::beginning() } else { OffsetNative::absolute(offset_index).unwrap() };
        let config = ConsumerConfigExtBuilderNative::default()
            .topic(topic.to_string())
            .partition(partition)
            .offset_start(offset)
            .build()
            .map_err(|e| e.to_string())?;

        let consumer_stream = run_block_on(self.inner.consumer_with_config(config)).map_err(|e| e.to_string())?;
        Ok(Box::new(FluvioStream { inner: Box::pin(consumer_stream) }))
    }
}



