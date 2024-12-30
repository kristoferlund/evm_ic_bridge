import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface EthPoolLiquidityPositionDto {
  'last_claimed_fee_per_token' : string,
  'amount' : string,
}
export interface HttpError {
  'code' : number,
  'message' : string,
  'details' : [] | [string],
}
export interface InitArgs { 'eth_min_confirmations' : bigint }
export type Result = { 'Ok' : EthPoolLiquidityPositionDto } |
  { 'Err' : HttpError };
export type Result_1 = { 'Ok' : User } |
  { 'Err' : HttpError };
export interface User { 'eth_address' : Uint8Array | number[] }
export interface _SERVICE {
  'eth_pool_create_position' : ActorMethod<[string], Result>,
  'user_create' : ActorMethod<[], Result_1>,
  'user_register_eth_address' : ActorMethod<[], Result_1>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
