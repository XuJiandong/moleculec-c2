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
  HeaderType header = block.tbl->header(&block);
  {
    mol2_cursor_t nonce_cur = header.tbl->nonce(&header);
    uint8_t nonce[nonce_cur.size];
    mol2_read_at(&nonce_cur, nonce, nonce_cur.size);
    assert(nonce_cur.size == 16 && nonce[0] == 0x12 && nonce[1] == 0x34);
  }
  RawHeaderType raw = header.tbl->raw(&header);
  uint32_t version = raw.tbl->version(&raw);
  assert(version == 0xFF);
  TransactionVecType txs = block.tbl->transactions(&block);
  size_t length = txs.tbl->len(&txs);
  assert(length == 2);
  int existing = 0;
  TransactionType tx0 = txs.tbl->get(&txs, 0, &existing);
  BytesVecType witnesses = tx0.tbl->witnesses(&tx0);
  size_t witnesses_length = witnesses.tbl->len(&witnesses);
  assert(witnesses_length == 2);
  {
    mol2_cursor_t witness_cur = witnesses.tbl->get(&witnesses, 0, &existing);
    uint8_t witness[witness_cur.size];
    mol2_read_at(&witness_cur, witness, witness_cur.size);
    assert(witness_cur.size == 3 && witness[0] == 0x12 && witness[1] == 0x34);
  }
  RawTransactionType raw_tx = tx0.tbl->raw(&tx0);
  uint32_t tx_version = raw_tx.tbl->version(&raw_tx);
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
