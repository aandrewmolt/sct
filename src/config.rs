use std::env;
use anyhow::{Result, anyhow};
use log::info;

pub struct Config {
    pub rpc_endpoint: String,
    pub ws_endpoint: String,
    pub target_wallet: String,
    pub private_key: String,
    #[allow(dead_code)]
    pub take_profit: f64,
    #[allow(dead_code)]
    pub stop_loss: f64,
    #[allow(dead_code)]
    pub order_size: f64,
    #[allow(dead_code)]
    pub buyin_percentage: f64,
    #[allow(dead_code)]
    pub jito_fee: f64,
    #[allow(dead_code)]
    pub bloxroute_fee: f64,
    #[allow(dead_code)]
    pub commitment_level: String,
    #[allow(dead_code)]
    pub raydium_program_ids: Vec<String>,
}

impl Config {
    pub fn new() -> Result<Self> {
        dotenv::dotenv().ok();
        
        let rpc_endpoint = env::var("RPC_ENDPOINT")
            .unwrap_or_else(|_| "https://api.mainnet-beta.solana.com".to_string());
            
        let ws_endpoint = env::var("WS_ENDPOINT")
            .unwrap_or_else(|_| "wss://api.mainnet-beta.solana.com".to_string());
            
        let target_wallet = env::var("TARGET_WALLET")
            .map_err(|_| anyhow!("TARGET_WALLET not set"))?;
            
        let private_key = env::var("PRIVATE_KEY")
            .map_err(|_| anyhow!("PRIVATE_KEY not set"))?;
            
        let take_profit = env::var("TAKE_PROFIT")
            .unwrap_or_else(|_| "0.1".to_string())
            .parse::<f64>()?;
            
        let stop_loss = env::var("STOP_LOSS")
            .unwrap_or_else(|_| "0.2".to_string())
            .parse::<f64>()?;
            
        let order_size = env::var("ORDER_SIZE")
            .unwrap_or_else(|_| "0.00001".to_string())
            .parse::<f64>()?;
            
        let buyin_percentage = env::var("BUYIN_PERCENTAGE")
            .unwrap_or_else(|_| "0.05".to_string())
            .parse::<f64>()?;
            
        let jito_fee = env::var("JITO_FEE")
            .unwrap_or_else(|_| "0.0001".to_string())
            .parse::<f64>()?;
            
        let bloxroute_fee = env::var("BLOXROUTE_FEE")
            .unwrap_or_else(|_| "0.001".to_string())
            .parse::<f64>()?;
            
        let commitment_level = env::var("COMMITMENT_LEVEL")
            .unwrap_or_else(|_| "finalized".to_string());
        
        let raydium_program_ids = vec![
            env::var("RAYDIUM_AMM_ID")
                .unwrap_or_else(|_| "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8".to_string()),
            env::var("RAYDIUM_POOL_ID")
                .unwrap_or_else(|_| "RVKd61ztZW9sGAucZx5GJ6eWQmJ7f9wMZD3wJftnqGJ".to_string()),
        ];
        
        info!("Monitoring Raydium program IDs: {:?}", raydium_program_ids);
        
        Ok(Config {
            rpc_endpoint,
            ws_endpoint,
            target_wallet,
            private_key,
            take_profit,
            stop_loss,
            order_size,
            buyin_percentage,
            jito_fee,
            bloxroute_fee,
            commitment_level,
            raydium_program_ids,
        })
    }
}
