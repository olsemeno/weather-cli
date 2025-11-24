use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProviderError {
    #[error("Address is required")]
    AddressRequired(String),
    #[error("API key is required: {0}")]
    APIKeyRequired(String),
    #[error("API error: {0}")]
    APIError(String),
}