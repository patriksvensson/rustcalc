Write-Host "Compiling..."
&cargo build --target wasm32-unknown-unknown --release
if($LASTEXITCODE -ne 0) {
    Throw "An error occured during compilation"
}

Write-Host "Generating JS bindings..."
&wasm-bindgen --target web --no-typescript --out-dir ./www target/wasm32-unknown-unknown/release/rustcalc.wasm
if($LASTEXITCODE -ne 0) {
    Throw "An error occured when running wasm-bindgen" 
}

Write-Host "Success!"