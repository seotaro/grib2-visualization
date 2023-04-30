install:
	cd react && yarn
	cd rust && curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

build-for-wasm:
	cd rust && wasm-pack build --target web
	mkdir -p react/src/wasm
	cp rust/pkg/rust.js rust/pkg/rust_bg.wasm react/src/wasm/

run:
	cd react && yarn start

