use fluvio::FluvioAdmin as FluvioAdminNative;
use fluvio_sc_schema::topic::TopicSpec;
use fluvio_future::task::run_block_on;

pub struct FluvioAdmin { pub inner: FluvioAdminNative }


impl FluvioAdmin {
    pub fn connect() -> Result<Box<FluvioAdmin>, String> {
        run_block_on(FluvioAdminNative::connect()).map(|a| Box::new(FluvioAdmin { inner: a })).map_err(|e| e.to_string())
    }

    pub fn create_topic(self: &Self, topic: &str, partitions: i32, replicas: i32) -> Result<(), String> {
        run_block_on(self.inner.create(topic.to_string(), false, TopicSpec::new_computed(partitions as u32, replicas as u32, None)))
            .map_err(|e| e.to_string())
    }

    pub fn delete_topic(self: &Self, topic: &str) -> Result<(), String> {
        run_block_on(self.inner.delete::<TopicSpec>(topic.to_string()))
            .map_err(|e| e.to_string())
    }

}
