// This is an experimental feature to generate Rust binding from Candid.
// You may want to manually adjust some of the types.
#![allow(dead_code, unused_imports)]
use candid::{self, CandidType, Deserialize, Principal, Encode, Decode};
use ic_cdk::api::call::CallResult as Result;

#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct ChangeArchiveOptions {
  pub num_blocks_to_archive: Option<u64>,
  pub max_transactions_per_response: Option<u64>,
  pub trigger_threshold: Option<u64>,
  pub more_controller_ids: Option<Vec<Principal>>,
  pub max_message_size_bytes: Option<u64>,
  pub cycles_for_archive_creation: Option<u64>,
  pub node_max_memory_size_bytes: Option<u64>,
  pub controller_id: Option<Principal>,
}

#[derive(Debug, CandidType, Deserialize, Clone)]
pub enum MetadataValue {
  Int(candid::Int),
  Nat(candid::Nat),
  Blob(serde_bytes::ByteBuf),
  Text(String),
}

pub type Subaccount = serde_bytes::ByteBuf;
#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct Account { pub owner: Principal, pub subaccount: Option<Subaccount> }

#[derive(Debug, CandidType, Deserialize, Clone)]
pub enum ChangeFeeCollector { SetTo(Account), Unset }

#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct FeatureFlags { pub icrc2: bool }

#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct UpgradeArgs {
  pub change_archive_options: Option<ChangeArchiveOptions>,
  pub token_symbol: Option<String>,
  pub transfer_fee: Option<candid::Nat>,
  pub metadata: Option<Vec<(String,MetadataValue,)>>,
  pub accounts_overflow_trim_quantity: Option<u64>,
  pub change_fee_collector: Option<ChangeFeeCollector>,
  pub max_memo_length: Option<u16>,
  pub token_name: Option<String>,
  pub feature_flags: Option<FeatureFlags>,
}

#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct InitArgsArchiveOptions {
  pub num_blocks_to_archive: u64,
  pub max_transactions_per_response: Option<u64>,
  pub trigger_threshold: u64,
  pub more_controller_ids: Option<Vec<Principal>>,
  pub max_message_size_bytes: Option<u64>,
  pub cycles_for_archive_creation: Option<u64>,
  pub node_max_memory_size_bytes: Option<u64>,
  pub controller_id: Principal,
}

#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct InitArgs {
  pub decimals: Option<u8>,
  pub token_symbol: String,
  pub transfer_fee: candid::Nat,
  pub metadata: Vec<(String,MetadataValue,)>,
  pub minting_account: Account,
  pub initial_balances: Vec<(Account,candid::Nat,)>,
  pub maximum_number_of_accounts: Option<u64>,
  pub accounts_overflow_trim_quantity: Option<u64>,
  pub fee_collector_account: Option<Account>,
  pub archive_options: InitArgsArchiveOptions,
  pub max_memo_length: Option<u16>,
  pub token_name: String,
  pub feature_flags: Option<FeatureFlags>,
}

#[derive(Debug, CandidType, Deserialize, Clone)]
pub enum LedgerArg { Upgrade(Option<UpgradeArgs>), Init(InitArgs) }

pub type BlockIndex = candid::Nat;
#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct ArchiveInfo {
  pub block_range_end: BlockIndex,
  pub canister_id: Principal,
  pub block_range_start: BlockIndex,
}

#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct GetBlocksArgs { pub start: BlockIndex, pub length: candid::Nat }

pub type Map = Vec<(String,Box<Value>,)>;
#[derive(Debug, CandidType, Deserialize, Clone)]
pub enum Value {
  Int(candid::Int),
  Map(Map),
  Nat(candid::Nat),
  Nat64(u64),
  Blob(serde_bytes::ByteBuf),
  Text(String),
  Array(Vec<Box<Value>>),
}

pub type Block = Box<Value>;
#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct BlockRange { pub blocks: Vec<Block> }

candid::define_function!(pub QueryBlockArchiveFn : (GetBlocksArgs) -> (
    BlockRange,
  ) query);
#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct GetBlocksResponseArchivedBlocksItem {
  pub callback: QueryBlockArchiveFn,
  pub start: BlockIndex,
  pub length: candid::Nat,
}

#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct GetBlocksResponse {
  pub certificate: Option<serde_bytes::ByteBuf>,
  pub first_index: BlockIndex,
  pub blocks: Vec<Block>,
  pub chain_length: u64,
  pub archived_blocks: Vec<GetBlocksResponseArchivedBlocksItem>,
}

#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct DataCertificate {
  pub certificate: Option<serde_bytes::ByteBuf>,
  pub hash_tree: serde_bytes::ByteBuf,
}

pub type TxIndex = candid::Nat;
#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct GetTransactionsRequest {
  pub start: TxIndex,
  pub length: candid::Nat,
}

