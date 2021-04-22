MOLC    := moleculec
MOLC_VERSION := 0.7.0

RUST_SRC := $(wildcard src/*.rs)

all: sample blockchain types

sample: tests/sample/sample-api.h tests/sample/sample-api2.h
	make -C tests/sample

tests/sample/sample-api2.h: mol/sample.json $(RUST_SRC)
	cargo run -- --input mol/sample.json | clang-format -style=Google > tests/sample/sample-api2.h

mol/sample.json: mol/sample.mol
	moleculec --language - --schema-file mol/sample.mol --format json > mol/sample.json

tests/sample/sample-api.h: mol/sample.mol
	moleculec --language c --schema-file mol/sample.mol > tests/sample/sample-api.h

blockchain: tests/blockchain/blockchain-api.h tests/blockchain/blockchain-api2.h
	make -C tests/blockchain

tests/blockchain/blockchain-api2.h: mol/blockchain.json $(RUST_SRC)
	cargo run -- --input mol/blockchain.json | clang-format -style=Google > tests/blockchain/blockchain-api2.h

mol/blockchain.json: mol/blockchain.mol
	moleculec --language - --schema-file mol/blockchain.mol --format json > mol/blockchain.json

tests/blockchain/blockchain-api.h: mol/blockchain.mol
	moleculec --language c --schema-file mol/blockchain.mol > tests/blockchain/blockchain-api.h

mol/types.json: mol/types.mol
	moleculec --language - --schema-file mol/types.mol --format json > mol/types.json

tests/types/types-api2.h: mol/types.json $(RUST_SRC)
	cargo run -- --input mol/types.json | clang-format -style=Google > tests/types/types-api2.h

types: mol/types.json tests/types/types-api2.h
	make -C tests/types

copy-files:
	cp tests/blockchain/blockchain-api2.h ~/projects/ckb-production-scripts-xudt/deps/ckb-c-stdlib-simulator-only2/molecule
	cp include/molecule2_reader.h ~/projects/ckb-production-scripts-xudt/deps/ckb-c-stdlib-simulator-only2/molecule

fmt:
	clang-format -i -style=Google $(wildcard include/*.h)
	git diff --exit-code $(wildcard include/*.h)

ci:
	cargo install moleculec --vers ${MOLC_VERSION}
	make clean
	make all
	make fmt

install-tools:
	if [ ! -x "$$(command -v "${MOLC}")" ] \
			|| [ "$$(${MOLC} --version | awk '{ print $$2 }' | tr -d ' ')" != "${MOLC_VERSION}" ]; then \
		cargo install --force --version "${MOLC_VERSION}" "${MOLC}"; \
	fi



clean:
	rm -f tests/sample/sample-api2.h
	rm -f tests/sample/sample-api.h
	rm -f tests/blockchain/blockchain-api2.h
	rm -f tests/blockchain/blockchain-api.h
	rm -f tests/types/types-api2.h
	rm -f mol/*.json
	make -C tests/sample clean
	make -C tests/blockchain clean
	make -C tests/types clean
