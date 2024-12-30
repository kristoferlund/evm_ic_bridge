cargo build -p bridge --release --target wasm32-unknown-unknown
BUILD_DIR="./target/wasm32-unknown-unknown/release"
candid-extractor "$BUILD_DIR/bridge.wasm" >./packages/bridge/bridge.did
ic-wasm "$BUILD_DIR/bridge.wasm" -o "$BUILD_DIR/bridge.wasm" metadata candid:service -f ./packages/bridge/bridge.did -v public
gzip -c "$BUILD_DIR/bridge.wasm" >"$BUILD_DIR/bridge.wasm.gz"

