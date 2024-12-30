dfx canister create internet_identity
dfx canister create evm_rpc
dfx canister create bridge
dfx canister create frontend
dfx canister create ic_siwe_provider

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

cargo build -p bridge --release --target wasm32-unknown-unknown
cd ./target/wasm32-unknown-unknown/release
candid-extractor bridge.wasm >../../../packages/bridge/bridge.did
ic-wasm bridge.wasm -o bridge.wasm metadata candid:service -f ../../../packages/bridge/bridge.did -v public
gzip -c bridge.wasm >bridge.wasm.gz

dfx deploy bridge --with-cycles 10t

dfx deploy frontend

dfx generate
