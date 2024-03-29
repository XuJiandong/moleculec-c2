![moleculec-c2](https://github.com/XuJiandong/moleculec-c2/workflows/moleculec-c2/badge.svg)

# moleculec-c2
Improved C/Rust plugin for the molecule serialization system.
Read as `molecule   c   c2`: the first `c` means compiler, `c2` is the code name. 
We already have a [moleculec](https://github.com/nervosnetwork/molecule) which is used here.

### How to use
- Install "moleculec"
```bash
make install-tools
```
- Compile Rust code to binary
```bash
cargo build --release
```
- Generate C/Rust files by moleculec-c2 and moleculec
```bash
# generate intermedia json file
moleculec --language - --schema-file mol/blockchain.mol --format json > mol/blockchain.json
# generate C
target/release/moleculec-c2 --input mol/blockchain.json | clang-format -style=Google > tests/blockchain/blockchain-api2.h
# generate Rust
target/release/moleculec-c2 --rust --input mol/blockchain.json | rustfmt > tests/blockchain_rust/src/blockchain.rs
```
- Include the generated file to your source file

The json file is intermedia file.  
`clang-format -style=Google` or `rustfmt` is not needed if you don't care about coding style.
_________________


The following are optimized compared to the old C/Rust API:
### Strong type for C
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

| Molecule type  | Type name   | C Type     | Rust Type |
|----------------|-------------|------------|-----------|
| byte           |  /          |  uint8_t   |   u8      |
| `[byte; 1]`     | int8       |  int8_t    |   i8      |
| `[byte; 1]`     | uint8      |  uint8_t   |   u8     |
| `[byte; 2]`     | int16      |  int16_t   |   i16    |
| `[byte; 2]`     | uint16     |  uint16_t  |   u16    |
| `[byte; 4]`     | int32      |  int32_t   |   i32    |
| `[byte; 4]`     | uint32     |  uint32_t  |   u32    |
| `[byte; 8]`     | int64      |  int64_t   |   i64    |
| `[byte; 8]`     | uint64     |  uint64_t  |   u64    |
| `[byte; N]`     | /          |  mol2_cursor_t  | `Cursor`  |
| `<byte>`        | /          |  mol2_cursor_t  | `Cursor`  |
| option          | /          |  /              | `Option<_>` |
 
The type name is case-insensitive. For example, int8, Int8, INT8 are all mapped to int8_t.

### Load memory on demand

mol_seg_t, is the most important data structure in old molecule API:
```C
typedef struct {
    uint8_t                     *ptr;               // Pointer
    mol_num_t                   size;               // Full size
} mol_seg_t;
```
It comes with an assumption: the data has been loaded into memory already. 
It's not a good design to system like [CKB-VM](https://github.com/nervosnetwork/ckb-vm) 
which only has very limited memory (4M). 

As we look into the [Molecule Spec](https://github.com/nervosnetwork/rfcs/blob/master/rfcs/0008-serialization/0008-serialization.md),
if we only need some part of data, we can get the data through some "hops". 
We can read the header only, estimating where to hop and don't need to read the remaining data. 
For a lot of scenarios which only need some part of data, we can have a load-on-demand mechanic.

This load-on-demand mechanic is introduced by the following data structure:
```C
typedef struct mol2_cursor_t {
  uint32_t offset;  // offset of slice
  uint32_t size;    // size of slice
  mol2_data_source_t *data_source;
} mol2_cursor_t;
```

We have a very simple implementation of "read" field over memory: 
```C
uint32_t mol2_source_memory(uintptr_t args[], uint8_t *ptr, uint32_t len,
                            uint32_t offset) {
  uint32_t mem_len = (uint32_t)args[1];
  ASSERT(offset < mem_len);
  uint32_t remaining_len = mem_len - offset;

  uint32_t min_len = MIN(remaining_len, len);
  uint8_t *start_mem = (uint8_t *)args[0];
  ASSERT((offset + min_len) <= mem_len);
  memcpy(ptr, start_mem + offset, min_len);
  return min_len;
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

The rust version is much simpler:
```Rust
impl Read for Vec<u8> {
    fn read(&self, buf: &mut [u8], offset: usize) -> Result<usize, Error> {
        let mem_len = self.len();
        if offset >= mem_len {
            return Err(Error::OutOfBound);
        }

        let remaining_len = mem_len - offset;
        let min_len = min(remaining_len, buf.len());

        if (offset + min_len) > mem_len {
            return Err(Error::OutOfBound);
        }
        buf[0..min_len].copy_from_slice(&self.as_slice()[offset..offset + min_len]);
        Ok(min_len)
    }
}

// same as `make_cursor_from_memory` in C
impl From<Vec<u8>> for Cursor {
    fn from(mem: Vec<u8>) -> Self {
        Cursor::new(MAX_CACHE_SIZE, mem.len(), Box::new(mem))
    }
}

```


### Cache support
When the data is read from data source via syscall, the costs on every syscall is expensive.
It would be great if it can read more data for future use for each syscall: now it supports cache for every reading.
See `mol2_read_at`(in C) or `read_at` (in Rust) for more information.

_________________

### Split declaration and definition for C
When the header file is generated, it can only be included in one single source file.
If you choose multiple source files, it's better to split declaration and definition.
Follow the following steps:
1. Define macro "MOLECULEC_C2_DECLARATION_ONLY" and include the header files
```C
#define MOLECULEC_C2_DECLARATION_ONLY
#include "sample-api2.h"
```
See [here](https://github.com/XuJiandong/moleculec-c2/blob/d00b3cfc9ceb9108507f4aa90220cfc42f3bf20f/tests/sample/decl-only-sample.c#L12-L13).
It can be repeated for every source files if needed.

2. Include header file fully in another source file (.c)
```C
#include "sample-api2.h"
```
See [here](https://github.com/XuJiandong/moleculec-c2/blob/d00b3cfc9ceb9108507f4aa90220cfc42f3bf20f/tests/sample/decl-only-impl.c#L5).
It can only be done once. 

### For CKB developer
There is an already generated file [blockchain-api2.h](https://github.com/XuJiandong/moleculec-c2/blob/master/tests/blockchain/blockchain-api2.h), together with 
[molecule2_reader.h](https://github.com/XuJiandong/moleculec-c2/blob/master/include/molecule2_reader.h): they can be included in source file directly.

The original mol file is [here](https://github.com/nervosnetwork/ckb/blob/master/util/types/schemas/blockchain.mol).
