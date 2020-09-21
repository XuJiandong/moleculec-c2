
gen:
	cargo run -- --input deps/molecule-toolkit/mol/sample.json > deps/molecule-toolkit/generated/sample-api2-generated.h
	clang-format -i -style=Google deps/molecule-toolkit/generated/sample-api2-generated.h
