FROM scratch
COPY ./spin.toml ./spin.toml
COPY ./target/wasm32-wasi/release/home_sync.wasm ./target/wasm32-wasi/release/home_sync.wasm
