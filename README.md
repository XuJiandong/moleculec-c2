# moleculec-c2
Improved C plugin for the molecule serialization system.

### How to use
First, install "moleculec" by "cargo install moleculec".
Then, compile binary (moleculec-c2) by "cargo build --release", and put the binary in PATH.
```bash
$cargo install moleculec
$cargo build --release
$moleculec --language - --schema-file mol/blockchain.mol --format json > mol/blockchain.json
$target/release/moleculec-c2 --input mol/blockchain.json | clang-format -style=Google > tests/blockchain/blockchain-api2.h
```
Note: "clang-format -style=Google" is not needed if you don't care about coding style.

_________________


The following are optimized compared to the old C Reader API:
### Strong type
If we look into the code of old molecule API usage, 
we find that mol_seg_t is everywhere: it's like a weak type in dynamic languages(Python, lua). 
We can't use type system of C compilers to check whether we use the API correctly. 
With new API, we can use the type system help us to reduce possibilities for bugs,  
checking that the code is written in a consistent way, giving hint while coding.
Here is an [example usage of blockchain](https://github.com/XuJiandong/moleculec-c2/blob/bf3e045c94a43d03ab5eae16040d10600227f3b2/tests/blockchain/blockchain.c#L13-L44).
And [browse the generated API for blockchain](https://github.com/XuJiandong/moleculec-c2/blob/master/tests/blockchain/blockchain-api2.h).

### Extra support for known types
From the [Encoding Spec](https://github.com/nervosnetwork/molecule/blob/master/docs/encoding_spec.md), we know that there is no types system in molecule.
For example, we can find the following definitions in [molecule](https://github.com/XuJiandong/molecule-toolkit/blob/master/mol/blockchain.mol):
```text
array Uint32 [byte; 4];
array Uint64 [byte; 8];
```
We now have "version" with type "Uint32". But with old molecule API, the API still returns "uint_8*" instead of "uint32_t".

Now the following type names are reserved for types:
- Uint8, Int8
- Uint16, Int16
- Uint32, Int32
- Uint64, Int64


When they appear in schema file, it is automatically converted to the corresponding types in the generated files.
Here are the mapping list:

| Molecule type  | Type name   | C Type     |
|----------------|-------------|------------|
| byte           |  /          |  uint8_t   |   
| `[byte; 1]`     | int8       |  int8_t    |   
| `[byte; 1]`     | uint8      |  uint8_t   |   
| `[byte; 2]`     | int16      |  int16_t   |   
| `[byte; 2]`     | uint16     |  uint16_t  |   
| `[byte; 4]`     | int32      |  int32_t   |   
| `[byte; 4]`     | uint32     |  uint32_t  |   
| `[byte; 8]`     | int64      |  int64_t   |   
| `[byte; 8]`     | uint64     |  uint64_t  |   
| `[byte; N]`     | /          |  mol2_cursor_t  |   
| `<byte>`        | /          |  mol2_cursor_t  |  
 
The type name is case insensitive, for example, int8, Int8, INT8 are all mapped to int8_t.

### Load memory on demand

mol_seg_t, is the most important data structure in old molecule API:
```C
typedef struct {
    uint8_t                     *ptr;               // Pointer
    mol_num_t                   size;               // Full size
} mol_seg_t;
```
It comes with an assumption: the data has been loaded into memory already. It's not a good design to system like [CKB-VM](https://github.com/nervosnetwork/ckb-vm) which only has very limited memory. 

As we look into the [Molecule Spec](https://github.com/nervosnetwork/rfcs/blob/master/rfcs/0008-serialization/0008-serialization.md),
if we only need some part of data, we can get the data through some "hops". We can read the header only, estimating where to hop and don't need to read the remaining data. 
For a lot of scenarios which only need some part of data, we can have a load-on-demand mechanic.

This load-on-demand mechanic is introduced by the following data structure:
```C
typedef struct mol2_cursor_t {
  uint32_t offset;     // offset of slice
  uint32_t size;       // size of slice
  mol2_source_t read;  // data source
  void *arg;           // data source
} mol2_cursor_t;
```
The "read" together with "arg" are the data source. The "offset" together with "size", is an "view"/"slice" of the data source. Here the relationship between "read" and the "arg" is the same as "fopen" and "FILE*" in standard C library.

We have a very simple implementation of "read" field over memory: 
```C
// a sample source over memory
uint32_t mol2_source_memory(void *arg, uint8_t *ptr, uint32_t len,
                            uint32_t offset) {
  uint8_t *start_mem = (uint8_t *)arg;
  memcpy(ptr, start_mem + offset, len);
  return len;
}
```
We can also make another one based on syscall.

When "mol2_cursor_t" is returned from functions, it doesn't access memory.
As the name "cursor" suggests, it's only an cursor. We can access memory on demand by "mol2_read_at", for example:
```C
    mol2_cursor_t witness_cur = witnesses.tbl->at(&witnesses, 0);
    uint8_t witness[witness_cur.size];
    mol2_read_at(&witness_cur, witness, witness_cur.size);
    assert(witness_cur.size == 3 && witness[0] == 0x12 && witness[1] == 0x34);
```
