use fluvio::FluvioAdmin as FluvioAdminNative;
use fluvio_sc_schema::topic::TopicSpec;
use fluvio_future::task::run_block_on;
use fluvio_sc_schema::objects::ListFilter;

pub struct FluvioAdmin { pub inner: FluvioAdminNative }


impl FluvioAdmin {
    pub fn connect() -> Result<Box<FluvioAdmin>, String> {
        run_block_on(FluvioAdminNative::connect()).map(|a| Box::new(FluvioAdmin { inner: a })).map_err(|e| e.to_string())
    }

    pub fn connect_with_config(config: &super::FluvioConfig) -> Result<Box<FluvioAdmin>, String> {
        run_block_on(FluvioAdminNative::connect_with_config(&config.inner))
            .map(|a| Box::new(FluvioAdmin { inner: a }))
            .map_err(|e| e.to_string())
    }

    pub fn topic_exists(self: &Self, topic: &str) -> bool {
        run_block_on(self.inner.list::<TopicSpec, ListFilter>(Vec::new()))
            .map(|topics| topics.into_iter().any(|t| t.name == topic))
            .unwrap_or(false)
    }

    pub fn create_topic(self: &Self, topic: &str, partitions: i32, replicas: i32) -> Result<(), String> {
        run_block_on(self.inner.create(topic.to_string(), false, TopicSpec::new_computed(partitions as u32, replicas as u32, None)))
            .map_err(|e| e.to_string())
    }

    pub fn list_topics(&self) -> Result<Vec<String>, String> {
        run_block_on(self.inner.list::<TopicSpec, ListFilter>(Vec::new()))
            .map(|topics| topics.into_iter().map(|t| t.name).collect())
            .map_err(|e| e.to_string())
    }

    pub fn delete_topic(self: &Self, topic: &str) -> Result<(), String> {
        run_block_on(self.inner.delete::<TopicSpec>(topic.to_string()))
            .map_err(|e| e.to_string())
    }

}
