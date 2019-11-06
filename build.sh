echo "Compiling..."
cargo build --target wasm32-unknown-unknown --release
if [ $? != 0 ]; then 
    echo "An error occured during compilation" 
    exit 1
fi

echo "Generating JS bindings..."
wasm-bindgen --target web --no-typescript --out-dir ./www target/wasm32-unknown-unknown/release/rustcalc.wasm
if [ $? != 0 ]; then 
    echo "An error occured when running wasm-bindgen" 
    exit 1
fi

echo "Success!"