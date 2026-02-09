use thiserror::Error;

#[derive(Error, Debug)]
pub enum WalletError {
    #[error("Key generation error: {0}")]
    KeyGeneration(String),

    #[error("Network error: {0}")]
    Network(String),

    #[error("Invalid parameters: {0}")]
    InvalidParameters(String),

    #[error("Invalid address format")]
    InvalidAddressFormat,

    #[error("I/O error: {0}")]
    Io(String),
}

impl From<std::io::Error> for WalletError {
    fn from(error: std::io::Error) -> Self {
        WalletError::Io(error.to_string())
    }
}

impl From<hex::FromHexError> for WalletError {
    fn from(error: hex::FromHexError) -> Self {
        WalletError::InvalidParameters(format!("Hex decode error: {}", error))
    }
}

impl From<String> for WalletError {
    fn from(error: String) -> Self {
        WalletError::InvalidParameters(error)
    }
}

impl From<secp256k1::Error> for WalletError {
    fn from(error: secp256k1::Error) -> Self {
        WalletError::KeyGeneration(format!("Secp256k1 error: {}", error))
    }
}
