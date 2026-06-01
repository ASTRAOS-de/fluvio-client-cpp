use crate::{
    client::Fluvio,
    producer::TopicProducerPool,
    consumer::{FluvioStream, Record},
    produce_output::ProduceOutput,
    config::FluvioConfig
};
use std::os::raw::c_char;
use std::ffi::CStr;

#[repr(C)]
pub struct fluvio_config_t {
    _private: [u8; 0],
}

#[unsafe(no_mangle)]
pub extern "C" fn fluvio_c_connect(out_client: *mut *mut Fluvio) -> i32 {
    match Fluvio::connect() {
        Ok(client) => {
            unsafe { *out_client = Box::into_raw(client); }
            0
        }
        Err(_) => -1,
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fluvio_c_connect_with_config(config: *mut fluvio_config_t, out_client: *mut *mut Fluvio) -> i32 {
    if config.is_null() || out_client.is_null() { return -1; }
    let config_wrapper = &mut *(config as *mut FluvioConfig);
    match Fluvio::connect_with_config(config_wrapper) {
        Ok(client) => {
            unsafe { *out_client = Box::into_raw(client); }
            0
        }
        Err(_) => -1,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn fluvio_c_client_free(client: *mut Fluvio) {
    if !client.is_null() { unsafe { let _ = Box::from_raw(client); } }
}

#[unsafe(no_mangle)]
pub extern "C" fn fluvio_c_create_producer(client: *mut Fluvio, topic: *const c_char, out_producer: *mut *mut TopicProducerPool) -> i32 {
    if client.is_null() || topic.is_null() || out_producer.is_null() { return -1; }
    let topic_str = unsafe { CStr::from_ptr(topic).to_str() }.unwrap_or("");
    match Fluvio::topic_producer(unsafe { &*client }, topic_str) {
        Ok(producer) => { unsafe { *out_producer = Box::into_raw(producer); } 0 }
        Err(_) => -1,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn fluvio_c_producer_send(producer: *mut TopicProducerPool, key: *const u8, key_len: usize, val: *const u8, val_len: usize, out: *mut *mut ProduceOutput) -> i32 {
    if producer.is_null() || (key.is_null() && key_len > 0) || (val.is_null() && val_len > 0) { return -1; }
    let key_slice = if key_len > 0 { unsafe { std::slice::from_raw_parts(key, key_len) } } else { &[] };
    let val_slice = if val_len > 0 { unsafe { std::slice::from_raw_parts(val, val_len) } } else { &[] };
    match TopicProducerPool::send(unsafe { &*producer }, key_slice, val_slice) {
        Ok(o) => { if !out.is_null() { unsafe { *out = Box::into_raw(o); } } 0 }
        Err(_) => -1,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn fluvio_c_produce_output_wait(out: *mut ProduceOutput) -> i32 {
    if out.is_null() { return -1; }
    match ProduceOutput::wait(unsafe { &mut *out }) { Ok(_) => 0, Err(_) => -1 }
}

#[unsafe(no_mangle)]
pub extern "C" fn fluvio_c_producer_flush(producer: *mut TopicProducerPool) -> i32 {
    if producer.is_null() { return -1; }
    match TopicProducerPool::flush(unsafe { &*producer }) { Ok(_) => 0, Err(_) => -1 }
}

#[unsafe(no_mangle)]
pub extern "C" fn fluvio_c_producer_free(producer: *mut TopicProducerPool) {
    if !producer.is_null() { unsafe { let _ = Box::from_raw(producer); } }
}

#[unsafe(no_mangle)]
pub extern "C" fn fluvio_c_produce_output_free(out: *mut ProduceOutput) {
    if !out.is_null() { unsafe { let _ = Box::from_raw(out); } }
}

#[unsafe(no_mangle)]
pub extern "C" fn fluvio_c_config_load(out_config: *mut *mut fluvio_config_t) -> i32 {
    if out_config.is_null() { return -1; }
    match FluvioConfig::load() {
        Ok(config) => { unsafe { *out_config = Box::into_raw(config) as *mut fluvio_config_t; } 0 }
        Err(_) => -1,
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fluvio_c_config_set_endpoint(config: *mut fluvio_config_t, endpoint: *const std::ffi::c_char) {
    if config.is_null() || endpoint.is_null() { return; }
    let config_wrapper = &mut *(config as *mut FluvioConfig);
    let ep_str = std::ffi::CStr::from_ptr(endpoint).to_string_lossy();
    config_wrapper.set_endpoint(&ep_str);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fluvio_c_config_set_client_id(config: *mut fluvio_config_t, client_id: *const std::ffi::c_char) {
    if config.is_null() || client_id.is_null() { return; }
    let config_wrapper = &mut *(config as *mut FluvioConfig);
    let client_id_str = std::ffi::CStr::from_ptr(client_id).to_string_lossy();
    config_wrapper.set_client_id(&client_id_str);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fluvio_c_config_disable_tls(config: *mut fluvio_config_t) {
    if config.is_null() { return; }
    let config_wrapper = &mut *(config as *mut FluvioConfig);
    config_wrapper.disable_tls();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fluvio_c_config_set_anonymous_tls(config: *mut fluvio_config_t) {
    if config.is_null() { return; }
    let config_wrapper = &mut *(config as *mut FluvioConfig);
    config_wrapper.set_anonymous_tls();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fluvio_c_config_set_inline_tls(config: *mut fluvio_config_t, domain: *const std::ffi::c_char, key: *const std::ffi::c_char, cert: *const std::ffi::c_char, ca_cert: *const std::ffi::c_char) {
    if config.is_null() || domain.is_null() || key.is_null() || cert.is_null() || ca_cert.is_null() { return; }
    let config_wrapper = &mut *(config as *mut FluvioConfig);
    config_wrapper.set_inline_tls(
        &std::ffi::CStr::from_ptr(domain).to_string_lossy(),
        &std::ffi::CStr::from_ptr(key).to_string_lossy(),
        &std::ffi::CStr::from_ptr(cert).to_string_lossy(),
        &std::ffi::CStr::from_ptr(ca_cert).to_string_lossy(),
    );
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fluvio_c_config_set_tls_file_paths(config: *mut fluvio_config_t, domain: *const std::ffi::c_char, key_path: *const std::ffi::c_char, cert_path: *const std::ffi::c_char, ca_cert_path: *const std::ffi::c_char) {
    if config.is_null() || domain.is_null() || key_path.is_null() || cert_path.is_null() || ca_cert_path.is_null() { return; }
    let config_wrapper = &mut *(config as *mut FluvioConfig);
    config_wrapper.set_tls_file_paths(
        &std::ffi::CStr::from_ptr(domain).to_string_lossy(),
        &std::ffi::CStr::from_ptr(key_path).to_string_lossy(),
        &std::ffi::CStr::from_ptr(cert_path).to_string_lossy(),
        &std::ffi::CStr::from_ptr(ca_cert_path).to_string_lossy(),
    );
}

#[unsafe(no_mangle)]
pub extern "C" fn fluvio_c_consumer_stream(client: *mut Fluvio, topic: *const c_char, partition: u32, offset_index: i64, out_stream: *mut *mut FluvioStream) -> i32 {
    if client.is_null() || topic.is_null() || out_stream.is_null() { return -1; }
    let topic_str = unsafe { CStr::from_ptr(topic).to_str() }.unwrap_or("");
    match Fluvio::consumer_stream(unsafe { &*client }, topic_str, partition, offset_index) {
        Ok(stream) => { unsafe { *out_stream = Box::into_raw(stream); } 0 }
        Err(_) => -1,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn fluvio_c_stream_next(stream: *mut FluvioStream, out_record: *mut *mut Record) -> i32 {
    if stream.is_null() || out_record.is_null() { return -1; }
    match FluvioStream::next(unsafe { &mut *stream }) {
        Ok(record) => { unsafe { *out_record = Box::into_raw(record); } 0 }
        Err(_) => -1,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn fluvio_c_record_value(record: *mut Record, out_buf: *mut *const u8, out_len: *mut usize) -> i32 {
    if record.is_null() || out_buf.is_null() || out_len.is_null() { return -1; }
    let val = unsafe { &*record }.inner.value();
    unsafe { *out_buf = val.as_ptr(); *out_len = val.len(); }
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn fluvio_c_record_free(rec: *mut Record) {
    if !rec.is_null() { unsafe { let _ = Box::from_raw(rec); } }
}

#[unsafe(no_mangle)]
pub extern "C" fn fluvio_c_stream_free(stream: *mut FluvioStream) {
    if !stream.is_null() { unsafe { let _ = Box::from_raw(stream); } }
}

