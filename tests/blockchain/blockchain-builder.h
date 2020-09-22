// Can use build_Block to build a valid block data structure
// for testing and demo purpose.
// Include this file, then call "build_Block".

#include "blockchain-api.h"

#define ____ 0x00
typedef unsigned char byte;

mol_seg_t build_Transaction();
mol_seg_t build_UncleBlock();

mol_seg_t build_RawHeader() {
  mol_builder_t b;
  mol_seg_res_t res;
  MolBuilder_RawHeader_init(&b);
  uint32_t version = 0xFF;
  uint32_t compact_target = 0x12;
  uint64_t timestamp = 0x23;
  uint64_t number = 0x34;
  uint64_t epoch = 0x45;
  byte parent_hash[32] = {0};
  byte transactions_root[32] = {0};
  byte propsals_hash[32] = {0};
  byte uncles_hash[32] = {0};
  byte dao[32] = {0};

  MolBuilder_RawHeader_set_version(&b, (const uint8_t *)&version);
  MolBuilder_RawHeader_set_compact_target(&b, (const uint8_t *)&compact_target);
  MolBuilder_RawHeader_set_timestamp(&b, (const uint8_t *)&timestamp);
  MolBuilder_RawHeader_set_number(&b, (const uint8_t *)&number);
  MolBuilder_RawHeader_set_epoch(&b, (const uint8_t *)&epoch);
  MolBuilder_RawHeader_set_parent_hash(&b, (const uint8_t *)&parent_hash);
  MolBuilder_RawHeader_set_transactions_root(&b, transactions_root);
  MolBuilder_RawHeader_set_proposals_hash(&b, propsals_hash);
  MolBuilder_RawHeader_set_uncles_hash(&b, uncles_hash);
  MolBuilder_RawHeader_set_dao(&b, dao);
  res = MolBuilder_RawHeader_build(b);
  assert(res.errno == 0);
  assert(MolReader_RawHeader_verify(&res.seg, false) == 0);
  return res.seg;
}

#define FROM_INT(i) ((const uint8_t *)(&i))

mol_seg_t build_Bytes() {
  mol_builder_t b;
  mol_seg_res_t res;
  MolBuilder_Bytes_init(&b);
  MolBuilder_Bytes_push(&b, 0x12);
  MolBuilder_Bytes_push(&b, 0x34);
  MolBuilder_Bytes_push(&b, 0x56);
  res = MolBuilder_Bytes_build(b);
  assert(res.errno == 0);
  assert(MolReader_Bytes_verify(&res.seg, false) == 0);
  return res.seg;
}

mol_seg_t build_BytesVec() {
  mol_builder_t b;
  mol_seg_res_t res;
  MolBuilder_BytesVec_init(&b);
  mol_seg_t bytes = build_Bytes();
  MolBuilder_BytesVec_push(&b, bytes.ptr, bytes.size);
  MolBuilder_BytesVec_push(&b, bytes.ptr, bytes.size);
  res = MolBuilder_BytesVec_build(b);
  assert(res.errno == 0);
  assert(MolReader_BytesVec_verify(&res.seg, false) == 0);
  return res.seg;
}

mol_seg_t build_Byte32Vec() {
  mol_builder_t b;
  mol_seg_res_t res;
  MolBuilder_Byte32Vec_init(&b);

  byte content[32] = {0};
  MolBuilder_Byte32Vec_push(&b, content);
  MolBuilder_Byte32Vec_push(&b, content);

  res = MolBuilder_Byte32Vec_build(b);
  assert(res.errno == 0);
  assert(MolReader_Byte32Vec_verify(&res.seg, false) == 0);
  return res.seg;
}

