initialize:
	cd react && yarn

build:
	cd rust && wasm-pack build --target web
	mkdir -p react/src/wasm
	cp rust/pkg/rust.js rust/pkg/rust_bg.wasm react/src/wasm/

run:
	cd react && yarn start

