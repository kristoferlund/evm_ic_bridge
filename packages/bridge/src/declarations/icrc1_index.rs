// This is an experimental feature to generate Rust binding from Candid.
// You may want to manually adjust some of the types.
#![allow(dead_code, unused_imports)]
use candid::{self, CandidType, Deserialize, Principal, Encode, Decode};
use ic_cdk::api::call::CallResult as Result;

#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct InitArgs { pub ledger_id: Principal }

pub type TxId = candid::Nat;
#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct Account {
  pub owner: Principal,
  pub subaccount: Option<serde_bytes::ByteBuf>,
}

#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct GetAccountTransactionsArgs {
  pub max_results: candid::Nat,
  pub start: Option<TxId>,
  pub account: Account,
}

#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct Burn {
  pub from: Account,
  pub memo: Option<serde_bytes::ByteBuf>,
  pub created_at_time: Option<u64>,
  pub amount: candid::Nat,
  pub spender: Option<Account>,
}

#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct Mint {
  pub to: Account,
  pub memo: Option<serde_bytes::ByteBuf>,
  pub created_at_time: Option<u64>,
  pub amount: candid::Nat,
}

#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct Approve {
  pub fee: Option<candid::Nat>,
  pub from: Account,
  pub memo: Option<serde_bytes::ByteBuf>,
  pub created_at_time: Option<u64>,
  pub amount: candid::Nat,
  pub expected_allowance: Option<candid::Nat>,
  pub expires_at: Option<u64>,
  pub spender: Account,
}

#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct Transfer {
  pub to: Account,
  pub fee: Option<candid::Nat>,
  pub from: Account,
  pub memo: Option<serde_bytes::ByteBuf>,
  pub created_at_time: Option<u64>,
  pub amount: candid::Nat,
  pub spender: Option<Account>,
}

#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct Transaction {
  pub burn: Option<Burn>,
  pub kind: String,
  pub mint: Option<Mint>,
  pub approve: Option<Approve>,
  pub timestamp: u64,
  pub transfer: Option<Transfer>,
}

#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct TransactionWithId { pub id: TxId, pub transaction: Transaction }

#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct GetTransactions {
  pub transactions: Vec<TransactionWithId>,
  pub oldest_tx_id: Option<TxId>,
}

#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct GetTransactionsErr { pub message: String }

#[derive(Debug, CandidType, Deserialize, Clone)]
pub enum GetTransactionsResult { Ok(GetTransactions), Err(GetTransactionsErr) }

pub type SubAccount = serde_bytes::ByteBuf;
#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct ListSubaccountsArgs {
  pub owner: Principal,
  pub start: Option<SubAccount>,
}

pub struct Icrc1Index(pub Principal);
impl Icrc1Index {
  pub async fn get_account_transactions(
    &self,
    arg0: GetAccountTransactionsArgs,
  ) -> Result<(GetTransactionsResult,)> {
    ic_cdk::call(self.0, "get_account_transactions", (arg0,)).await
  }
  pub async fn ledger_id(&self) -> Result<(Principal,)> {
    ic_cdk::call(self.0, "ledger_id", ()).await
  }
  pub async fn list_subaccounts(&self, arg0: ListSubaccountsArgs) -> Result<
    (Vec<SubAccount>,)
  > { ic_cdk::call(self.0, "list_subaccounts", (arg0,)).await }
}
pub const CANISTER_ID : Principal = Principal::from_slice(&[0, 0, 0, 0, 2, 48, 0, 154, 1, 1]); // sh5u2-cqaaa-aaaar-qacna-cai
pub const icrc1_index : Icrc1Index = Icrc1Index(CANISTER_ID);