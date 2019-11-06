# Rust Calculator

A simple experiment showing a Rust library compiled to WASM and
used in a simple web page.

## Prerequisites

To build this you will need to have the `wasm32-unknown-unknown` target installed 
and the `wasm-bindgen-cli` which can be acquired via Cargo.

```
> rustup target add wasm32-unknown-unknown
> cargo install wasm-bindgen-cli
```

## Building

Windows:

```
> ./build.ps1
```

Linux:

```
> ./build.sh
```

## Running

To run the application you will need to host the content in the `www` folder
in a web server. For this you could (for example) use 
[Simple HTTP Server](https://crates.io/crates/simple-http-server) that you can get
via Cargo.

```
> cd www
> cargo install simple-http-server
> simple-http-server --index
```