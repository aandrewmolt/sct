use std::str::FromStr;
use solana_sdk::signature::{Keypair, Signer};
use solana_sdk::pubkey::Pubkey;
use anyhow::{Result, anyhow};
use solana_client::rpc_client::RpcClient;
use colored::*;
use bs58;

pub struct Wallet {
    pub keypair: Keypair,
    pub public_key: Pubkey,
}

impl Wallet {
    pub fn new(private_key_b58: String) -> Result<Self> {
        let private_key_bytes = bs58::decode(&private_key_b58)
            .into_vec()
            .map_err(|_| anyhow!("Invalid base58 private key"))?;

        let keypair = Keypair::from_bytes(&private_key_bytes)
            .map_err(|_| anyhow!("Invalid private key bytes"))?;

        Ok(Wallet {
            public_key: keypair.pubkey(),
            keypair,
        })
    }

    #[allow(dead_code)]
    pub fn get_bal(rpc_endpoint: String, target_wallet: &str) {
        let rpc = RpcClient::new(rpc_endpoint);
        let pubkey_str = target_wallet;
       
        let balance = rpc
             .get_account(&Pubkey::from_str(pubkey_str).unwrap())
             .unwrap().lamports;
       
        let sol_balance = balance as f64 / 1e9;
        println!("Sol balance of Target Wallet {} is {} SOL", 
            pubkey_str.bright_green(), 
            format!("{:.4}", sol_balance).bright_yellow()
        );
    }
}