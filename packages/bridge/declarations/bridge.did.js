export const idlFactory = ({ IDL }) => {
  const InitArgs = IDL.Record({ 'eth_min_confirmations' : IDL.Nat64 });
  const EthPoolLiquidityPositionDto = IDL.Record({
    'last_claimed_fee_per_token' : IDL.Text,
    'amount' : IDL.Text,
  });
  const HttpError = IDL.Record({
    'code' : IDL.Nat16,
    'message' : IDL.Text,
    'details' : IDL.Opt(IDL.Text),
  });
  const Result = IDL.Variant({
    'Ok' : EthPoolLiquidityPositionDto,
    'Err' : HttpError,
  });
  const User = IDL.Record({ 'eth_address' : IDL.Vec(IDL.Nat8) });
  const Result_1 = IDL.Variant({ 'Ok' : User, 'Err' : HttpError });
  return IDL.Service({
    'eth_pool_create_position' : IDL.Func([IDL.Text], [Result], ['query']),
    'user_create' : IDL.Func([], [Result_1], []),
    'user_register_eth_address' : IDL.Func([], [Result_1], []),
  });
};
export const init = ({ IDL }) => {
  const InitArgs = IDL.Record({ 'eth_min_confirmations' : IDL.Nat64 });
  return [InitArgs];
};
