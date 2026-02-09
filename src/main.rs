mod address;
mod error;
mod rpc;

use crate::error::WalletError;
use crate::rpc::RpcClient;
use clap::{Parser, Subcommand};
use secp256k1::{PublicKey, Secp256k1, SecretKey};
use std::fs;
use std::io::{self, Write};

const DEFAULT_RPC_URL: &str = "https://api-tn10.kaspa.org";

#[derive(Parser)]
#[command(name = "kasparustwallet")]
#[command(about = "A Kaspa cryptocurrency wallet CLI", long_about = None)]
#[command(version = "0.2.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Create {
        #[arg(short, long, default_value = "testnet-10")]
        network: String,
        #[arg(short, long)]
        output: Option<String>,
    },
    Info {
        #[arg(short, long)]
        private_key: String,
        #[arg(short, long, default_value = "testnet-10")]
        network: String,
    },
    Address {
        #[arg(short, long)]
        private_key: String,
        #[arg(short, long, default_value = "testnet-10")]
        network: String,
    },
    Balance {
        #[arg(short, long)]
        address: String,
        #[arg(long)]
        rpc: Option<String>,
    },
    ValidateAddress {
        #[arg(short, long)]
        address: String,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    if let Err(e) = run_cli(cli).await {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

async fn run_cli(cli: Cli) -> Result<(), WalletError> {
    match cli.command {
        Commands::Create { network, output } => create_new_wallet(&network, output),
        Commands::Info {
            private_key,
            network,
        } => show_wallet_info(&private_key, &network),
        Commands::Address {
            private_key,
            network,
        } => generate_address(&private_key, &network),
        Commands::Balance { address, rpc } => get_balance(&address, rpc.as_deref()).await,
        Commands::ValidateAddress { address } => validate_address(&address),
    }
}

fn create_new_wallet(network: &str, output: Option<String>) -> Result<(), WalletError> {
    let mut rng = rand::rngs::OsRng;
    let mut secret_bytes = [0u8; 32];
    rand::RngCore::fill_bytes(&mut rng, &mut secret_bytes);
    let secret_key = SecretKey::from_slice(&secret_bytes).expect("Failed to create secret key");
    let secp = Secp256k1::new();
    let public_key = PublicKey::from_secret_key(&secp, &secret_key);

    let address = address::generate_address(&public_key, network)?;

    let private_key_hex = hex::encode(secret_key.secret_bytes());
    let public_key_hex = hex::encode(public_key.serialize());

    let wallet_info = format!(
        "Network: {}\n\
         Private Key: {}\n\
         Public Key: {}\n\
         Address: {}\n",
        network,
        private_key_hex,
        public_key_hex,
        address
    );

    if let Some(output_path) = output {
        fs::write(&output_path, wallet_info)?;
        println!("Wallet created and saved to {}", output_path);
    } else {
        println!("New Wallet Created:");
        println!("==================");
        print!("{}", wallet_info);

        print!("Save this information securely. Press Enter to continue...");
        io::stdout().flush()?;
        let _ = io::stdin().read_line(&mut String::new());
    }

    Ok(())
}

fn show_wallet_info(private_key: &str, network: &str) -> Result<(), WalletError> {
    let secret_key_bytes = hex::decode(private_key)?;
    let secret_key = SecretKey::from_slice(&secret_key_bytes)?;

    let secp = Secp256k1::new();
    let public_key = PublicKey::from_secret_key(&secp, &secret_key);

    let address = address::generate_address(&public_key, network)?;

    println!("Wallet Information:");
    println!("==================");
    println!("Network: {}", network);
    println!("Private Key: {}", private_key);
    println!("Public Key: {}", hex::encode(public_key.serialize()));
    println!("Address: {}", address);

    Ok(())
}

fn generate_address(private_key: &str, network: &str) -> Result<(), WalletError> {
    let secret_key_bytes = hex::decode(private_key)?;
    let secret_key = SecretKey::from_slice(&secret_key_bytes)?;

    let secp = Secp256k1::new();
    let public_key = PublicKey::from_secret_key(&secp, &secret_key);

    let address = address::generate_address(&public_key, network)?;

    println!("Generated Address:");
    println!("==================");
    println!("{}", address);

    Ok(())
}

async fn get_balance(address: &str, rpc_url: Option<&str>) -> Result<(), WalletError> {
    let rpc = rpc_url.unwrap_or(DEFAULT_RPC_URL);
    let client = RpcClient::new(Some(rpc));

    match client.get_balance_by_address(address).await {
        Ok(response) => {
            println!("Balance for {}:", address);
            println!("==================");
            println!("Balance: {} sompi", response.balance);
            println!("KAS: {:.8}", response.balance as f64 / 100_000_000.0);
        }
        Err(e) => {
            return Err(WalletError::Network(format!("Failed to get balance: {}", e)));
        }
    }

    Ok(())
}

fn validate_address(address: &str) -> Result<(), WalletError> {
    let is_valid = address::validate_address(address)?;

    println!("Address Validation:");
    println!("==================");
    println!("Address: {}", address);
    println!("Valid: {}", is_valid);

    Ok(())
}
