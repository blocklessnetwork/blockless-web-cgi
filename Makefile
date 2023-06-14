all: build

ALL_FILES = $(shell find src -name "*.rs")

build: target/php-cgi.wasm

target/php-cgi.wasm: $(ALL_FILES)
	cargo build --target wasm32-wasi --release
	mv target/wasm32-wasi/release/php-cgi.wasm target