composed.wasm: deps/onion/service.wasm deps/onion/virt.wasm deps/onion/wrapper.wasm
	wac encode -d onion:onion=wit/onion.wit composition.wac -o composed.wasm

deps/onion/%.wasm: target/wasm32-unknown-unknown/release/%.wasm
	@mkdir -p "$(@D)"
	wasm-tools component new $< -o $@

target/wasm32-unknown-unknown/release/%.wasm: %/Cargo.toml %/src/lib.rs
	@mkdir -p "$(@D)"
	cargo build --release --target wasm32-unknown-unknown --manifest-path=$<

.PHONY: test
test: composed.wasm host/Cargo.toml host/src/main.rs
	cargo run --release --manifest-path host/Cargo.toml

.PHONY: clean
clean:
	rm -rf deps/onion composed.wasm
