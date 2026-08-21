@echo off

echo Compiling for wasm

cargo build --release --target wasm32-unknown-unknown
echo Generating Bindings

wasm-bindgen --out-dir ./web ./target/wasm32-unknown-unknown/release/dungeon_game.wasm

echo Done!