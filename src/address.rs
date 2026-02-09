use crate::error::WalletError;
use kaspa_addresses::{Address, Prefix, Version};
use secp256k1::PublicKey;

pub fn generate_address(public_key: &PublicKey, network: &str) -> Result<String, WalletError> {
    let prefix = match network.trim() {
        "mainnet" => Prefix::Mainnet,
        "testnet-10" | "testnet10" | "testnet" => Prefix::Testnet,
        "testnet-11" | "testnet11" => Prefix::Testnet,
        "simnet" => Prefix::Simnet,
        _ => {
            return Err(WalletError::Network(format!(
                "Unknown network: {}",
                network
            )))
        }
    };

    let pubkey_bytes = public_key.serialize();
    let xonly_pubkey = &pubkey_bytes[1..];

    let address = Address::new(prefix, Version::PubKey, xonly_pubkey);
    Ok(address.to_string())
}

pub fn validate_address(address: &str) -> Result<bool, WalletError> {
    let _ = Address::try_from(address).map_err(|_| WalletError::InvalidAddressFormat)?;
    Ok(true)
}
