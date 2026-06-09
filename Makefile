.PHONY: release
release:
	cargo build --release && ./target/release/lolang

.PHONY: coverage
coverage:
	cargo tarpaulin --out html


# This is optional or using for first time 
.PHONY: install-all
install-all:
	make install-cargo && \
	make install-cargo-dep

# first time, no cargo
.PHONY: install-cargo
install-cargo:
	curl https://sh.rustup.rs -sSf | sh

##### Additional cargo packages #####
.PHONY: install-cargo-dep
install-cargo-dep:
	cargo install cargo-tarpaulin