mol_seg_t build_Script() {
  mol_builder_t b;
  mol_seg_res_t res;
  MolBuilder_Script_init(&b);
  byte code_hash[32] = {0x12, 0x34, 0x56, 0x78};
  byte hash_type = 0x12;

  MolBuilder_Script_set_code_hash(&b, code_hash, 32);
  MolBuilder_Script_set_hash_type(&b, hash_type);
  mol_seg_t bytes = build_Bytes();
  MolBuilder_Script_set_args(&b, bytes.ptr, bytes.size);

  res = MolBuilder_Script_build(b);
  assert(res.errno == 0);
  assert(MolReader_Script_verify(&res.seg, false) == 0);
  return res.seg;
}

mol_seg_t build_OutPoint() {
  mol_builder_t b;
  mol_seg_res_t res;
  MolBuilder_OutPoint_init(&b);

  byte tx_hash[32] = {0};
  MolBuilder_OutPoint_set_tx_hash(&b, tx_hash);
  uint32_t index = 0x12;
  MolBuilder_OutPoint_set_index(&b, FROM_INT(index));

  res = MolBuilder_OutPoint_build(b);
  assert(res.errno == 0);
  assert(MolReader_OutPoint_verify(&res.seg, false) == 0);
  return res.seg;
}

mol_seg_t build_CellInput() {
  mol_builder_t b;
  mol_seg_res_t res;

  MolBuilder_CellInput_init(&b);

  uint64_t since = 0x12;
  MolBuilder_CellInput_set_since(&b, FROM_INT(since));

  mol_seg_t previous_output = build_OutPoint();
  MolBuilder_CellInput_set_previous_output(&b, previous_output.ptr);
  res = MolBuilder_CellInput_build(b);
  assert(res.errno == 0);
  assert(MolReader_CellInput_verify(&res.seg, false) == 0);
  return res.seg;
}

mol_seg_t build_CellOutput() {
  mol_builder_t b;
  mol_seg_res_t res;

  MolBuilder_CellOutput_init(&b);

  uint64_t capacity = 1000;
  MolBuilder_CellOutput_set_capacity(&b, FROM_INT(capacity), 8);
  mol_seg_t lock = build_Script();
  MolBuilder_CellOutput_set_lock(&b, lock.ptr, lock.size);
  MolBuilder_CellOutput_set_type_(&b, lock.ptr, lock.size);

  res = MolBuilder_CellOutput_build(b);
  assert(res.errno == 0);
  assert(MolReader_CellOutput_verify(&res.seg, false) == 0);
  return res.seg;
}

mol_seg_t build_CellDep() {
  mol_builder_t b;
  mol_seg_res_t res;

  MolBuilder_CellDep_init(&b);
  mol_seg_t out_point = build_OutPoint();
  MolBuilder_CellDep_set_out_point(&b, out_point.ptr);
  MolBuilder_CellDep_set_dep_type(&b, 0);
  res = MolBuilder_CellDep_build(b);
  assert(res.errno == 0);
  assert(MolReader_CellDep_verify(&res.seg, false) == 0);
  return res.seg;
}

mol_seg_t build_Header() {
  mol_builder_t b;
  mol_seg_res_t res;

  MolBuilder_Header_init(&b);
  mol_seg_t raw = build_RawHeader();
  MolBuilder_Header_set_raw(&b, raw.ptr);
  byte nonce[16] = {0x12, 0x34};
  MolBuilder_Header_set_nonce(&b, nonce);
  res = MolBuilder_Header_build(b);
  assert(res.errno == 0);
  assert(MolReader_Header_verify(&res.seg, false) == 0);
  return res.seg;
}

mol_seg_t build_ProposalShortId() {
  mol_builder_t b;
  mol_seg_res_t res;

  MolBuilder_ProposalShortId_init(&b);
  MolBuilder_ProposalShortId_set_nth0(&b, 0x12);
  MolBuilder_ProposalShortId_set_nth1(&b, 0x34);
  res = MolBuilder_ProposalShortId_build(b);
  assert(res.errno == 0);
  return res.seg;
}

