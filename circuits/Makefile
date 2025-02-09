# Makefile
.PHONY: wasm

wasm:
	GOOS=js GOARCH=wasm go build -o circuit.wasm main.go circuit.go
	cp "$(shell go env GOROOT)/misc/wasm/wasm_exec.js" .