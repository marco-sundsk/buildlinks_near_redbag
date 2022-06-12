RFLAGS="-C link-arg=-s"

build: contracts/blredbag
	rustup target add wasm32-unknown-unknown
	RUSTFLAGS=$(RFLAGS) cargo build -p blredbag --target wasm32-unknown-unknown --release
	mkdir -p res
	cp target/wasm32-unknown-unknown/release/blredbag.wasm ./res/blredbag.wasm

test: build mock-ft
	RUSTFLAGS=$(RFLAGS) cargo test -p blredbag

release:
	$(call docker_build,_rust_setup.sh)
	mkdir -p res
	cp target/wasm32-unknown-unknown/release/blredbag.wasm res/blredbag_release.wasm

mock-ft: contracts/mock-ft
	rustup target add wasm32-unknown-unknown
	RUSTFLAGS=$(RFLAGS) cargo build -p mock-ft --target wasm32-unknown-unknown --release
	mkdir -p res
	cp target/wasm32-unknown-unknown/release/mock_ft.wasm ./res/mock_ft.wasm

clean:
	cargo clean
	rm -rf res/

define docker_build
	docker build -t my-contract-builder .
	docker run \
		--mount type=bind,source=${PWD},target=/host \
		--cap-add=SYS_PTRACE --security-opt seccomp=unconfined \
		-w /host \
		-e RUSTFLAGS=$(RFLAGS) \
		-i -t my-contract-builder \
		/bin/bash $(1)
endef
