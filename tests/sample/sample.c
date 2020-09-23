#undef NDEBUG
#include <assert.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include <stdbool.h>

#include "sample-api2.h"
//#include "sample-api2.h"
#include "sample-builder.h"

void verify_sample_option_table(mol2_cursor_t* sample_table2) {
  SampleOptionTableType option_t = make_SampleOptionTable(sample_table2);
  assert(!option_t.t->is_none(&option_t));
  assert(option_t.t->is_some(&option_t));
  SampleTableType t = option_t.t->unwrap(&option_t);
  mol2_cursor_t b2 = t.t->byte2(&t);
  uint8_t buff[2];
  mol2_read_at(&b2, buff, 2);
  assert(buff[0] == 1);
  assert(buff[1] == 2);
}

void verify_sample_union(void) {
  mol_seg_t sample = build_SampleUnion();
  // build new data from scratch
  mol2_cursor_t sample_union;
  sample_union.offset = 0;
  sample_union.size = sample_union.size;
  sample_union.read = mol2_source_memory;
  sample_union.arg = sample.ptr;

  SampleUnionType u = make_SampleUnion(&sample_union);
  assert(u.t->item_id(&u) == 0);

  SampleStructType t = u.t->as_SampleStruct(&u);
  assert(t.t->u32(&t) == 1024);

  mol2_cursor_t b2 = t.t->byte2(&t);
  uint8_t buff[2];
  int read_len = mol2_read_at(&b2, buff, 2);
  assert(read_len == 2);
  assert(buff[0] == 1 && buff[1] == 2);
}


int main(int argc, const char* argv[]) {
  mol_seg_t sample_table = build_SampleTable();

  mol2_cursor_t sample_table2;
  sample_table2.offset = 0;
  sample_table2.size = sample_table.size;
  sample_table2.read = mol2_source_memory;
  sample_table2.arg = sample_table.ptr;

  SampleTableType t = make_SampleTable(&sample_table2);
  mol2_cursor_t b2 = t.t->byte2(&t);
  uint8_t buff[2];
  mol2_read_at(&b2, buff, 2);
  assert(buff[0] == 1);
  assert(buff[1] == 2);

  SampleDynVectorType byte_2d = t.t->byte_2d_vector(&t);
  bool existing = false;
  uint32_t len = byte_2d.t->len(&byte_2d);
  mol2_cursor_t cur = byte_2d.t->get(&byte_2d, 0, &existing);
  mol2_read_at(&cur, buff, 2);
  assert(buff[0] == 0xBE);
  assert(buff[1] == 0xEF);
  cur = byte_2d.t->get(&byte_2d, 1, &existing);
  mol2_read_at(&cur, buff, 2);
  assert(buff[0] == 0xBE);
  assert(buff[1] == 0xEF);

  verify_sample_option_table(&sample_table2);
  verify_sample_union();

  printf("\n------- sample passed---------\n\n");
  return 0;
}

