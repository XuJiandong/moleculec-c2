#ifndef MOLECULE_TOOLKIT_SRC_SAMPLE_BUILDER_H_
#define MOLECULE_TOOLKIT_SRC_SAMPLE_BUILDER_H_

#include "sample-api.h"

mol_seg_t build_SampleByte2() {
  mol_builder_t b;
  MolBuilder_SampleByte2_init(&b);
  MolBuilder_SampleByte2_set_nth0(&b, 1);
  MolBuilder_SampleByte2_set_nth1(&b, 2);
  mol_seg_res_t res = MolBuilder_SampleByte2_build(b);
  return res.seg;
}

mol_seg_t build_SampleFixedVector() {
  mol_builder_t b;
  MolBuilder_SampleFixedVector_init(&b);
  mol_seg_t s1 = build_SampleByte2();
  MolBuilder_SampleFixedVector_push(&b, 0xBE);
  MolBuilder_SampleFixedVector_push(&b, 0xEF);
  mol_seg_res_t res = MolBuilder_SampleFixedVector_build(b);

  assert(res.errno == 0);
  assert(MolReader_SampleFixedVector_verify(&res.seg, false) == 0);
  return res.seg;
}

mol_seg_t build_SampleDynVector() {
  mol_builder_t b;
  MolBuilder_SampleDynVector_init(&b);
  mol_seg_t s1 = build_SampleFixedVector();
  MolBuilder_SampleDynVector_push(&b, s1.ptr, s1.size);
  MolBuilder_SampleDynVector_push(&b, s1.ptr, s1.size);
  mol_seg_res_t res = MolBuilder_SampleDynVector_build(b);

  assert(res.errno == 0);
  assert(MolReader_SampleDynVector_verify(&res.seg, false) == 0);
  return res.seg;
}

mol_seg_t build_SampleTable() {
  mol_builder_t b;
  MolBuilder_SampleTable_init(&b);
  mol_seg_t s1 = build_SampleDynVector();
  mol_seg_t s2 = build_SampleByte2();
  MolBuilder_SampleTable_set_byte_2d_vector(&b, s1.ptr, s1.size);
  MolBuilder_SampleTable_set_byte2(&b, s2.ptr, s2.size);
  mol_seg_res_t res = MolBuilder_SampleTable_build(b);

  assert(res.errno == 0);
  assert(MolReader_SampleTable_verify(&res.seg, false) == 0);
  return res.seg;
}

mol_seg_t build_SampleTableOption() {
  mol_seg_t seg = build_SampleTable();
  return seg;
}

mol_seg_t build_SampleStruct() {
  mol_builder_t b;
  mol_seg_t b2 = build_SampleByte2();
  MolBuilder_SampleStruct_init(&b);
  uint32_t value = 1024;
  MolBuilder_SampleStruct_set_u32(&b, (uint8_t*)&value);
  MolBuilder_SampleStruct_set_byte2(&b, b2.ptr);
  mol_seg_res_t res = MolBuilder_SampleStruct_build(b);
  assert(res.errno == 0);
  assert(MolReader_SampleStruct_verify(&res.seg, false) == 0);
  return res.seg;
}

mol_seg_t build_SampleUnion() {
  mol_seg_t seg = build_SampleStruct();

  mol_builder_t b;
  mol_union_builder_initialize(&b, 16, 0, MolDefault_SampleStruct, 6);

  MolBuilder_SampleUnion_set_SampleStruct(&b, seg.ptr, seg.size);
  mol_seg_res_t res = MolBuilder_SampleUnion_build(b);
  assert(res.errno == 0);
  assert(MolReader_SampleUnion_verify(&res.seg, false) == 0);
  return res.seg;
}

#endif  // MOLECULE_TOOLKIT_SRC_SAMPLE_BUILDER_H_
