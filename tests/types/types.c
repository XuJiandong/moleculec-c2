#undef NDEBUG
#include <assert.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include <stdbool.h>

#include "types-api2.h"
void* build_all(int* size);

#define assert_eq(a, b) assert((a) == (b))

void assert_cursor(mol2_cursor_t* cur) {
  uint8_t temp_buff[255];
  size_t temp_len = 255;
  uint32_t read = mol2_read_at(cur, temp_buff, temp_len);
  assert_eq(read, cur->size);
  for (int i = 0; i < cur->size; i++) {
    assert_eq(temp_buff[i], 0);
  }
}

int main(int argc, const char* argv[]) {
  int size = 0;
  uint8_t* buff = build_all(&size);

  mol2_cursor_t table;
  table.offset = 0;
  table.size = size;
  table.read = mol2_source_memory;
  table.arg = buff;

  AllInOneType all = make_AllInOne(&table);
  assert_eq(all.t->f0(&all), 0);

  uint8_t temp_buff[255];
  size_t temp_len = 255;
  {
    mol2_cursor_t f = all.t->f9(&all);
    assert_cursor(&f);
  }
  {
    bool existing = false;
    Word8Type f = all.t->f23(&all);
    assert_eq(f.t->len(&f), 8);
    mol2_cursor_t t2 = f.t->get(&f, 0, &existing);
    assert_cursor(&t2);
  }
  {
    bool existing = false;
    Byte9x3Type f = all.t->f27(&all);
    assert_eq(f.t->len(&f), 3);
    mol2_cursor_t t2 = f.t->get(&f, 0, &existing);
    assert_cursor(&t2);
  }
  {
    StructPType f = all.t->f40(&all);
    StructJType j = f.t->f1(&f);
    mol2_cursor_t f2 = j.t->f1(&j);
    assert_cursor(&f2);
  }
  {
    StructIVecType f = all.t->f45(&all);
    assert_eq(f.t->len(&f), 0);
    StructJVecType j = all.t->f46(&all);
    assert_eq(j.t->len(&j), 0);
    StructPVecType p = all.t->f47(&all);
    assert_eq(p.t->len(&p), 0);
    BytesVecType b = all.t->f48(&all);
    assert_eq(b.t->len(&b), 0);
    WordsVecType w = all.t->f49(&all);
    assert_eq(w.t->len(&w), 0);
  }
  {
    Table6Type t = all.t->f56(&all);
    Table5Type s = t.t->f6(&t);
    StructAType a = s.t->f3(&s);
    assert_eq(a.t->f1(&a), 0);
  }
  {
    ByteOptType b = all.t->f57(&all);
    assert_eq(b.t->is_none(&b), true);
    assert_eq(b.t->is_some(&b), false);
  }
  {
    ByteOptVecType b = all.t->f68(&all);
    assert_eq(b.t->len(&b), 0);

    WordsOptVecType w = all.t->f70(&all);
    assert_eq(w.t->len(&w), 0);
  }
  {
    TableAType t = all.t->f73(&all);
    Word2Type w = t.t->f1(&t);
    assert_eq(w.t->len(&w), 2);
  }

  printf("\n------- types passed---------\n\n");
  return 0;
}
