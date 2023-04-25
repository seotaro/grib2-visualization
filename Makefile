initialize:
	cd react && yarn

build:
	cd rust && wasm-pack build --target web
	cp -r rust/pkg/ react/src/wasm/
	cp -r rust/pkg/ react/public/wasm/

run:
	cd react && yarn start

