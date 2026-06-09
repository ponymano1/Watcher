use crate::error::AppError;

pub struct AlloyClient {
    pub rpc_url: String,
}

impl AlloyClient {
    pub fn new(rpc_url: String) -> Self {
        Self { rpc_url }
    }

    pub async fn connect(&self) -> Result<(), AppError> {
        Ok(())
    }
}
