#undef NDEBUG
#define ASSERT assert
#define MOL2_EXIT exit

#include <assert.h>
#include <stdint.h>
#include <stdio.h>

#include "blockchain-builder.h"

// -------------------------------
// here we use new api to read
// -------------------------------
#include "blockchain-api2.h"
#define assert_eq(a, b) assert((a) == (b))

mol2_cursor_t getCursor(mol_seg_t *block);
void assert_cursor(mol2_cursor_t* cur, uint8_t v0, uint8_t v1) {
  uint8_t temp_buff[255];
  size_t temp_len = 255;
  uint32_t read = mol2_read_at(cur, temp_buff, temp_len);
  assert_eq(read, cur->size);
  assert_eq(temp_buff[0], v0);
  assert_eq(temp_buff[1], v1);
}


void read_with_new_api(mol2_cursor_t data) {
  bool existing = false;

  // note, the life time of the memory which data.ptr points to,
  // should be longer than any other objects(header, raw, egc)
  // which are derived from "block"
  BlockType block = make_Block(&data);
  HeaderType header = block.t->header(&block);
  {
    mol2_cursor_t nonce_cur = header.t->nonce(&header);
    assert_cursor(&nonce_cur, 0x12, 0x34);
  }
  RawHeaderType raw = header.t->raw(&header);
  uint32_t version = raw.t->version(&raw);
  assert(version == 0xFF);
  assert_eq(raw.t->number(&raw), 0x34);

  TransactionVecType txs = block.t->transactions(&block);
  size_t length = txs.t->len(&txs);
  assert(length == 2);

  TransactionType tx0 = txs.t->get(&txs, 0, &existing);
  BytesVecType witnesses = tx0.t->witnesses(&tx0);
  size_t witnesses_length = witnesses.t->len(&witnesses);
  assert(witnesses_length == 2);
  {
    mol2_cursor_t witness_cur = witnesses.t->get(&witnesses, 0, &existing);
    assert_cursor(&witness_cur, 0x12, 0x34);
  }
  RawTransactionType raw_tx = tx0.t->raw(&tx0);
  uint32_t tx_version = raw_tx.t->version(&raw_tx);
  assert(tx_version == 0x12);

  {
    CellDepVecType cell_deps = raw_tx.t->cell_deps(&raw_tx);
    assert_eq(cell_deps.t->len(&cell_deps), 2);
    CellDepType cell_dep = cell_deps.t->get(&cell_deps, 0, &existing);
    assert_eq(cell_dep.t->dep_type(&cell_dep), 0x12);

    OutPointType out_point = cell_dep.t->out_point(&cell_dep);
    assert_eq(out_point.t->index(&out_point), 0x12);
    mol2_cursor_t tx_hash = out_point.t->tx_hash(&out_point);
    assert_cursor(&tx_hash, 0x12, 0x34);
  }
  CellOutputVecType cell_outputs = raw_tx.t->outputs(&raw_tx);
  CellOutputType cell_output = cell_outputs.t->get(&cell_outputs, 0, &existing);
  assert_eq(cell_output.t->capacity(&cell_output), 1000);
  {
    ScriptType lock_script = cell_output.t->lock(&cell_output);
    assert_eq(lock_script.t->hash_type(&lock_script), 0x12);

    mol2_cursor_t code_hash = lock_script.t->code_hash(&lock_script);
    assert_cursor(&code_hash, 0x12, 0x34);

    mol2_cursor_t args = lock_script.t->args(&lock_script);
    assert_cursor(&args, 0x12, 0x34);
  }
  {
    ScriptOptType type_script_opt = cell_output.t->type_(&cell_output);
    assert(type_script_opt.t->is_some(&type_script_opt));

    ScriptType type_script = type_script_opt.t->unwrap(&type_script_opt);
    assert_eq(type_script.t->hash_type(&type_script), 0x12);

    mol2_cursor_t code_hash = type_script.t->code_hash(&type_script);
    assert_cursor(&code_hash, 0x12, 0x34);

    mol2_cursor_t args = type_script.t->args(&type_script);
    assert_cursor(&args, 0x12, 0x34);
  }
}

void stress_test_cache(void* ptr, uint32_t size) {
  for (uint32_t i = 4; i <= 2048; i += 4) {
    mol2_cursor_t cur = mol2_make_cursor_from_memory(ptr, size);
    // change max_cache_size, to trigger cache miss
    cur.data_source->max_cache_size = i;
    read_with_new_api(cur);
  }

  printf("------- stress test passed---------\n");
}

int main(int argc, const char *argv[]) {
  mol_seg_t block = build_Block();
  read_Block(block);
  mol2_cursor_t block2 = mol2_make_cursor_from_memory(block.ptr, block.size);
  read_with_new_api(block2);

  stress_test_cache(block.ptr, block.size);
  printf("\n------- blockchain passed---------\n\n");
  return 0;
}
