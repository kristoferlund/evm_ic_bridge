export const idlFactory = ({ IDL }) => {
  const InitArgs = IDL.Record({
    'ecdsa_key_id' : IDL.Text,
    'evm_rpc_url' : IDL.Text,
    'siwe_provider_canister' : IDL.Text,
    'eth_min_confirmations' : IDL.Nat64,
  });
  const HttpError = IDL.Record({
    'code' : IDL.Nat16,
    'message' : IDL.Text,
    'details' : IDL.Opt(IDL.Text),
  });
  const Result = IDL.Variant({ 'Ok' : IDL.Text, 'Err' : HttpError });
  const EthPoolLiquidityPositionDto = IDL.Record({
    'last_claimed_fee_per_token' : IDL.Text,
    'timestamp' : IDL.Nat64,
    'tx_hash' : IDL.Text,
    'amount' : IDL.Text,
  });
  const Result_1 = IDL.Variant({
    'Ok' : EthPoolLiquidityPositionDto,
    'Err' : HttpError,
  });
  const Event = IDL.Variant({
    'Init' : InitArgs,
    'RegisterEthAddress' : IDL.Tuple(IDL.Principal, IDL.Vec(IDL.Nat8)),
    'CreateUser' : IDL.Principal,
    'PostUpgrade' : InitArgs,
    'EThPoolCreatePosition' : IDL.Tuple(
      IDL.Principal,
      IDL.Text,
      IDL.Vec(IDL.Nat8),
      IDL.Nat64,
    ),
  });
  const Result_2 = IDL.Variant({ 'Ok' : IDL.Vec(Event), 'Err' : HttpError });
  const UserDto = IDL.Record({
    'principal' : IDL.Principal,
    'eth_address' : IDL.Opt(IDL.Text),
  });
  const Result_3 = IDL.Variant({ 'Ok' : UserDto, 'Err' : HttpError });
  return IDL.Service({
    'eth_pool_address' : IDL.Func([], [Result], ['query']),
    'eth_pool_create_position' : IDL.Func([IDL.Text], [Result_1], []),
    'event_log' : IDL.Func([], [Result_2], ['query']),
    'user_create' : IDL.Func([], [Result_3], []),
    'user_register_eth_address' : IDL.Func([], [Result_3], []),
  });
};
export const init = ({ IDL }) => {
  const InitArgs = IDL.Record({
    'ecdsa_key_id' : IDL.Text,
    'evm_rpc_url' : IDL.Text,
    'siwe_provider_canister' : IDL.Text,
    'eth_min_confirmations' : IDL.Nat64,
  });
  return [InitArgs];
};
