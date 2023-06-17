all: release

ALL_FILES = $(shell find src -name "*.rs")

release: target/blockless-web-cgi.wasm
debug: target/blockless-web-cgi-debug.wasm

target/blockless-web-cgi.wasm: $(ALL_FILES)
	cargo build --target wasm32-wasi --release
	mv target/wasm32-wasi/release/blockless-web-cgi.wasm target

target/blockless-web-cgi-debug.wasm: $(ALL_FILES)
	cargo build --target wasm32-wasi 
	mv target/wasm32-wasi/debug/blockless-web-cgi.wasm target/blockless-web-cgi-debug.wasm
