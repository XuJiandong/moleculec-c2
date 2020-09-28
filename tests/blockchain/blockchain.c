#undef NDEBUG
#include <assert.h>
#include <stdint.h>
#include <stdio.h>

#include "blockchain-builder.h"

// -------------------------------
// here we use new api to read
// -------------------------------
#include "blockchain-api2.h"

void read_with_new_api(mol2_cursor_t data) {
  // note, the life time of the memory which data.ptr points to,
  // should be longer than any other objects(header, raw, egc)
  // which are derived from "block"
  BlockType block = make_Block(&data);
  HeaderType header = block.t->header(&block);
  {
    mol2_cursor_t nonce_cur = header.t->nonce(&header);
    uint8_t nonce[nonce_cur.size];
    mol2_read_at(&nonce_cur, nonce, nonce_cur.size);
    assert(nonce_cur.size == 16 && nonce[0] == 0x12 && nonce[1] == 0x34);
  }
  RawHeaderType raw = header.t->raw(&header);
  uint32_t version = raw.t->version(&raw);
  assert(version == 0xFF);
  TransactionVecType txs = block.t->transactions(&block);
  size_t length = txs.t->len(&txs);
  assert(length == 2);
  bool existing = false;
  TransactionType tx0 = txs.t->get(&txs, 0, &existing);
  BytesVecType witnesses = tx0.t->witnesses(&tx0);
  size_t witnesses_length = witnesses.t->len(&witnesses);
  assert(witnesses_length == 2);
  {
    mol2_cursor_t witness_cur = witnesses.t->get(&witnesses, 0, &existing);
    uint8_t witness[witness_cur.size];
    mol2_read_at(&witness_cur, witness, witness_cur.size);
    assert(witness_cur.size == 3 && witness[0] == 0x12 && witness[1] == 0x34);
  }
  RawTransactionType raw_tx = tx0.t->raw(&tx0);
  uint32_t tx_version = raw_tx.t->version(&raw_tx);
  assert(tx_version == 0x12);

  printf("done");
}

int main(int argc, const char *argv[]) {
  mol_seg_t block = build_Block();
  read_Block(block);

  mol2_cursor_t block2;
  block2.offset = 0;
  block2.size = block.size;
  block2.read = mol2_source_memory;
  block2.arg = block.ptr;
  read_with_new_api(block2);

  printf("\n------- blockchain passed---------\n\n");

  return 0;
}
