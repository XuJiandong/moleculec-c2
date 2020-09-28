

all: sample blockchain

sample: mol/sample.json tests/sample/sample-api.h tests/sample/sample-api2.h
	make -C tests/sample

tests/sample/sample-api2.h: mol/sample.json
	cargo run -- --input mol/sample.json | clang-format -style=Google > tests/sample/sample-api2.h

mol/sample.json: mol/sample.mol
	moleculec --language - --schema-file mol/sample.mol --format json > mol/sample.json

tests/sample/sample-api.h: mol/sample.mol
	moleculec --language c --schema-file mol/sample.mol > tests/sample/sample-api.h

blockchain: mol/blockchain.json tests/blockchain/blockchain-api.h tests/blockchain/blockchain-api2.h
	make -C tests/blockchain

tests/blockchain/blockchain-api2.h: mol/blockchain.json
	cargo run -- --input mol/blockchain.json | clang-format -style=Google > tests/blockchain/blockchain-api2.h

mol/blockchain.json: mol/blockchain.mol
	moleculec --language - --schema-file mol/blockchain.mol --format json > mol/blockchain.json

tests/blockchain/blockchain-api.h: mol/blockchain.mol
	moleculec --language c --schema-file mol/blockchain.mol > tests/blockchain/blockchain-api.h

fmt:
	clang-format -i -style=Google $(wildcard include/*.h)
	git diff --exit-code $(wildcard include/*.h)

clean:
	rm -f tests/sample/sample-api2.h
	rm -f tests/sample/sample-api.h
	rm -f tests/blockchain/blockchain-api2.h
	rm -f tests/blockchain/blockchain-api.h
	rm -f mol/*.json
	make -C tests/sample clean
	make -C tests/blockchain clean

