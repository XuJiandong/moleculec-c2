
SHELL=/bin/bash -o pipefail

MOLC    := moleculec
MOLC_VERSION := 0.7.3

RUST_SRC := $(wildcard src/*.rs)

all: sample blockchain types blockchain_rust

sample: tests/sample/sample-api.h tests/sample/sample-api2.h
	make -C tests/sample

tests/sample/sample-api2.h: mol/sample.json $(RUST_SRC)
	cargo run --bin moleculec-c2 -- --input mol/sample.json | clang-format -style=Google > tests/sample/sample-api2.h

mol/sample.json: mol/sample.mol
	moleculec --language - --schema-file mol/sample.mol --format json > mol/sample.json

tests/sample/sample-api.h: mol/sample.mol
	moleculec --language c --schema-file mol/sample.mol > tests/sample/sample-api.h

blockchain: tests/blockchain/blockchain-api.h tests/blockchain/blockchain-api2.h
	make -C tests/blockchain

tests/blockchain/blockchain-api2.h: mol/blockchain.json $(RUST_SRC)
	cargo run --bin moleculec-c2 -- --input mol/blockchain.json | clang-format -style=Google > tests/blockchain/blockchain-api2.h

tests/blockchain_rust/src/blockchain.rs: mol/blockchain.json $(RUST_SRC)
	cargo run --bin moleculec-c2 -- --rust --input $< | rustfmt > $@

tests/blockchain_rust/src/import.rs: mol/import.json $(RUST_SRC)
	cargo run --bin moleculec-c2 -- --rust --input $< | rustfmt > $@

tests/blockchain_rust/src/sample.rs: mol/sample.json $(RUST_SRC)
	cargo run --bin moleculec-c2 -- --rust --input $< | rustfmt > $@

tests/blockchain_rust/src/types.rs: mol/types.json $(RUST_SRC)
	cargo run --bin moleculec-c2 -- --rust --input $< | rustfmt > $@

blockchain_rust: tests/blockchain_rust/src/blockchain.rs tests/blockchain_rust/src/sample.rs tests/blockchain_rust/src/types.rs tests/blockchain_rust/src/import.rs
	cd tests/blockchain_rust && cargo test

mol/blockchain.json: mol/blockchain.mol
	moleculec --language - --schema-file mol/blockchain.mol --format json > mol/blockchain.json

mol/import.json: mol/import.mol
	moleculec --language - --schema-file $< --format json > $@

tests/blockchain/blockchain-api.h: mol/blockchain.mol
	moleculec --language c --schema-file mol/blockchain.mol > tests/blockchain/blockchain-api.h

mol/types.json: mol/types.mol
	moleculec --language - --schema-file mol/types.mol --format json > mol/types.json

tests/types/types-api2.h: mol/types.json $(RUST_SRC)
	cargo run --bin moleculec-c2 -- --input mol/types.json | clang-format -style=Google > tests/types/types-api2.h

types: mol/types.json tests/types/types-api2.h
	make -C tests/types

copy-files:
	cp tests/blockchain/blockchain-api2.h ~/projects/ckb-production-scripts-xudt/deps/ckb-c-stdlib-simulator-only2/molecule
	cp include/molecule2_reader.h ~/projects/ckb-production-scripts-xudt/deps/ckb-c-stdlib-simulator-only2/molecule

clippy:
	cargo clippy

fmt:
	clang-format -i -style=Google $(wildcard include/*.h)
	git diff --exit-code $(wildcard include/*.h)

ci:
	make clean
	make all
	cargo test
	
install-tools:
	cargo install --path moleculec-c2
	cargo install --force --version "${MOLC_VERSION}" "${MOLC}"

clean:
	rm -f tests/sample/sample-api2.h
	rm -f tests/sample/sample-api.h
	rm -f tests/blockchain/blockchain-api2.h
	rm -f tests/blockchain/blockchain-api.h
	rm -f tests/types/types-api2.h
	rm -f mol/*.json
	rm -f tests/blockchain_rust/src/blockchain.rs
	rm -f tests/blockchain_rust/src/sample.rs
	rm -f tests/blockchain_rust/src/types.rs
	rm -f tests/blockchain_rust/src/import.rs
	make -C tests/sample clean
	make -C tests/blockchain clean
	make -C tests/types clean

