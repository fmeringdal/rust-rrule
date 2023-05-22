install-wasm-pack:
	curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

build-wasm-nodejs:
	wasm-pack build --release --target nodejs --features "wasm"

test-wasm-on-nodejs:
	node examples/wasm/nodejs/app.js

build-wasm-web:
	wasm-pack build --release --target web --features "wasm"

test-wasm-on-web-browser:
	npx http-server /examples/wasm/web/index.html

pack:
	wasm-pack pack pkg