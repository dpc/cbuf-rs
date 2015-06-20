PKG_NAME=cbuf
DEFAULT_TARGET=build

default: $(DEFAULT_TARGET)

.PHONY: run test build doc clean release rrun bench
run test build doc clean:
	cargo $@

simple:
	cargo run

release:
	cargo build --release

rrun:
	cargo run --release

bench:
	cargo bench

.PHONY: docview
docview: doc
	xdg-open target/doc/$(PKG_NAME)/index.html
