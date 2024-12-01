use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UsersWallet{
    pub id:u32,
    pub uid:u32,
    pub coin:Decimal,
    pub dcoin:Decimal,
    pub bonus:Decimal,
    pub score:Option<u32>,
    pub secret_key:Option<String>,
    pub btc_add:Option<String>,
    pub ltc_add:Option<String>,
    pub eth_add:Option<String>,
    pub tron_add:Option<String>,
    pub updated_at:Option<u32>,
    pub created_at:Option<u32>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WalletUId{
    pub uid:u32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WalletInfo{
    pub id:u32,
    pub uid:u32,
    pub coin:Decimal,
    pub dcoin:Decimal,
    pub bonus:Decimal,
    pub score:Option<u32>,
    pub btc_add:Option<String>,
    pub ltc_add:Option<String>,
    pub eth_add:Option<String>,
    pub tron_add:Option<String>
}