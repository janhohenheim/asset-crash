# Embedded Asset Crash on Windows Wasm Builds

Run the following:

```sh
cargo build --target wasm32-unknown-unknown --no-default-features
wasm-bindgen --out-name asset_crash --out-dir wasm/ --target web .\target\wasm32-unknown-unknown\debug\asset-crash.wasm
cd wasm/
http-server
```

Go to `http://127.0.0.1:7878/index.html` and open the console. You will see the following on Windows only:

```
Failed to find src_prefix "src" in "src\\screen\\splash\\mod.rs"
```

Note that this panic does not occur on Linux.
