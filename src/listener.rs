use solana_sdk::pubkey::Pubkey;
use serde_json::Value;
use tokio::sync::mpsc;
use anyhow::Result;
use crate::types::{TradeDetails, TradeType};
use crate::config::Config;
use tokio_tungstenite::connect_async;
use url::Url;
use log::{info, error, debug};
use std::str::FromStr;
use futures::StreamExt;
use futures::SinkExt;
use tokio_tungstenite::tungstenite::Message;
use base64::decode;

pub struct Listener {
    ws_endpoint: String,
    target_wallet: Pubkey,
    raydium_program_ids: Vec<String>,
}

impl Listener {
    pub fn new(config: &Config) -> Result<Self> {
        Ok(Listener {
            ws_endpoint: config.ws_endpoint.clone(),
            target_wallet: Pubkey::from_str(&config.target_wallet)?,
            raydium_program_ids: config.raydium_program_ids.clone(),
        })
    }
    
    pub async fn start_listening(&self, tx: mpsc::Sender<TradeDetails>) -> Result<()> {
        info!("Connecting to WebSocket endpoint: {}", self.ws_endpoint);
        let url = Url::parse(&self.ws_endpoint)?;
        
        let (ws_stream, response) = connect_async(url).await?;
        info!("WebSocket connected with status: {}", response.status());
        
        let (mut write, mut read) = ws_stream.split();
        
        // Subscribe to transaction notifications
        let subscribe_msg = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "logsSubscribe",
            "params": [
                {
                    "mentions": [ self.target_wallet.to_string() ]
                },
                {
                    "commitment": "confirmed",
                    "encoding": "jsonParsed"
                }
            ]
        });
        
        info!("Sending subscription request: {}", subscribe_msg);
        write.send(Message::Text(subscribe_msg.to_string())).await?;
        
        match read.next().await {
            Some(Ok(Message::Text(text))) => {
                info!("Subscription response: {}", text);
            }
            _ => error!("Failed to get subscription confirmation"),
        }
        
        info!("Waiting for messages...");
        while let Some(msg) = read.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    info!("Received WebSocket message: {}", text);
                    let v: Value = serde_json::from_str(&text).unwrap_or(Value::Null);
                    
                    if let Some(method) = v.get("method") {
                        info!("Message method: {}", method);
                    }
                    
                    if let Some(params) = v.get("params") {
                        if let Some(result) = params.get("result") {
                            if let Some(logs) = result.get("logs") {
                                info!("Transaction logs: {:?}", logs);
                                
                                // Create a longer-lived Vec for the logs
                                let empty_vec = Vec::new();
                                let logs_array = logs.as_array().unwrap_or(&empty_vec);
                                
                                for log in logs_array {
                                    let log_str = log.as_str().unwrap_or("");
                                    info!("Log entry: {}", log_str);
                                    
                                    // Check for Raydium program invocations
                                    if self.raydium_program_ids.iter().any(|id| log_str.contains(id)) {
                                        info!("Found Raydium transaction!");
                                        
                                        if let Some(signature) = result.get("signature") {
                                            info!("Transaction signature: {}", signature);
                                            
                                            if let Some(trade_details) = self.process_raydium_transaction(result)? {
                                                tx.send(trade_details).await?;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                },
                Ok(Message::Close(frame)) => {
                    error!("WebSocket closed by server: {:?}", frame);
                    break;
                },
                Err(e) => {
                    error!("WebSocket error: {:?}", e);
                    break;
                },
                _ => {
                    debug!("Received non-text message");
                }
            }
        }
        
        info!("WebSocket connection closed, attempting to reconnect...");
        Ok(())
    }

    fn process_raydium_transaction(&self, transaction: &Value) -> Result<Option<TradeDetails>> {
        // Get logs directly from the transaction parameter
        if let Some(logs) = transaction.get("logs").and_then(|l| l.as_array()) {
            for log in logs {
                if let Some(log_str) = log.as_str() {
                    // Look for ray_log entries
                    if log_str.contains("ray_log:") {
                        info!("Found Raydium log entry");
                        let encoded_data = log_str.split("ray_log: ").nth(1).unwrap_or("");
                        let decoded = decode(encoded_data)?;
                        
                        // Raydium swap data structure:
                        // 0-8:   input amount (u64)
                        // 8-16:  minimum output amount (u64)
                        // 16-24: fee (u64)
                        // 24-32: price impact (u64)
                        
                        let input_amount = u64::from_le_bytes(decoded[0..8].try_into()?);
                        let min_output = u64::from_le_bytes(decoded[8..16].try_into()?);
                        let fee = u64::from_le_bytes(decoded[16..24].try_into()?);
                        let price_impact = u64::from_le_bytes(decoded[24..32].try_into()?);
                        
                        info!("Decoded swap details:");
                        info!("  Input amount: {} lamports", input_amount);
                        info!("  Minimum output: {} lamports", min_output);
                        info!("  Fee: {} lamports", fee);
                        info!("  Price Impact: {}", price_impact);
                        
                        // Get transaction signature
                        let signature = transaction
                            .get("signature")
                            .and_then(|s| s.as_str())
                            .unwrap_or("unknown");
                            
                        info!("Transaction signature: {}", signature);
                        
                        return Ok(Some(TradeDetails {
                            pool_id: Pubkey::new_unique(),
                            input_token: Pubkey::new_unique(),
                            output_token: Pubkey::new_unique(),
                            input_amount,
                            output_amount: min_output,
                            trade_type: TradeType::Swap,
                        }));
                    }
                }
            }
        }
        
        Ok(None)
    }
}