pub mod admin;
pub mod client;
pub mod config;
pub mod consumer;
pub mod produce_output;
pub mod producer;
pub mod c_api;

#[cxx::bridge]
mod ffi {
    extern "Rust" {
        type Fluvio;
        type TopicProducerPool;
        type FluvioStream;
        type Record;

        type FluvioConfig;
        type ConsumerConfigBuilder;
        type TopicProducerConfigBuilder;
        type ProduceOutput;
        type RecordMetadata;
        type FluvioAdmin;

        /// Connects to a Fluvio cluster
        #[Self = "Fluvio"]
        fn connect() -> Result<Box<Fluvio>>;

        /// Connects to a Fluvio cluster with explicit config
        #[Self = "Fluvio"]
        fn connect_with_config(config: &FluvioConfig) -> Result<Box<Fluvio>>;

        /// Creates a producer for the specified topic
        fn topic_producer(self: &Fluvio, topic: &str) -> Result<Box<TopicProducerPool>>;

        /// Creates a producer for the specified topic with custom configuration
        fn topic_producer_with_config(self: &Fluvio, topic: &str, config: &TopicProducerConfigBuilder) -> Result<Box<TopicProducerPool>>;

        /// Creates a continuous stream for the consumer starting from the given offset index (0=Beginning, -1=End)
        fn consumer_stream(self: &Fluvio, topic: &str, partition: u32, offset_index: i64) -> Result<Box<FluvioStream>>;




        /// Creates a new topic producer configuration builder
        #[Self = "TopicProducerConfigBuilder"]
        fn create() -> Box<TopicProducerConfigBuilder>;

        /// Sets the maximum batch size in bytes for the producer
        fn batch_size(self: &mut TopicProducerConfigBuilder, size: usize) -> Box<TopicProducerConfigBuilder>;

        /// Sets the linger time in milliseconds for the producer
        fn linger(self: &mut TopicProducerConfigBuilder, linger: u64) -> Box<TopicProducerConfigBuilder>;



        /// Creates a new consumer configuration builder
        #[Self = "ConsumerConfigBuilder"]
        fn create() -> Box<ConsumerConfigBuilder>;
        /// Sets the maximum bytes to fetch per request
        fn max_bytes(self: &mut ConsumerConfigBuilder, max: i32) -> Box<ConsumerConfigBuilder>;
        /// Disables continuous fetching
        fn disable_continuous(self: &mut ConsumerConfigBuilder, val: bool) -> Box<ConsumerConfigBuilder>;



        /// Creates a new Fluvio cluster configuration with the specified endpoint
        #[Self = "FluvioConfig"]
        fn create(addr: &str) -> Box<FluvioConfig>;

        /// Loads the Fluvio configuration from the default profile path
        #[Self = "FluvioConfig"]
        fn load() -> Result<Box<FluvioConfig>>;

        /// Sets the endpoint for the cluster configuration
        fn set_endpoint(self: &mut FluvioConfig, endpoint: &str);

        /// Sets the client identifier for the cluster configuration
        fn set_client_id(self: &mut FluvioConfig, client_id: &str);

        fn disable_tls(self: &mut FluvioConfig);

        fn set_anonymous_tls(self: &mut FluvioConfig);

        fn set_inline_tls(self: &mut FluvioConfig, domain: &str, key: &str, cert: &str, ca_cert: &str);

        fn set_tls_file_paths(self: &mut FluvioConfig, domain: &str, key_path: &str, cert_path: &str, ca_cert_path: &str);




        /// Sends a key-value record to the topic asynchronously
        fn send(self: &TopicProducerPool, key: &[u8], value: &[u8]) -> Result<Box<ProduceOutput>>;

        /// Flushes the producer batches
        fn flush(self: &TopicProducerPool) -> Result<()>;



        /// Blocks and waits for the producer record confirmation
        fn wait(self: &mut ProduceOutput) -> Result<Box<RecordMetadata>>;



        /// Retrieves the next record from the stream blocks until available
        fn next(self: &mut FluvioStream) -> Result<Box<Record>>;



        /// Retrieves the payload value byte array from a fetched record
        fn value(self: &Record) -> Vec<u8>;
        /// Retrieves the key byte array from a fetched record
        fn key(self: &Record) -> Vec<u8>;
        /// Retrieves the literal offset index of the fetched record
        fn offset(self: &Record) -> i64;



        /// Connects to the Fluvio Administrative controller
        #[Self = "FluvioAdmin"]
        fn connect() -> Result<Box<FluvioAdmin>>;
        /// Connects to the Fluvio Administrative controller with explicit config
        #[Self = "FluvioAdmin"]
        fn connect_with_config(config: &FluvioConfig) -> Result<Box<FluvioAdmin>>;
        /// Lists all topics
        fn list_topics(self: &FluvioAdmin) -> Result<Vec<String>>;
        /// Checks if a topic exists
        fn  topic_exists(self: &FluvioAdmin, topic: &str) -> bool;
        /// Dispatches a command to create a new topic
        fn create_topic(self: &FluvioAdmin, topic: &str, partitions: i32, replicas: i32) -> Result<()>;
        /// Dispatches a command to violently delete a topic
        fn delete_topic(self: &FluvioAdmin, topic: &str) -> Result<()>;
    }
}

// Re-export all functions so cxx::bridge can find them in the root crate module
pub use admin::*;
pub use client::*;
pub use config::*;
pub use consumer::*;
pub use produce_output::*;
pub use producer::*;