pub type Timestamp = u64;
#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct Burn {
  pub from: Account,
  pub memo: Option<serde_bytes::ByteBuf>,
  pub created_at_time: Option<Timestamp>,
  pub amount: candid::Nat,
  pub spender: Option<Account>,
}

#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct Mint {
  pub to: Account,
  pub memo: Option<serde_bytes::ByteBuf>,
  pub created_at_time: Option<Timestamp>,
  pub amount: candid::Nat,
}

#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct Approve {
  pub fee: Option<candid::Nat>,
  pub from: Account,
  pub memo: Option<serde_bytes::ByteBuf>,
  pub created_at_time: Option<Timestamp>,
  pub amount: candid::Nat,
  pub expected_allowance: Option<candid::Nat>,
  pub expires_at: Option<Timestamp>,
  pub spender: Account,
}

#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct Transfer {
  pub to: Account,
  pub fee: Option<candid::Nat>,
  pub from: Account,
  pub memo: Option<serde_bytes::ByteBuf>,
  pub created_at_time: Option<Timestamp>,
  pub amount: candid::Nat,
  pub spender: Option<Account>,
}

#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct Transaction {
  pub burn: Option<Burn>,
  pub kind: String,
  pub mint: Option<Mint>,
  pub approve: Option<Approve>,
  pub timestamp: Timestamp,
  pub transfer: Option<Transfer>,
}

#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct TransactionRange { pub transactions: Vec<Transaction> }

candid::define_function!(pub QueryArchiveFn : (GetTransactionsRequest) -> (
    TransactionRange,
  ) query);
#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct GetTransactionsResponseArchivedTransactionsItem {
  pub callback: QueryArchiveFn,
  pub start: TxIndex,
  pub length: candid::Nat,
}

#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct GetTransactionsResponse {
  pub first_index: TxIndex,
  pub log_length: candid::Nat,
  pub transactions: Vec<Transaction>,
  pub archived_transactions: Vec<
    GetTransactionsResponseArchivedTransactionsItem
  >,
}

#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct Icrc10SupportedStandardsRetItem { pub url: String, pub name: String }

pub type Tokens = candid::Nat;
#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct StandardRecord { pub url: String, pub name: String }

#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct TransferArg {
  pub to: Account,
  pub fee: Option<Tokens>,
  pub memo: Option<serde_bytes::ByteBuf>,
  pub from_subaccount: Option<Subaccount>,
  pub created_at_time: Option<Timestamp>,
  pub amount: Tokens,
}

#[derive(Debug, CandidType, Deserialize, Clone)]
pub enum TransferError {
  GenericError{ message: String, error_code: candid::Nat },
  TemporarilyUnavailable,
  BadBurn{ min_burn_amount: Tokens },
  Duplicate{ duplicate_of: BlockIndex },
  BadFee{ expected_fee: Tokens },
  CreatedInFuture{ ledger_time: Timestamp },
  TooOld,
  InsufficientFunds{ balance: Tokens },
}

#[derive(Debug, CandidType, Deserialize, Clone)]
pub enum TransferResult { Ok(BlockIndex), Err(TransferError) }

#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct Icrc21ConsentMessageMetadata {
  pub utc_offset_minutes: Option<i16>,
  pub language: String,
}

#[derive(Debug, CandidType, Deserialize, Clone)]
pub enum Icrc21ConsentMessageSpecDeviceSpecInner {
  GenericDisplay,
  LineDisplay{ characters_per_line: u16, lines_per_page: u16 },
}

#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct Icrc21ConsentMessageSpec {
  pub metadata: Icrc21ConsentMessageMetadata,
  pub device_spec: Option<Icrc21ConsentMessageSpecDeviceSpecInner>,
}

#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct Icrc21ConsentMessageRequest {
  pub arg: serde_bytes::ByteBuf,
  pub method: String,
  pub user_preferences: Icrc21ConsentMessageSpec,
}

#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct Icrc21ConsentMessageLineDisplayMessagePagesItem {
  pub lines: Vec<String>,
}

#[derive(Debug, CandidType, Deserialize, Clone)]
pub enum Icrc21ConsentMessage {
  LineDisplayMessage{
    pages: Vec<Icrc21ConsentMessageLineDisplayMessagePagesItem>,
  },
  GenericDisplayMessage(String),
}

#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct Icrc21ConsentInfo {
  pub metadata: Icrc21ConsentMessageMetadata,
  pub consent_message: Icrc21ConsentMessage,
}

#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct Icrc21ErrorInfo { pub description: String }

