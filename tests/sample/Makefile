
run: build/sample build/decl-only-sample
	build/sample
	build/decl-only-sample

build/sample: sample.c sample-api.h sample-api2.h sample-builder.h ../../include/molecule2_reader.h
	$(CC) sample.c -I../../include -I. -o build/sample

build/decl-only-sample: decl-only-sample.c decl-only-impl.c sample-api2.h sample-builder.h ../../include/molecule2_reader.h
	$(CC) decl-only-sample.c decl-only-impl.c -I../../include -I. -o build/decl-only-sample

clean:
	rm -f build/*
