use solana_sdk::{
    pubkey::Pubkey,
    system_instruction,
    transaction::Transaction,
    instruction::Instruction,
    signature::Keypair,
    native_token::LAMPORTS_PER_SOL,
};
use solana_client::rpc_client::RpcClient;
use anyhow::Result;
use crate::types::{TradeType, TradeDetails};

#[allow(dead_code)]
pub fn wrap_sol(
    client: &RpcClient,
    wallet_pubkey: &Pubkey,
    wallet_keypair: &Keypair,
    amount_in_sol: f64,
) -> Result<Pubkey> {
    let lamports = (amount_in_sol * LAMPORTS_PER_SOL as f64) as u64;
    
    // Convert native_mint::ID to Pubkey using its bytes
    let wsol_mint = Pubkey::new_from_array(spl_token::native_mint::ID.to_bytes());
    
    let transfer_instruction = system_instruction::transfer(
        wallet_pubkey,
        &wsol_mint,
        lamports
    );
    
    // Get latest blockhash
    let blockhash = client.get_latest_blockhash()?;
    
    let transaction = Transaction::new_signed_with_payer(
        &[transfer_instruction],
        Some(wallet_pubkey),
        &[wallet_keypair],
        blockhash,
    );
    
    client.send_and_confirm_transaction(&transaction)?;
    
    Ok(wallet_pubkey.clone())
}

#[allow(dead_code)]
pub fn unwrap_sol(
    _client: &RpcClient,
    _wsol_account: &Pubkey,
    _wallet_pubkey: &Pubkey,
    _wallet_keypair: &solana_sdk::signature::Keypair,
) -> Result<()> {
    Ok(())
}

#[allow(dead_code)]
pub fn is_raydium_program(program_id: &Pubkey, raydium_program_ids: &[String]) -> bool {
    raydium_program_ids.iter().any(|id| program_id.to_string() == *id)
}

#[allow(dead_code)]
pub fn parse_trade(instruction: &Instruction, raydium_program_ids: &[String]) -> Result<Option<TradeDetails>> {
    // Check if the instruction is from Raydium
    if !raydium_program_ids.contains(&instruction.program_id.to_string()) {
        return Ok(None);
    }
    
    // Implement parsing logic based on Raydium's instruction data
    // This is highly dependent on the specific instruction layout
    
    // Placeholder example:
    // Assume the instruction data contains the trade type, pool ID, tokens, and amounts
    // You need to decode the instruction data according to Raydium's schema
    
    // Example:
    /*
    let trade_type = TradeType::Swap; // Extracted from instruction data
    let pool_id = Pubkey::new_unique(); // Extracted from instruction data
    let input_token = Pubkey::new_unique(); // Extracted from instruction data
    let output_token = Pubkey::new_unique(); // Extracted from instruction data
    let input_amount = 1000000; // Extracted from instruction data
    let output_amount = 2000000; // Extracted from instruction data
    */
    
    // Replace the above with actual parsing logic
    Ok(Some(TradeDetails {
        pool_id: Pubkey::new_unique(),
        input_token: Pubkey::new_unique(),
        output_token: Pubkey::new_unique(),
        input_amount: 1_000_000,
        output_amount: 2_000_000,
        trade_type: TradeType::Swap,
    }))
}


// #[cfg(test)]
// mod tests {
//     use super::*;
//     use solana_sdk::instruction::Instruction;

//     #[test]
//     fn test_parse_trade_swap() {
//         let instruction = Instruction {
//             program_id: Pubkey::from_str("EhhTKr2YLwn5YwfbFeAA1rz35YyYLM5NhefrPBbkE5um").unwrap(),
//             accounts: vec![],
//             data: vec![],
//         };
//         let raydium_program_ids = vec!["EhhTKr2YLwn5YwfbFeAA1rz35YyYLM5NhefrPBbkE5um".to_string()];
//         let result = parse_trade(&instruction, &raydium_program_ids).unwrap();
//         assert!(result.is_some());
//         let trade = result.unwrap();
//         assert_eq!(trade.trade_type, TradeType::Swap);
//     }
// }