#[derive(Debug, CandidType, Deserialize, Clone)]
pub enum Icrc21Error {
  GenericError{ description: String, error_code: candid::Nat },
  InsufficientPayment(Icrc21ErrorInfo),
  UnsupportedCanisterCall(Icrc21ErrorInfo),
  ConsentMessageUnavailable(Icrc21ErrorInfo),
}

#[derive(Debug, CandidType, Deserialize, Clone)]
pub enum Icrc21ConsentMessageResponse {
  Ok(Icrc21ConsentInfo),
  Err(Icrc21Error),
}

#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct AllowanceArgs { pub account: Account, pub spender: Account }

#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct Allowance {
  pub allowance: candid::Nat,
  pub expires_at: Option<Timestamp>,
}

#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct ApproveArgs {
  pub fee: Option<candid::Nat>,
  pub memo: Option<serde_bytes::ByteBuf>,
  pub from_subaccount: Option<serde_bytes::ByteBuf>,
  pub created_at_time: Option<Timestamp>,
  pub amount: candid::Nat,
  pub expected_allowance: Option<candid::Nat>,
  pub expires_at: Option<Timestamp>,
  pub spender: Account,
}

#[derive(Debug, CandidType, Deserialize, Clone)]
pub enum ApproveError {
  GenericError{ message: String, error_code: candid::Nat },
  TemporarilyUnavailable,
  Duplicate{ duplicate_of: BlockIndex },
  BadFee{ expected_fee: candid::Nat },
  AllowanceChanged{ current_allowance: candid::Nat },
  CreatedInFuture{ ledger_time: Timestamp },
  TooOld,
  Expired{ ledger_time: Timestamp },
  InsufficientFunds{ balance: candid::Nat },
}

#[derive(Debug, CandidType, Deserialize, Clone)]
pub enum ApproveResult { Ok(BlockIndex), Err(ApproveError) }

#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct TransferFromArgs {
  pub to: Account,
  pub fee: Option<Tokens>,
  pub spender_subaccount: Option<Subaccount>,
  pub from: Account,
  pub memo: Option<serde_bytes::ByteBuf>,
  pub created_at_time: Option<Timestamp>,
  pub amount: Tokens,
}

#[derive(Debug, CandidType, Deserialize, Clone)]
pub enum TransferFromError {
  GenericError{ message: String, error_code: candid::Nat },
  TemporarilyUnavailable,
  InsufficientAllowance{ allowance: Tokens },
  BadBurn{ min_burn_amount: Tokens },
  Duplicate{ duplicate_of: BlockIndex },
  BadFee{ expected_fee: Tokens },
  CreatedInFuture{ ledger_time: Timestamp },
  TooOld,
  InsufficientFunds{ balance: Tokens },
}

#[derive(Debug, CandidType, Deserialize, Clone)]
pub enum TransferFromResult { Ok(BlockIndex), Err(TransferFromError) }

#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct GetArchivesArgs { pub from: Option<Principal> }

#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct GetArchivesResultItem {
  pub end: candid::Nat,
  pub canister_id: Principal,
  pub start: candid::Nat,
}

pub type GetArchivesResult = Vec<GetArchivesResultItem>;
#[derive(Debug, CandidType, Deserialize, Clone)]
pub enum Icrc3Value {
  Int(candid::Int),
  Map(Vec<(String,Box<Icrc3Value>,)>),
  Nat(candid::Nat),
  Blob(serde_bytes::ByteBuf),
  Text(String),
  Array(Vec<Box<Icrc3Value>>),
}

#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct GetBlocksResultBlocksItem {
  pub id: candid::Nat,
  pub block: Box<Icrc3Value>,
}

candid::define_function!(pub GetBlocksResultArchivedBlocksItemCallback : (
    Vec<GetBlocksArgs>,
  ) -> (GetBlocksResult) query);
#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct GetBlocksResultArchivedBlocksItem {
  pub args: Vec<GetBlocksArgs>,
  pub callback: GetBlocksResultArchivedBlocksItemCallback,
}

#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct GetBlocksResult {
  pub log_length: candid::Nat,
  pub blocks: Vec<GetBlocksResultBlocksItem>,
  pub archived_blocks: Vec<GetBlocksResultArchivedBlocksItem>,
}

#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct Icrc3DataCertificate {
  pub certificate: serde_bytes::ByteBuf,
  pub hash_tree: serde_bytes::ByteBuf,
}

#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct Icrc3SupportedBlockTypesRetItem {
  pub url: String,
  pub block_type: String,
}

