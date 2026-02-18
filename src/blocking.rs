use crate::client::KiCadClient;
use crate::error::KiCadError;

#[derive(Clone, Debug)]
pub struct KiCadClientBlocking {
    inner: KiCadClient,
}

impl KiCadClientBlocking {
    pub fn connect() -> Result<Self, KiCadError> {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_time()
            .build()
            .map_err(|err| KiCadError::RuntimeJoin(err.to_string()))?;
        let inner = runtime.block_on(KiCadClient::connect())?;
        Ok(Self { inner })
    }

    pub fn inner(&self) -> &KiCadClient {
        &self.inner
    }
}
