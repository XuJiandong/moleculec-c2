
run: build/sample
	build/sample

build/sample: sample.c sample-api.h sample-api2.h sample-builder.h ../../include/molecule2_reader.h
	$(CC) sample.c -I../../include -I. -o build/sample

clean:
	rm -f build/sample
