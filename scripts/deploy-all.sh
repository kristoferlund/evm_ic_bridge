dfx canister create internet_identity
dfx canister create evm_rpc
dfx canister create bridge
dfx canister create frontend
dfx canister create ic_siwe_provider
dfx canister create icrc1_ledger
dfx canister create icrc1_index

# Use the currently active identity as the owner for the token
export OWNER=$(dfx identity get-principal)

# Deploy token ledger for local ckSepoliaETH
dfx deploy icrc1_ledger --argument '
  (variant {
    Init = record {
      token_name = "Chain key Sepolia Ethereum";
      token_symbol = "ckSepoliaETH";
      decimals = opt 18;
      minting_account = record {
        owner = principal "'${OWNER}'";
      };
      initial_balances = vec {
        record {
          record {
            owner = principal "'${OWNER}'";
          };
          100_000_000_000;
        };
      };
      metadata = vec {};
      transfer_fee = 10;
      archive_options = record {
        trigger_threshold = 2000;
        num_blocks_to_archive = 1000;
        controller_id = principal "'${OWNER}'";
      }
    }
  })
'

# Deploy token index canister for local ckSepoliaETH
dfx deploy icrc1_index --argument '
  record {
   ledger_id = (principal "apia6-jaaaa-aaaar-qabma-cai");
  }
'

dfx deploy ic_siwe_provider --argument $'(
    record {
        domain = "localhost";
        uri = "http://localhost:5173";
        salt = "mysecretsalt123";
        chain_id = opt 1;
        scheme = opt "http";
        statement = opt "Login to the app";
        sign_in_expires_in = opt 300000000000;    
        session_expires_in = opt 604800000000000;
        targets = opt vec {
            "'$(dfx canister id ic_siwe_provider)'"; 
            "'$(dfx canister id bridge)'"; 
        };
    }
)'

dfx deploy internet_identity

dfx deploy evm_rpc

dfx deploy bridge --with-cycles 10t --argument $'(
    record {
      ecdsa_key_id = "dfx_test_key"; 
      siwe_provider_canister =  "'$(dfx canister id ic_siwe_provider)'";
      evm_rpc_url = "https://ic-alloy-evm-rpc-proxy.kristofer-977.workers.dev/eth-sepolia";
      eth_min_confirmations = 12;
  }
)'

dfx deploy frontend

dfx generate
