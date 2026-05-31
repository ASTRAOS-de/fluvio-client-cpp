use std::ops::DerefMut;
use fluvio::FluvioConfig as FluvioConfigNative;
use fluvio::consumer::ConsumerConfigBuilder as ConsumerConfigBuilderNative;
use fluvio::TopicProducerConfigBuilder as TopicProducerConfigBuilderNative;

pub struct ConsumerConfigBuilder { pub inner: ConsumerConfigBuilderNative }
pub struct TopicProducerConfigBuilder { pub inner: TopicProducerConfigBuilderNative }
pub struct FluvioConfig { pub inner: FluvioConfigNative }

impl TopicProducerConfigBuilder {
    pub fn create() -> Box<TopicProducerConfigBuilder> {
        Box::new(TopicProducerConfigBuilder { inner: TopicProducerConfigBuilderNative::default() })
    }
    pub fn batch_size(self: &mut Self, size: usize) -> Box<TopicProducerConfigBuilder> {
        self.inner.batch_size(size);
        Box::new(TopicProducerConfigBuilder { inner: self.inner.clone() })
    }
    pub fn linger(self: &mut Self, linger: u64) -> Box<TopicProducerConfigBuilder> {
        self.inner.linger(std::time::Duration::from_millis(linger));
        Box::new(TopicProducerConfigBuilder { inner: self.inner.clone() })
    }
}

impl ConsumerConfigBuilder {
    pub fn create() -> Box<ConsumerConfigBuilder> {
        Box::new(ConsumerConfigBuilder { inner: ConsumerConfigBuilderNative::default() })
    }
    pub fn max_bytes(self: &mut Self, max: i32) -> Box<ConsumerConfigBuilder> {
        self.inner.max_bytes(max);
        Box::new(ConsumerConfigBuilder { inner: self.inner.clone() })
    }
    pub fn disable_continuous(self: &mut Self, val: bool) -> Box<ConsumerConfigBuilder> {
        self.inner.disable_continuous(val);
        Box::new(ConsumerConfigBuilder { inner: self.inner.clone() })
    }

}

impl FluvioConfig {
    pub fn create(addr: &str) -> Box<FluvioConfig> {
        Box::new(FluvioConfig { inner: FluvioConfigNative::new(addr) })
    }
    pub fn load() -> Result<Box<FluvioConfig>, String> {
        FluvioConfigNative::load().map(|c| Box::new(FluvioConfig { inner: c })).map_err(|e| e.to_string())
    }
    pub fn set_endpoint(self: &mut Self, endpoint: &str) {
        self.inner.endpoint = endpoint.to_string();
    }
    pub fn set_client_id(self: &mut Self, client_id: &str) {
        self.inner.client_id = Some(client_id.to_string());
    }
    pub fn disable_tls(self: &mut Self) {
        self.inner.tls = fluvio::config::TlsPolicy::Disabled;
    }
    pub fn set_anonymous_tls(self: &mut Self) {
        self.inner.tls = fluvio::config::TlsPolicy::Anonymous;
    }
    pub fn set_inline_tls(self: &mut Self, domain: &str, key: &str, cert: &str, ca_cert: &str) {
        self.inner.tls = fluvio::config::TlsPolicy::Verified(fluvio::config::TlsConfig::Inline(fluvio::config::TlsCerts {
            domain: domain.to_string(),
            key: key.to_string(),
            cert: cert.to_string(),
            ca_cert: ca_cert.to_string(),
        }));
    }
    pub fn set_tls_file_paths(self: &mut Self, domain: &str, key_path: &str, cert_path: &str, ca_cert_path: &str) {
        self.inner.tls = fluvio::config::TlsPolicy::Verified(fluvio::config::TlsConfig::Files(fluvio::config::TlsPaths {
            domain: domain.to_string(),
            key: std::path::PathBuf::from(key_path),
            cert: std::path::PathBuf::from(cert_path),
            ca_cert: std::path::PathBuf::from(ca_cert_path),
        }));
    }
}
















