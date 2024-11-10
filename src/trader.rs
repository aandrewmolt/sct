use solana_client::rpc_client::RpcClient;
use solana_sdk::transaction::Transaction;
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;
use anyhow::Result;
use crate::types::TradeDetails;
use crate::wallet::Wallet;
use crate::config::Config;
use log::info;
use std::str::FromStr;

pub struct Trader {
    rpc_client: RpcClient,
    wallet: Wallet,
}

impl Trader {
    pub fn new(config: &Config) -> Result<Self> {
        let rpc_client = RpcClient::new(config.rpc_endpoint.clone());
        let wallet = Wallet::new(config.private_key.clone())?;
        
        Ok(Trader {
            rpc_client,
            wallet,
        })
    }
    
    pub fn execute_trade(&self, trade: TradeDetails) -> Result<()> {
        match trade.trade_type {
            crate::types::TradeType::Swap => self.swap_tokens(),
            crate::types::TradeType::AddLiquidity => self.add_liquidity(trade),
            crate::types::TradeType::RemoveLiquidity => self.remove_liquidity(trade),
        }
    }
    
    fn swap_tokens(&self) -> Result<()> {
        let instruction = Instruction {
            program_id: Pubkey::from_str("675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8")?,
            accounts: vec![], 
            data: vec![], 
        };
        
        let recent_blockhash = self.rpc_client.get_latest_blockhash()?;
        
        let transaction = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&self.wallet.public_key),
            &[&self.wallet.keypair],
            recent_blockhash,
        );
        
        let signature = self.rpc_client.send_and_confirm_transaction(&transaction)?;
        info!("Swap executed with signature: {}", signature);
        
        Ok(())
    }
    
    fn add_liquidity(&self, _trade: TradeDetails) -> Result<()> {
        Ok(())
    }
    
    fn remove_liquidity(&self, _trade: TradeDetails) -> Result<()> {
        Ok(())
    }
}
