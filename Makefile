all: build

ALL_FILES = $(shell find src -name "*.rs")

build: target/php-cgi.wasm
build-debug: target/php-cgi-debug.wasm

target/php-cgi.wasm: $(ALL_FILES)
	cargo build --target wasm32-wasi --release
	mv target/wasm32-wasi/release/php-cgi.wasm target


target/php-cgi-debug.wasm: $(ALL_FILES)
	cargo build --target wasm32-wasi 
	mv target/wasm32-wasi/debug/php-cgi.wasm target/php-cgi-debug.wasm
