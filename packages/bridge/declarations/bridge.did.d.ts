import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface EthPoolLiquidityPositionDto {
  'last_claimed_fee_per_token' : string,
  'timestamp' : bigint,
  'tx_hash' : string,
  'amount' : string,
}
export type Event = { 'Init' : InitArgs } |
  { 'RegisterEthAddress' : [Principal, Uint8Array | number[]] } |
  { 'CreateUser' : Principal } |
  { 'PostUpgrade' : InitArgs } |
  {
    'EThPoolCreatePosition' : [Principal, string, Uint8Array | number[], bigint]
  };
export interface HttpError {
  'code' : number,
  'message' : string,
  'details' : [] | [string],
}
export interface InitArgs {
  'ecdsa_key_id' : string,
  'evm_rpc_url' : string,
  'siwe_provider_canister' : string,
  'eth_min_confirmations' : bigint,
}
export type Result = { 'Ok' : string } |
  { 'Err' : HttpError };
export type Result_1 = { 'Ok' : EthPoolLiquidityPositionDto } |
  { 'Err' : HttpError };
export type Result_2 = { 'Ok' : Array<Event> } |
  { 'Err' : HttpError };
export type Result_3 = { 'Ok' : UserDto } |
  { 'Err' : HttpError };
export interface UserDto {
  'principal' : Principal,
  'eth_address' : [] | [string],
}
export interface _SERVICE {
  'eth_pool_address' : ActorMethod<[], Result>,
  'eth_pool_create_position' : ActorMethod<[string], Result_1>,
  'event_log' : ActorMethod<[], Result_2>,
  'user_create' : ActorMethod<[], Result_3>,
  'user_register_eth_address' : ActorMethod<[], Result_3>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
