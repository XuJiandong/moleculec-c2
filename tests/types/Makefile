

all: run

run: build/types
	build/types

build/types: types.c types-builder.c types-api2.h  ../../include/molecule2_reader.h
	$(CC) types.c types-builder.c -I../../include -I. -o build/types

clean:
	rm -f build/*
