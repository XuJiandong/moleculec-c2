

all: sample

sample: mol/sample.json tests/sample/sample-api.h tests/sample/sample-api2.h
	make -C tests/sample

tests/sample/sample-api2.h: mol/sample.json
	cargo run -- --input mol/sample.json | clang-format -style=Google > tests/sample/sample-api2.h

mol/sample.json: mol/sample.mol
	moleculec --language - --schema-file mol/sample.mol --format json > mol/sample.json

tests/sample/sample-api.h: mol/sample.mol
	moleculec --language c --schema-file mol/sample.mol > tests/sample/sample-api.h

clean:
	rm -f tests/sample/sample-api2.h
	rm -f mol/sample.json
	rm -f tests/sample/sample-api.h
	make -C tests/sample clean
