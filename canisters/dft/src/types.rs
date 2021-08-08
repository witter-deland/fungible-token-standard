use crate::utils;
use candid::CandidType;
use ic_types::{CanisterId, PrincipalId};
use ledger_canister::account_identifier::AccountIdentifier;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;
use std::string::String;

pub type TransactionId = u128;
pub type ExtendData = HashMap<String, String>;
pub type Balances = HashMap<TokenHolder, u128>;
pub type Allowances = HashMap<TokenHolder, HashMap<TokenHolder, u128>>;
#[derive(CandidType, Debug, Deserialize)]
pub struct TokenPayload {
    pub initialized: bool,
    pub owner: PrincipalId,
    pub fee_cashier: TokenHolder,
    pub meta: MetaData,
    pub extend: Vec<(String, String)>,
    pub logo: Vec<u8>,
    pub balances: Vec<(TokenHolder, u128)>,
    pub allowances: Vec<(TokenHolder, Vec<(TokenHolder, u128)>)>,
    pub total_fee: u128,
    pub tx_id_cursor: u128,
    pub storage_canister_id: CanisterId,
}
// Rate decimals = 8
// transferFee = amount * rate / 1000000
#[derive(CandidType, Debug, Clone, Deserialize)]
pub enum Fee {
    Fixed(u128),
    RateWithLowestLimit(u128, u8),
}

impl fmt::Display for Fee {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        let s = match &self {
            Fee::Fixed(_fee) => _fee.to_string(),
            Fee::RateWithLowestLimit(_fee, rate) => format!("{{lowest:{0},rate:{1}}}", _fee, rate),
        };
        write!(f, "{}", s)
    }
}

#[derive(CandidType, Debug, Deserialize)]
pub struct MetaData {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: u128,
    pub fee: Fee,
}

#[derive(CandidType, Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum TokenHolder {
    Account(AccountIdentifier),
    Principal(PrincipalId),
    Canister(CanisterId),
}

#[derive(CandidType, Debug, Clone, Deserialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct KeyValuePair {
    pub k: String,
    pub v: String,
}

impl FromStr for TokenHolder {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pid = s.parse::<PrincipalId>();
        match pid {
            Ok(_principal) => match _principal {
                _principal if utils::is_canister(&_principal) => {
                    let cid = CanisterId::new(_principal).unwrap();
                    Ok(TokenHolder::Canister(cid))
                }
                _principal if utils::is_user_principal(&_principal) => {
                    Ok(TokenHolder::Principal(_principal))
                }
                _ => Err("invalid token holder format".to_string()),
            },
            _ => {
                let account_identity = s.parse::<AccountIdentifier>();
                match account_identity {
                    Ok(_ai) => Ok(TokenHolder::Account(_ai)),
                    _ => Err("invalid token holder format".to_string()),
                }
            }
        }
    }
}
impl fmt::Display for TokenHolder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        let s = match &self {
            TokenHolder::Account(_ai) => _ai.to_string(),
            TokenHolder::Principal(_pid) => _pid.to_string(),
            TokenHolder::Canister(_cid) => _cid.to_string(),
        };
        write!(f, "{}", s)
    }
}

pub type TransferFrom = TokenHolder;
pub type TokenReceiver = TokenHolder;

#[derive(CandidType, Debug, Clone, Deserialize, Serialize)]
pub struct CallData {
    pub method: String,
    pub args: Vec<u8>,
}

#[derive(CandidType, Debug, Clone, Deserialize, Serialize)]
pub enum TransferResult {
    //transfer succeed, but call failed & notify failed
    Ok(TransactionId, Option<Vec<String>>),
    Err(String),
}

#[derive(CandidType, Debug, Clone, Deserialize, Serialize)]
pub enum BurnResult {
    Ok,
    Err(String),
}

// Invalid data: Invalid IDL blob by candid 0.6.21
#[derive(CandidType, Debug, Clone, Deserialize, Serialize)]
pub enum ApproveResult {
    Ok(Option<String>),
    Err(String),
}

#[derive(CandidType, Debug, Clone, Deserialize, Serialize)]
pub enum TxRecord {
    // caller, owner, spender, value, fee, timestamp
    Approve(PrincipalId, TokenHolder, TokenReceiver, u128, u128, u64),
    // caller, from, to, value, fee, timestamp
    Transfer(PrincipalId, TokenHolder, TokenReceiver, u128, u128, u64),
    // caller, from, value, timestamp
    Burn(PrincipalId, TokenHolder, u128, u64),
}