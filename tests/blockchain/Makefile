
run: build/blockchain
	build/blockchain

build/blockchain: blockchain.c blockchain-api.h blockchain-api2.h blockchain-builder.h ../../include/molecule2_reader.h
	$(CC) blockchain.c -I../../include -I. -o build/blockchain

clean:
	rm -f build/blockchain