mol_seg_t build_ProposalShortIdVec() {
  mol_builder_t b;
  mol_seg_res_t res;

  MolBuilder_ProposalShortIdVec_init(&b);

  MolBuilder_ProposalShortIdVec_push(&b, build_ProposalShortId().ptr);
  MolBuilder_ProposalShortIdVec_push(&b, build_ProposalShortId().ptr);
  res = MolBuilder_ProposalShortIdVec_build(b);
  assert(res.errno == 0);
  assert(MolReader_ProposalShortIdVec_verify(&res.seg, false) == 0);
  return res.seg;
}

mol_seg_t build_UncleBlockVec() {
  mol_builder_t b;
  mol_seg_res_t res;

  MolBuilder_UncleBlockVec_init(&b);

  mol_seg_t uncle_block = build_UncleBlock();

  MolBuilder_UncleBlockVec_push(&b, uncle_block.ptr, uncle_block.size);
  MolBuilder_UncleBlockVec_push(&b, uncle_block.ptr, uncle_block.size);

  res = MolBuilder_UncleBlockVec_build(b);
  assert(res.errno == 0);
  assert(MolReader_UncleBlockVec_verify(&res.seg, false) == 0);
  return res.seg;
}

mol_seg_t build_TransactionVec() {
  mol_builder_t b;
  mol_seg_res_t res;

  MolBuilder_TransactionVec_init(&b);

  mol_seg_t tx = build_Transaction();

  MolBuilder_TransactionVec_push(&b, tx.ptr, tx.size);
  MolBuilder_TransactionVec_push(&b, tx.ptr, tx.size);

  res = MolBuilder_TransactionVec_build(b);
  assert(res.errno == 0);
  assert(MolReader_TransactionVec_verify(&res.seg, false) == 0);
  return res.seg;
}

mol_seg_t build_CellDepVec() {
  mol_builder_t b;
  mol_seg_res_t res;

  MolBuilder_CellDepVec_init(&b);

  MolBuilder_CellDepVec_push(&b, build_CellDep().ptr);
  MolBuilder_CellDepVec_push(&b, build_CellDep().ptr);

  res = MolBuilder_CellDepVec_build(b);
  assert(res.errno == 0);
  assert(MolReader_CellDepVec_verify(&res.seg, false) == 0);
  return res.seg;
}

mol_seg_t build_CellInputVec() {
  mol_builder_t b;
  mol_seg_res_t res;

  MolBuilder_CellInputVec_init(&b);

  MolBuilder_CellInputVec_push(&b, build_CellInput().ptr);
  MolBuilder_CellInputVec_push(&b, build_CellInput().ptr);

  res = MolBuilder_CellInputVec_build(b);
  assert(res.errno == 0);
  assert(MolReader_CellInputVec_verify(&res.seg, false) == 0);
  return res.seg;
}

mol_seg_t build_CellOutputVec() {
  mol_builder_t b;
  mol_seg_res_t res;

  MolBuilder_CellOutputVec_init(&b);

  mol_seg_t cell_output = build_CellOutput();
  MolBuilder_CellOutputVec_push(&b, cell_output.ptr, cell_output.size);
  MolBuilder_CellOutputVec_push(&b, cell_output.ptr, cell_output.size);

  res = MolBuilder_CellOutputVec_build(b);
  assert(res.errno == 0);
  assert(MolReader_CellOutputVec_verify(&res.seg, false) == 0);
  return res.seg;
}

mol_seg_t build_UncleBlock() {
  mol_builder_t b;
  mol_seg_res_t res;

  MolBuilder_UncleBlock_init(&b);
  mol_seg_t header = build_Header();
  MolBuilder_UncleBlock_set_header(&b, header.ptr, header.size);
  mol_seg_t proposal = build_ProposalShortIdVec();
  MolBuilder_UncleBlock_set_proposals(&b, proposal.ptr, proposal.size);

  res = MolBuilder_UncleBlock_build(b);
  assert(res.errno == 0);
  assert(MolReader_UncleBlock_verify(&res.seg, false) == 0);
  return res.seg;
}

