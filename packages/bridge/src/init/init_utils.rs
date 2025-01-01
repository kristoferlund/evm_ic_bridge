use super::InitArgs;
use ic_cdk::trap;

pub fn validate_init_args(args: &InitArgs) {
    if args.ecdsa_key_id.is_empty() {
        trap("The field ecdsa_key_id is required");
    }
    if args.siwe_provider_canister.is_empty() {
        trap("The field siwe_provider_canister is required");
    }
    if args.evm_rpc_url.is_empty() {
        trap("The field evm_rpc_url is required");
    }
}