pub struct Icrc1Ledger(pub Principal);
impl Icrc1Ledger {
  pub async fn archives(&self) -> Result<(Vec<ArchiveInfo>,)> {
    ic_cdk::call(self.0, "archives", ()).await
  }
  pub async fn get_blocks(&self, arg0: GetBlocksArgs) -> Result<
    (GetBlocksResponse,)
  > { ic_cdk::call(self.0, "get_blocks", (arg0,)).await }
  pub async fn get_data_certificate(&self) -> Result<(DataCertificate,)> {
    ic_cdk::call(self.0, "get_data_certificate", ()).await
  }
  pub async fn get_transactions(&self, arg0: GetTransactionsRequest) -> Result<
    (GetTransactionsResponse,)
  > { ic_cdk::call(self.0, "get_transactions", (arg0,)).await }
  pub async fn icrc_10_supported_standards(&self) -> Result<
    (Vec<Icrc10SupportedStandardsRetItem>,)
  > { ic_cdk::call(self.0, "icrc10_supported_standards", ()).await }
  pub async fn icrc_1_balance_of(&self, arg0: Account) -> Result<(Tokens,)> {
    ic_cdk::call(self.0, "icrc1_balance_of", (arg0,)).await
  }
  pub async fn icrc_1_decimals(&self) -> Result<(u8,)> {
    ic_cdk::call(self.0, "icrc1_decimals", ()).await
  }
  pub async fn icrc_1_fee(&self) -> Result<(Tokens,)> {
    ic_cdk::call(self.0, "icrc1_fee", ()).await
  }
  pub async fn icrc_1_metadata(&self) -> Result<
    (Vec<(String,MetadataValue,)>,)
  > { ic_cdk::call(self.0, "icrc1_metadata", ()).await }
  pub async fn icrc_1_minting_account(&self) -> Result<(Option<Account>,)> {
    ic_cdk::call(self.0, "icrc1_minting_account", ()).await
  }
  pub async fn icrc_1_name(&self) -> Result<(String,)> {
    ic_cdk::call(self.0, "icrc1_name", ()).await
  }
  pub async fn icrc_1_supported_standards(&self) -> Result<
    (Vec<StandardRecord>,)
  > { ic_cdk::call(self.0, "icrc1_supported_standards", ()).await }
  pub async fn icrc_1_symbol(&self) -> Result<(String,)> {
    ic_cdk::call(self.0, "icrc1_symbol", ()).await
  }
  pub async fn icrc_1_total_supply(&self) -> Result<(Tokens,)> {
    ic_cdk::call(self.0, "icrc1_total_supply", ()).await
  }
  pub async fn icrc_1_transfer(&self, arg0: TransferArg) -> Result<
    (TransferResult,)
  > { ic_cdk::call(self.0, "icrc1_transfer", (arg0,)).await }
  pub async fn icrc_21_canister_call_consent_message(
    &self,
    arg0: Icrc21ConsentMessageRequest,
  ) -> Result<(Icrc21ConsentMessageResponse,)> {
    ic_cdk::call(self.0, "icrc21_canister_call_consent_message", (arg0,)).await
  }
  pub async fn icrc_2_allowance(&self, arg0: AllowanceArgs) -> Result<
    (Allowance,)
  > { ic_cdk::call(self.0, "icrc2_allowance", (arg0,)).await }
  pub async fn icrc_2_approve(&self, arg0: ApproveArgs) -> Result<
    (ApproveResult,)
  > { ic_cdk::call(self.0, "icrc2_approve", (arg0,)).await }
  pub async fn icrc_2_transfer_from(&self, arg0: TransferFromArgs) -> Result<
    (TransferFromResult,)
  > { ic_cdk::call(self.0, "icrc2_transfer_from", (arg0,)).await }
  pub async fn icrc_3_get_archives(&self, arg0: GetArchivesArgs) -> Result<
    (GetArchivesResult,)
  > { ic_cdk::call(self.0, "icrc3_get_archives", (arg0,)).await }
  pub async fn icrc_3_get_blocks(&self, arg0: Vec<GetBlocksArgs>) -> Result<
    (GetBlocksResult,)
  > { ic_cdk::call(self.0, "icrc3_get_blocks", (arg0,)).await }
  pub async fn icrc_3_get_tip_certificate(&self) -> Result<
    (Option<Icrc3DataCertificate>,)
  > { ic_cdk::call(self.0, "icrc3_get_tip_certificate", ()).await }
  pub async fn icrc_3_supported_block_types(&self) -> Result<
    (Vec<Icrc3SupportedBlockTypesRetItem>,)
  > { ic_cdk::call(self.0, "icrc3_supported_block_types", ()).await }
  pub async fn is_ledger_ready(&self) -> Result<(bool,)> {
    ic_cdk::call(self.0, "is_ledger_ready", ()).await
  }
}
pub const CANISTER_ID : Principal = Principal::from_slice(&[0, 0, 0, 0, 2, 48, 0, 88, 1, 1]); // apia6-jaaaa-aaaar-qabma-cai
pub const icrc1_ledger : Icrc1Ledger = Icrc1Ledger(CANISTER_ID);