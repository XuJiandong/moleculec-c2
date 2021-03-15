
#ifndef MOLECULEC_C2_FUZZER_TESTS_FUZZER_FUZZER_FUNC_H_
#define MOLECULEC_C2_FUZZER_TESTS_FUZZER_FUZZER_FUNC_H_

#define ASSERT assert
#define	assert(e)	((void)0)
#define mol2_printf(x, ...) ((void)0)

#define MOL2_PANIC(err)                              \
  do {                                               \
    longjmp(g_return_point, err); \
  } while (0)


#include <stdint.h>
#include <stdio.h>
#include <setjmp.h>
jmp_buf g_return_point;

// -------------------------------
// here we use new api to read
// -------------------------------
#include "blockchain-api2.h"
#define assert_eq(a, b) assert((a) == (b))

void assert_cursor(mol2_cursor_t* cur, uint8_t v0, uint8_t v1) {
  uint8_t temp_buff[255];
  size_t temp_len = 255;
  uint32_t read = mol2_read_at(cur, temp_buff, temp_len);
//  assert_eq(read, cur->size);
//  assert_eq(temp_buff[0], v0);
//  assert_eq(temp_buff[1], v1);
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
  raw.t->number(&raw);

  TransactionVecType txs = block.t->transactions(&block);
  size_t length = txs.t->len(&txs);

  TransactionType tx0 = txs.t->get(&txs, 0, &existing);
  if (!existing) {
    return;
  }
  BytesVecType witnesses = tx0.t->witnesses(&tx0);
  size_t witnesses_length = witnesses.t->len(&witnesses);
  {
    mol2_cursor_t witness_cur = witnesses.t->get(&witnesses, 0, &existing);
    if (!existing)
      return;
    assert_cursor(&witness_cur, 0x12, 0x34);
  }
  RawTransactionType raw_tx = tx0.t->raw(&tx0);
  uint32_t tx_version = raw_tx.t->version(&raw_tx);
  {
    CellDepVecType cell_deps = raw_tx.t->cell_deps(&raw_tx);
    cell_deps.t->len(&cell_deps);
    CellDepType cell_dep = cell_deps.t->get(&cell_deps, 0, &existing);
    if (!existing)
      return;
    {
      cell_dep.t->dep_type(&cell_dep);
      OutPointType out_point = cell_dep.t->out_point(&cell_dep);
      out_point.t->index(&out_point);
      mol2_cursor_t tx_hash = out_point.t->tx_hash(&out_point);
      assert_cursor(&tx_hash, 0x12, 0x34);
    }
  }
  CellInputVecType cell_inputs = raw_tx.t->inputs(&raw_tx);
  CellInputType cell_input = cell_inputs.t->get(&cell_inputs, 0, &existing);
  if (!existing)
    return;
  OutPointType out_point = cell_input.t->previous_output(&cell_input);
  cell_input.t->since(&cell_input);
  out_point.t->tx_hash(&out_point);
  out_point.t->index(&out_point);

  CellOutputVecType cell_outputs = raw_tx.t->outputs(&raw_tx);
  CellOutputType cell_output = cell_outputs.t->get(&cell_outputs, 0, &existing);
  if (!existing) {
    return;
  }
  cell_output.t->capacity(&cell_output);
  ScriptType lock_script = cell_output.t->lock(&cell_output);
  lock_script.t->hash_type(&lock_script);

  mol2_cursor_t code_hash = lock_script.t->code_hash(&lock_script);
  assert_cursor(&code_hash, 0x12, 0x34);

  mol2_cursor_t args = lock_script.t->args(&lock_script);
  assert_cursor(&args, 0x12, 0x34);

  ScriptOptType type_script_opt = cell_output.t->type_(&cell_output);
  type_script_opt.t->is_some(&type_script_opt);

  ScriptType type_script = type_script_opt.t->unwrap(&type_script_opt);
  type_script.t->hash_type(&type_script);

  mol2_cursor_t code_hash2 = type_script.t->code_hash(&type_script);
  assert_cursor(&code_hash2, 0x12, 0x34);

  mol2_cursor_t args2 = type_script.t->args(&type_script);
  assert_cursor(&args2, 0x12, 0x34);
}



#endif //MOLECULEC_C2_FUZZER_TESTS_FUZZER_FUZZER_FUNC_H_
