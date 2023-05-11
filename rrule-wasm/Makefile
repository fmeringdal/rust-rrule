install-wasm-pack:
	curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

build-web-wasm:
	wasm-pack build --target web

test-web-wasm-on-browser:
	npx http-server

run-tests:
	wasm-pack test --node