mol_seg_t build_RawTransaction() {
  mol_builder_t b;
  mol_seg_res_t res;

  MolBuilder_RawTransaction_init(&b);
  uint32_t version = 0x12;
  MolBuilder_RawTransaction_set_version(&b, FROM_INT(version), 4);
  mol_seg_t cell_dep_vec = build_CellDepVec();
  MolBuilder_RawTransaction_set_cell_deps(&b, cell_dep_vec.ptr,
                                          cell_dep_vec.size);
  mol_seg_t header_dep = build_Byte32Vec();
  MolBuilder_RawTransaction_set_header_deps(&b, header_dep.ptr,
                                            header_dep.size);
  mol_seg_t cell_input = build_CellInputVec();
  MolBuilder_RawTransaction_set_inputs(&b, cell_input.ptr, cell_input.size);
  mol_seg_t cell_output = build_CellOutputVec();
  MolBuilder_RawTransaction_set_outputs(&b, cell_output.ptr, cell_output.size);
  mol_seg_t output_data = build_BytesVec();
  MolBuilder_RawTransaction_set_outputs_data(&b, output_data.ptr,
                                             output_data.size);

  res = MolBuilder_RawTransaction_build(b);
  assert(res.errno == 0);
  assert(MolReader_RawTransaction_verify(&res.seg, false) == 0);
  return res.seg;
}

mol_seg_t build_Transaction() {
  mol_builder_t b;
  mol_seg_res_t res;

  MolBuilder_Transaction_init(&b);
  mol_seg_t raw_transaction = build_RawTransaction();
  MolBuilder_Transaction_set_raw(&b, raw_transaction.ptr, raw_transaction.size);
  mol_seg_t witness = build_BytesVec();
  MolBuilder_Transaction_set_witnesses(&b, witness.ptr, witness.size);

  res = MolBuilder_Transaction_build(b);
  assert(res.errno == 0);
  assert(MolReader_Transaction_verify(&res.seg, false) == 0);
  return res.seg;
}

mol_seg_t build_Block() {
  mol_builder_t b;
  mol_seg_res_t res;

  MolBuilder_Block_init(&b);
  mol_seg_t header = build_Header();
  MolBuilder_Block_set_header(&b, header.ptr, header.size);
  mol_seg_t uncles = build_UncleBlockVec();
  MolBuilder_Block_set_uncles(&b, uncles.ptr, uncles.size);
  mol_seg_t transaction_vec = build_TransactionVec();
  MolBuilder_Block_set_transactions(&b, transaction_vec.ptr,
                                    transaction_vec.size);
  mol_seg_t proposals = build_ProposalShortIdVec();
  MolBuilder_Block_set_proposals(&b, proposals.ptr, proposals.size);

  res = MolBuilder_Block_build(b);
  assert(res.errno == 0);
  assert(MolReader_Block_verify(&res.seg, false) == 0);
  return res.seg;
}

void read_Block(mol_seg_t data) {
  mol_seg_t txs = MolReader_Block_get_transactions(&data);
  mol_seg_res_t res = MolReader_TransactionVec_get(&txs, 0);
  assert(res.errno == 0);
  mol_seg_t tx0 = res.seg;
  mol_seg_t raw = MolReader_Transaction_get_raw(&tx0);
  mol_seg_t outputs = MolReader_RawTransaction_get_outputs(&raw);
  res = MolReader_CellOutputVec_get(&outputs, 0);
  assert(res.errno == 0);
  mol_seg_t output = res.seg;
  mol_seg_t capacity = MolReader_CellOutput_get_capacity(&output);
  assert(mol_unpack_number(capacity.ptr) == 1000);

  mol_seg_t lock = MolReader_CellOutput_get_lock(&output);
  mol_seg_t code_hash = MolReader_Script_get_code_hash(&lock);
  assert(code_hash.ptr[0] == 0x12 && code_hash.ptr[1] == 0x34);
}
