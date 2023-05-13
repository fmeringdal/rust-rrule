install-wasm-pack:
	curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

build-wasm-nodejs:
	wasm-pack build --target nodejs

test-wasm-on-nodejs:
	node example/nodejs/app.js

build-wasm-web:
	wasm-pack build --target web

test-wasm-on-web-browser:
	cd example/web && npx http-server

run-unit-tests:
	wasm-pack test --node