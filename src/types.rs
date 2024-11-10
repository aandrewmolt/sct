use solana_sdk::pubkey::Pubkey;

#[derive(Debug, Clone, PartialEq)]
pub enum TradeType {
    Swap,
    #[allow(dead_code)]
    AddLiquidity,
    #[allow(dead_code)]
    RemoveLiquidity,
}

#[derive(Debug, Clone)]
pub struct TradeDetails {
    pub pool_id: Pubkey,
    #[allow(dead_code)]
    pub input_token: Pubkey,
    #[allow(dead_code)]
    pub output_token: Pubkey,
    pub input_amount: u64,
    pub output_amount: u64,
    pub trade_type: TradeType,
}