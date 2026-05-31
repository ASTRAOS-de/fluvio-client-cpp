use fluvio::{
    ProduceOutput as ProduceOutputNative,
    RecordMetadata as RecordMetadataNative
};
use fluvio_future::task::run_block_on;

pub struct ProduceOutput { pub inner: Option<ProduceOutputNative> }
pub struct RecordMetadata { pub inner: RecordMetadataNative }


impl ProduceOutput{
    pub fn wait(self: &mut Self) -> Result<Box<RecordMetadata>, String> {
        let inner = self.inner.take();
        match inner {
            Some(produce_output) => {
                run_block_on(produce_output.wait())
                    .map(|metadata| Box::new(RecordMetadata { inner: metadata }))
                    .map_err(|e| e.to_string())
            },
            None => Err("ProduceOutput already consumed".to_string())
        }
    }
}

