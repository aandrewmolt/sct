mod config;
mod wallet;
mod listener;
mod trader;
mod utils;
mod types;

use tokio::sync::mpsc;
use anyhow::Result;
use config::Config;
use listener::Listener;
use trader::Trader;
use types::TradeDetails;
use log::{info, error};
use env_logger;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::signal;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
    
    info!("Starting Solana copy trader...");
    
    // Create shutdown signal
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    
    // Handle Ctrl+C
    tokio::spawn(async move {
        if let Ok(_) = signal::ctrl_c().await {
            info!("Shutting down...");
            r.store(false, Ordering::SeqCst);
        }
    });

    match Config::new() {
        Ok(config) => {
            info!("Configuration loaded successfully");
            info!("Using RPC endpoint: {}", config.rpc_endpoint);
            info!("Using WebSocket endpoint: {}", config.ws_endpoint);
            info!("Monitoring wallet: {}", config.target_wallet);

            let trader = Trader::new(&config)?;
            let listener = Listener::new(&config)?;
            let (tx, mut rx) = mpsc::channel::<TradeDetails>(100);
            
            // Start listening for wallet activity
            let running_listener = running.clone();
            let listen_handle = tokio::spawn(async move {
                while running_listener.load(Ordering::SeqCst) {
                    match listener.start_listening(tx.clone()).await {
                        Ok(_) => {
                            // Connection closed normally
                            continue;
                        }
                        Err(e) => {
                            error!("Connection error: {:?}, reconnecting...", e);
                        }
                    }
                }
            });

            // info!("Fetching wallet information...");
            // Wallet::get_bal(config.rpc_endpoint, &config.target_wallet);
            
            // Process any detected trades
            while let Some(trade) = rx.recv().await {
                info!("Detected new transaction:");
                info!("  Type: {:?}", trade.trade_type);
                info!("  Input Amount: {} lamports", trade.input_amount);
                info!("  Output Amount: {} lamports", trade.output_amount);
                info!("  Pool: {}", trade.pool_id);
                
                match trader.execute_trade(trade) {
                    Ok(_) => info!("Successfully copied trade"),
                    Err(e) => error!("Failed to copy trade: {:?}", e),
                }
            }

            
            listen_handle.await?;
        }
        Err(e) => {
            error!("Configuration error: {:?}", e);
            return Err(e);
        }
    }
    
    Ok(())
}