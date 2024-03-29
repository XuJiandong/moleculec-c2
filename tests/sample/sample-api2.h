
#ifndef _SAMPLE_API2_H_
#define _SAMPLE_API2_H_

#define MOLECULEC2_VERSION 7002
#define MOLECULE2_API_VERSION_MIN 5000

#include "molecule2_reader.h"

#ifdef __cplusplus
extern "C" {
#endif /* __cplusplus */

// ----forward declaration--------
struct SampleByte2Type;
struct SampleByte2VTable;
struct SampleByte2VTable *GetSampleByte2VTable(void);
struct SampleByte2Type make_SampleByte2(mol2_cursor_t *cur);
uint32_t SampleByte2_len_impl(struct SampleByte2Type *);
uint8_t SampleByte2_get_impl(struct SampleByte2Type *, uint32_t, bool *);
struct Uint8Type;
struct Uint8VTable;
struct Uint8VTable *GetUint8VTable(void);
struct Uint8Type make_Uint8(mol2_cursor_t *cur);
uint32_t Uint8_len_impl(struct Uint8Type *);
uint8_t Uint8_get_impl(struct Uint8Type *, uint32_t, bool *);
struct Int8Type;
struct Int8VTable;
struct Int8VTable *GetInt8VTable(void);
struct Int8Type make_Int8(mol2_cursor_t *cur);
uint32_t Int8_len_impl(struct Int8Type *);
uint8_t Int8_get_impl(struct Int8Type *, uint32_t, bool *);
struct Uint16Type;
struct Uint16VTable;
struct Uint16VTable *GetUint16VTable(void);
struct Uint16Type make_Uint16(mol2_cursor_t *cur);
uint32_t Uint16_len_impl(struct Uint16Type *);
uint8_t Uint16_get_impl(struct Uint16Type *, uint32_t, bool *);
struct Int16Type;
struct Int16VTable;
struct Int16VTable *GetInt16VTable(void);
struct Int16Type make_Int16(mol2_cursor_t *cur);
uint32_t Int16_len_impl(struct Int16Type *);
uint8_t Int16_get_impl(struct Int16Type *, uint32_t, bool *);
struct Uint32Type;
struct Uint32VTable;
struct Uint32VTable *GetUint32VTable(void);
struct Uint32Type make_Uint32(mol2_cursor_t *cur);
uint32_t Uint32_len_impl(struct Uint32Type *);
uint8_t Uint32_get_impl(struct Uint32Type *, uint32_t, bool *);
struct Int32Type;
struct Int32VTable;
struct Int32VTable *GetInt32VTable(void);
struct Int32Type make_Int32(mol2_cursor_t *cur);
uint32_t Int32_len_impl(struct Int32Type *);
uint8_t Int32_get_impl(struct Int32Type *, uint32_t, bool *);
struct Uint64Type;
struct Uint64VTable;
struct Uint64VTable *GetUint64VTable(void);
struct Uint64Type make_Uint64(mol2_cursor_t *cur);
uint32_t Uint64_len_impl(struct Uint64Type *);
uint8_t Uint64_get_impl(struct Uint64Type *, uint32_t, bool *);
struct Int64Type;
struct Int64VTable;
struct Int64VTable *GetInt64VTable(void);
struct Int64Type make_Int64(mol2_cursor_t *cur);
uint32_t Int64_len_impl(struct Int64Type *);
uint8_t Int64_get_impl(struct Int64Type *, uint32_t, bool *);
struct SampleFixedVectorType;
struct SampleFixedVectorVTable;
struct SampleFixedVectorVTable *GetSampleFixedVectorVTable(void);
struct SampleFixedVectorType make_SampleFixedVector(mol2_cursor_t *cur);
uint32_t SampleFixedVector_len_impl(struct SampleFixedVectorType *);
uint8_t SampleFixedVector_get_impl(struct SampleFixedVectorType *, uint32_t,
                                   bool *);
struct SampleDynVectorType;
struct SampleDynVectorVTable;
struct SampleDynVectorVTable *GetSampleDynVectorVTable(void);
struct SampleDynVectorType make_SampleDynVector(mol2_cursor_t *cur);
uint32_t SampleDynVector_len_impl(struct SampleDynVectorType *);
mol2_cursor_t SampleDynVector_get_impl(struct SampleDynVectorType *, uint32_t,
                                       bool *);
struct SampleUint64VectorType;
struct SampleUint64VectorVTable;
struct SampleUint64VectorVTable *GetSampleUint64VectorVTable(void);
struct SampleUint64VectorType make_SampleUint64Vector(mol2_cursor_t *cur);
uint32_t SampleUint64Vector_len_impl(struct SampleUint64VectorType *);
uint64_t SampleUint64Vector_get_impl(struct SampleUint64VectorType *, uint32_t,
                                     bool *);
struct SampleStructType;
struct SampleStructVTable;
struct SampleStructVTable *GetSampleStructVTable(void);
struct SampleStructType make_SampleStruct(mol2_cursor_t *cur);
uint32_t SampleStruct_get_u32_impl(struct SampleStructType *);
mol2_cursor_t SampleStruct_get_byte2_impl(struct SampleStructType *);
struct SampleTableType;
struct SampleTableVTable;
struct SampleTableVTable *GetSampleTableVTable(void);
struct SampleTableType make_SampleTable(mol2_cursor_t *cur);
struct SampleDynVectorType SampleTable_get_byte_2d_vector_impl(
    struct SampleTableType *);
mol2_cursor_t SampleTable_get_byte2_impl(struct SampleTableType *);
struct SampleUint64VectorType SampleTable_get_u64_vector_impl(
    struct SampleTableType *);
struct SampleUnionType;
struct SampleUnionVTable;
struct SampleUnionVTable *GetSampleUnionVTable(void);
struct SampleUnionType make_SampleUnion(mol2_cursor_t *cur);
uint32_t SampleUnion_item_id_impl(struct SampleUnionType *);
struct SampleStructType SampleUnion_as_SampleStruct_impl(
    struct SampleUnionType *);
struct SampleTableType SampleUnion_as_SampleTable_impl(
    struct SampleUnionType *);
struct SampleOptionTableType;
struct SampleOptionTableVTable;
struct SampleOptionTableVTable *GetSampleOptionTableVTable(void);
struct SampleOptionTableType make_SampleOptionTable(mol2_cursor_t *cur);
bool SampleOptionTable_is_none_impl(struct SampleOptionTableType *);
bool SampleOptionTable_is_some_impl(struct SampleOptionTableType *);
struct SampleTableType SampleOptionTable_unwrap_impl(
    struct SampleOptionTableType *);

// ----definition-----------------
typedef struct SampleByte2VTable {
  uint32_t (*len)(struct SampleByte2Type *);
  uint8_t (*get)(struct SampleByte2Type *, uint32_t, bool *);
} SampleByte2VTable;
typedef struct SampleByte2Type {
  mol2_cursor_t cur;
  SampleByte2VTable *t;
} SampleByte2Type;

typedef struct Uint8VTable {
  uint32_t (*len)(struct Uint8Type *);
  uint8_t (*get)(struct Uint8Type *, uint32_t, bool *);
} Uint8VTable;
typedef struct Uint8Type {
  mol2_cursor_t cur;
  Uint8VTable *t;
} Uint8Type;

typedef struct Int8VTable {
  uint32_t (*len)(struct Int8Type *);
  uint8_t (*get)(struct Int8Type *, uint32_t, bool *);
} Int8VTable;
typedef struct Int8Type {
  mol2_cursor_t cur;
  Int8VTable *t;
} Int8Type;

typedef struct Uint16VTable {
  uint32_t (*len)(struct Uint16Type *);
  uint8_t (*get)(struct Uint16Type *, uint32_t, bool *);
} Uint16VTable;
typedef struct Uint16Type {
  mol2_cursor_t cur;
  Uint16VTable *t;
} Uint16Type;

typedef struct Int16VTable {
  uint32_t (*len)(struct Int16Type *);
  uint8_t (*get)(struct Int16Type *, uint32_t, bool *);
} Int16VTable;
typedef struct Int16Type {
  mol2_cursor_t cur;
  Int16VTable *t;
} Int16Type;

typedef struct Uint32VTable {
  uint32_t (*len)(struct Uint32Type *);
  uint8_t (*get)(struct Uint32Type *, uint32_t, bool *);
} Uint32VTable;
typedef struct Uint32Type {
  mol2_cursor_t cur;
  Uint32VTable *t;
} Uint32Type;

typedef struct Int32VTable {
  uint32_t (*len)(struct Int32Type *);
  uint8_t (*get)(struct Int32Type *, uint32_t, bool *);
} Int32VTable;
typedef struct Int32Type {
  mol2_cursor_t cur;
  Int32VTable *t;
} Int32Type;

typedef struct Uint64VTable {
  uint32_t (*len)(struct Uint64Type *);
  uint8_t (*get)(struct Uint64Type *, uint32_t, bool *);
} Uint64VTable;
typedef struct Uint64Type {
  mol2_cursor_t cur;
  Uint64VTable *t;
} Uint64Type;

typedef struct Int64VTable {
  uint32_t (*len)(struct Int64Type *);
  uint8_t (*get)(struct Int64Type *, uint32_t, bool *);
} Int64VTable;
typedef struct Int64Type {
  mol2_cursor_t cur;
  Int64VTable *t;
} Int64Type;

typedef struct SampleFixedVectorVTable {
  uint32_t (*len)(struct SampleFixedVectorType *);
  uint8_t (*get)(struct SampleFixedVectorType *, uint32_t, bool *);
} SampleFixedVectorVTable;
typedef struct SampleFixedVectorType {
  mol2_cursor_t cur;
  SampleFixedVectorVTable *t;
} SampleFixedVectorType;

typedef struct SampleDynVectorVTable {
  uint32_t (*len)(struct SampleDynVectorType *);
  mol2_cursor_t (*get)(struct SampleDynVectorType *, uint32_t, bool *);
} SampleDynVectorVTable;
typedef struct SampleDynVectorType {
  mol2_cursor_t cur;
  SampleDynVectorVTable *t;
} SampleDynVectorType;

typedef struct SampleUint64VectorVTable {
  uint32_t (*len)(struct SampleUint64VectorType *);
  uint64_t (*get)(struct SampleUint64VectorType *, uint32_t, bool *);
} SampleUint64VectorVTable;
typedef struct SampleUint64VectorType {
  mol2_cursor_t cur;
  SampleUint64VectorVTable *t;
} SampleUint64VectorType;

typedef struct SampleStructVTable {
  uint32_t (*u32)(struct SampleStructType *);
  mol2_cursor_t (*byte2)(struct SampleStructType *);
} SampleStructVTable;
typedef struct SampleStructType {
  mol2_cursor_t cur;
  SampleStructVTable *t;
} SampleStructType;

typedef struct SampleTableVTable {
  struct SampleDynVectorType (*byte_2d_vector)(struct SampleTableType *);
  mol2_cursor_t (*byte2)(struct SampleTableType *);
  struct SampleUint64VectorType (*u64_vector)(struct SampleTableType *);
} SampleTableVTable;
typedef struct SampleTableType {
  mol2_cursor_t cur;
  SampleTableVTable *t;
} SampleTableType;

typedef struct SampleUnionVTable {
  uint32_t (*item_id)(struct SampleUnionType *);
  struct SampleStructType (*as_SampleStruct)(struct SampleUnionType *);
  struct SampleTableType (*as_SampleTable)(struct SampleUnionType *);
} SampleUnionVTable;
typedef struct SampleUnionType {
  mol2_cursor_t cur;
  SampleUnionVTable *t;
} SampleUnionType;

typedef struct SampleOptionTableVTable {
  bool (*is_none)(struct SampleOptionTableType *);
  bool (*is_some)(struct SampleOptionTableType *);
  struct SampleTableType (*unwrap)(struct SampleOptionTableType *);
} SampleOptionTableVTable;
typedef struct SampleOptionTableType {
  mol2_cursor_t cur;
  SampleOptionTableVTable *t;
} SampleOptionTableType;

#ifndef MOLECULEC_C2_DECLARATION_ONLY

// ----implementation-------------
struct SampleByte2Type make_SampleByte2(mol2_cursor_t *cur) {
  SampleByte2Type ret;
  ret.cur = *cur;
  ret.t = GetSampleByte2VTable();
  return ret;
}
struct SampleByte2VTable *GetSampleByte2VTable(void) {
  static SampleByte2VTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.len = SampleByte2_len_impl;
  s_vtable.get = SampleByte2_get_impl;
  return &s_vtable;
}
uint32_t SampleByte2_len_impl(SampleByte2Type *this) { return 2; }
uint8_t SampleByte2_get_impl(SampleByte2Type *this, uint32_t index,
                             bool *existing) {
  uint8_t ret = {0};
  mol2_cursor_res_t res = mol2_slice_by_offset2(&this->cur, 1 * index, 1);
  if (res.errno != MOL2_OK) {
    *existing = false;
    return ret;
  } else {
    *existing = true;
  }
  ret = convert_to_Uint8(&res.cur);
  return ret;
}
struct Uint8Type make_Uint8(mol2_cursor_t *cur) {
  Uint8Type ret;
  ret.cur = *cur;
  ret.t = GetUint8VTable();
  return ret;
}
struct Uint8VTable *GetUint8VTable(void) {
  static Uint8VTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.len = Uint8_len_impl;
  s_vtable.get = Uint8_get_impl;
  return &s_vtable;
}
uint32_t Uint8_len_impl(Uint8Type *this) { return 1; }
uint8_t Uint8_get_impl(Uint8Type *this, uint32_t index, bool *existing) {
  uint8_t ret = {0};
  mol2_cursor_res_t res = mol2_slice_by_offset2(&this->cur, 1 * index, 1);
  if (res.errno != MOL2_OK) {
    *existing = false;
    return ret;
  } else {
    *existing = true;
  }
  ret = convert_to_Uint8(&res.cur);
  return ret;
}
struct Int8Type make_Int8(mol2_cursor_t *cur) {
  Int8Type ret;
  ret.cur = *cur;
  ret.t = GetInt8VTable();
  return ret;
}
struct Int8VTable *GetInt8VTable(void) {
  static Int8VTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.len = Int8_len_impl;
  s_vtable.get = Int8_get_impl;
  return &s_vtable;
}
uint32_t Int8_len_impl(Int8Type *this) { return 1; }
uint8_t Int8_get_impl(Int8Type *this, uint32_t index, bool *existing) {
  uint8_t ret = {0};
  mol2_cursor_res_t res = mol2_slice_by_offset2(&this->cur, 1 * index, 1);
  if (res.errno != MOL2_OK) {
    *existing = false;
    return ret;
  } else {
    *existing = true;
  }
  ret = convert_to_Uint8(&res.cur);
  return ret;
}
struct Uint16Type make_Uint16(mol2_cursor_t *cur) {
  Uint16Type ret;
  ret.cur = *cur;
  ret.t = GetUint16VTable();
  return ret;
}
struct Uint16VTable *GetUint16VTable(void) {
  static Uint16VTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.len = Uint16_len_impl;
  s_vtable.get = Uint16_get_impl;
  return &s_vtable;
}
uint32_t Uint16_len_impl(Uint16Type *this) { return 2; }
uint8_t Uint16_get_impl(Uint16Type *this, uint32_t index, bool *existing) {
  uint8_t ret = {0};
  mol2_cursor_res_t res = mol2_slice_by_offset2(&this->cur, 1 * index, 1);
  if (res.errno != MOL2_OK) {
    *existing = false;
    return ret;
  } else {
    *existing = true;
  }
  ret = convert_to_Uint8(&res.cur);
  return ret;
}
struct Int16Type make_Int16(mol2_cursor_t *cur) {
  Int16Type ret;
  ret.cur = *cur;
  ret.t = GetInt16VTable();
  return ret;
}
struct Int16VTable *GetInt16VTable(void) {
  static Int16VTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.len = Int16_len_impl;
  s_vtable.get = Int16_get_impl;
  return &s_vtable;
}
uint32_t Int16_len_impl(Int16Type *this) { return 2; }
uint8_t Int16_get_impl(Int16Type *this, uint32_t index, bool *existing) {
  uint8_t ret = {0};
  mol2_cursor_res_t res = mol2_slice_by_offset2(&this->cur, 1 * index, 1);
  if (res.errno != MOL2_OK) {
    *existing = false;
    return ret;
  } else {
    *existing = true;
  }
  ret = convert_to_Uint8(&res.cur);
  return ret;
}
struct Uint32Type make_Uint32(mol2_cursor_t *cur) {
  Uint32Type ret;
  ret.cur = *cur;
  ret.t = GetUint32VTable();
  return ret;
}
struct Uint32VTable *GetUint32VTable(void) {
  static Uint32VTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.len = Uint32_len_impl;
  s_vtable.get = Uint32_get_impl;
  return &s_vtable;
}
uint32_t Uint32_len_impl(Uint32Type *this) { return 4; }
uint8_t Uint32_get_impl(Uint32Type *this, uint32_t index, bool *existing) {
  uint8_t ret = {0};
  mol2_cursor_res_t res = mol2_slice_by_offset2(&this->cur, 1 * index, 1);
  if (res.errno != MOL2_OK) {
    *existing = false;
    return ret;
  } else {
    *existing = true;
  }
  ret = convert_to_Uint8(&res.cur);
  return ret;
}
struct Int32Type make_Int32(mol2_cursor_t *cur) {
  Int32Type ret;
  ret.cur = *cur;
  ret.t = GetInt32VTable();
  return ret;
}
struct Int32VTable *GetInt32VTable(void) {
  static Int32VTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.len = Int32_len_impl;
  s_vtable.get = Int32_get_impl;
  return &s_vtable;
}
uint32_t Int32_len_impl(Int32Type *this) { return 4; }
uint8_t Int32_get_impl(Int32Type *this, uint32_t index, bool *existing) {
  uint8_t ret = {0};
  mol2_cursor_res_t res = mol2_slice_by_offset2(&this->cur, 1 * index, 1);
  if (res.errno != MOL2_OK) {
    *existing = false;
    return ret;
  } else {
    *existing = true;
  }
  ret = convert_to_Uint8(&res.cur);
  return ret;
}
struct Uint64Type make_Uint64(mol2_cursor_t *cur) {
  Uint64Type ret;
  ret.cur = *cur;
  ret.t = GetUint64VTable();
  return ret;
}
struct Uint64VTable *GetUint64VTable(void) {
  static Uint64VTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.len = Uint64_len_impl;
  s_vtable.get = Uint64_get_impl;
  return &s_vtable;
}
uint32_t Uint64_len_impl(Uint64Type *this) { return 8; }
uint8_t Uint64_get_impl(Uint64Type *this, uint32_t index, bool *existing) {
  uint8_t ret = {0};
  mol2_cursor_res_t res = mol2_slice_by_offset2(&this->cur, 1 * index, 1);
  if (res.errno != MOL2_OK) {
    *existing = false;
    return ret;
  } else {
    *existing = true;
  }
  ret = convert_to_Uint8(&res.cur);
  return ret;
}
struct Int64Type make_Int64(mol2_cursor_t *cur) {
  Int64Type ret;
  ret.cur = *cur;
  ret.t = GetInt64VTable();
  return ret;
}
struct Int64VTable *GetInt64VTable(void) {
  static Int64VTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.len = Int64_len_impl;
  s_vtable.get = Int64_get_impl;
  return &s_vtable;
}
uint32_t Int64_len_impl(Int64Type *this) { return 8; }
uint8_t Int64_get_impl(Int64Type *this, uint32_t index, bool *existing) {
  uint8_t ret = {0};
  mol2_cursor_res_t res = mol2_slice_by_offset2(&this->cur, 1 * index, 1);
  if (res.errno != MOL2_OK) {
    *existing = false;
    return ret;
  } else {
    *existing = true;
  }
  ret = convert_to_Uint8(&res.cur);
  return ret;
}
struct SampleFixedVectorType make_SampleFixedVector(mol2_cursor_t *cur) {
  SampleFixedVectorType ret;
  ret.cur = *cur;
  ret.t = GetSampleFixedVectorVTable();
  return ret;
}
struct SampleFixedVectorVTable *GetSampleFixedVectorVTable(void) {
  static SampleFixedVectorVTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.len = SampleFixedVector_len_impl;
  s_vtable.get = SampleFixedVector_get_impl;
  return &s_vtable;
}
uint32_t SampleFixedVector_len_impl(SampleFixedVectorType *this) {
  return mol2_fixvec_length(&this->cur);
}
uint8_t SampleFixedVector_get_impl(SampleFixedVectorType *this, uint32_t index,
                                   bool *existing) {
  uint8_t ret = {0};
  mol2_cursor_res_t res = mol2_fixvec_slice_by_index(&this->cur, 1, index);
  if (res.errno != MOL2_OK) {
    *existing = false;
    return ret;
  } else {
    *existing = true;
  }
  ret = convert_to_Uint8(&res.cur);
  return ret;
}
struct SampleDynVectorType make_SampleDynVector(mol2_cursor_t *cur) {
  SampleDynVectorType ret;
  ret.cur = *cur;
  ret.t = GetSampleDynVectorVTable();
  return ret;
}
struct SampleDynVectorVTable *GetSampleDynVectorVTable(void) {
  static SampleDynVectorVTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.len = SampleDynVector_len_impl;
  s_vtable.get = SampleDynVector_get_impl;
  return &s_vtable;
}
uint32_t SampleDynVector_len_impl(SampleDynVectorType *this) {
  return mol2_dynvec_length(&this->cur);
}
mol2_cursor_t SampleDynVector_get_impl(SampleDynVectorType *this,
                                       uint32_t index, bool *existing) {
  mol2_cursor_t ret = {0};
  mol2_cursor_res_t res = mol2_dynvec_slice_by_index(&this->cur, index);
  if (res.errno != MOL2_OK) {
    *existing = false;
    return ret;
  } else {
    *existing = true;
  }
  return convert_to_rawbytes(&res.cur);
}
struct SampleUint64VectorType make_SampleUint64Vector(mol2_cursor_t *cur) {
  SampleUint64VectorType ret;
  ret.cur = *cur;
  ret.t = GetSampleUint64VectorVTable();
  return ret;
}
struct SampleUint64VectorVTable *GetSampleUint64VectorVTable(void) {
  static SampleUint64VectorVTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.len = SampleUint64Vector_len_impl;
  s_vtable.get = SampleUint64Vector_get_impl;
  return &s_vtable;
}
uint32_t SampleUint64Vector_len_impl(SampleUint64VectorType *this) {
  return mol2_fixvec_length(&this->cur);
}
uint64_t SampleUint64Vector_get_impl(SampleUint64VectorType *this,
                                     uint32_t index, bool *existing) {
  uint64_t ret = {0};
  mol2_cursor_res_t res = mol2_fixvec_slice_by_index(&this->cur, 8, index);
  if (res.errno != MOL2_OK) {
    *existing = false;
    return ret;
  } else {
    *existing = true;
  }
  ret = convert_to_Uint64(&res.cur);
  return ret;
}
struct SampleStructType make_SampleStruct(mol2_cursor_t *cur) {
  SampleStructType ret;
  ret.cur = *cur;
  ret.t = GetSampleStructVTable();
  return ret;
}
struct SampleStructVTable *GetSampleStructVTable(void) {
  static SampleStructVTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.u32 = SampleStruct_get_u32_impl;
  s_vtable.byte2 = SampleStruct_get_byte2_impl;
  return &s_vtable;
}
uint32_t SampleStruct_get_u32_impl(SampleStructType *this) {
  uint32_t ret;
  mol2_cursor_t ret2 = mol2_slice_by_offset(&this->cur, 0, 4);
  ret = convert_to_Uint32(&ret2);
  return ret;
}
mol2_cursor_t SampleStruct_get_byte2_impl(SampleStructType *this) {
  mol2_cursor_t ret;
  mol2_cursor_t ret2 = mol2_slice_by_offset(&this->cur, 4, 2);
  ret = convert_to_array(&ret2);
  return ret;
}
struct SampleTableType make_SampleTable(mol2_cursor_t *cur) {
  SampleTableType ret;
  ret.cur = *cur;
  ret.t = GetSampleTableVTable();
  return ret;
}
struct SampleTableVTable *GetSampleTableVTable(void) {
  static SampleTableVTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.byte_2d_vector = SampleTable_get_byte_2d_vector_impl;
  s_vtable.byte2 = SampleTable_get_byte2_impl;
  s_vtable.u64_vector = SampleTable_get_u64_vector_impl;
  return &s_vtable;
}
SampleDynVectorType SampleTable_get_byte_2d_vector_impl(SampleTableType *this) {
  SampleDynVectorType ret;
  mol2_cursor_t cur = mol2_table_slice_by_index(&this->cur, 0);
  ret.cur = cur;
  ret.t = GetSampleDynVectorVTable();
  return ret;
}
mol2_cursor_t SampleTable_get_byte2_impl(SampleTableType *this) {
  mol2_cursor_t ret;
  mol2_cursor_t ret2 = mol2_table_slice_by_index(&this->cur, 1);
  ret = convert_to_array(&ret2);
  return ret;
}
SampleUint64VectorType SampleTable_get_u64_vector_impl(SampleTableType *this) {
  SampleUint64VectorType ret;
  mol2_cursor_t cur = mol2_table_slice_by_index(&this->cur, 2);
  ret.cur = cur;
  ret.t = GetSampleUint64VectorVTable();
  return ret;
}
struct SampleUnionType make_SampleUnion(mol2_cursor_t *cur) {
  SampleUnionType ret;
  ret.cur = *cur;
  ret.t = GetSampleUnionVTable();
  return ret;
}
struct SampleUnionVTable *GetSampleUnionVTable(void) {
  static SampleUnionVTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.item_id = SampleUnion_item_id_impl;
  s_vtable.as_SampleStruct = SampleUnion_as_SampleStruct_impl;
  s_vtable.as_SampleTable = SampleUnion_as_SampleTable_impl;
  return &s_vtable;
}
uint32_t SampleUnion_item_id_impl(SampleUnionType *this) {
  return mol2_unpack_number(&this->cur);
}
SampleStructType SampleUnion_as_SampleStruct_impl(SampleUnionType *this) {
  SampleStructType ret;
  mol2_union_t u = mol2_union_unpack(&this->cur);
  ret.cur = u.cursor;
  ret.t = GetSampleStructVTable();
  return ret;
}
SampleTableType SampleUnion_as_SampleTable_impl(SampleUnionType *this) {
  SampleTableType ret;
  mol2_union_t u = mol2_union_unpack(&this->cur);
  ret.cur = u.cursor;
  ret.t = GetSampleTableVTable();
  return ret;
}
struct SampleOptionTableType make_SampleOptionTable(mol2_cursor_t *cur) {
  SampleOptionTableType ret;
  ret.cur = *cur;
  ret.t = GetSampleOptionTableVTable();
  return ret;
}
struct SampleOptionTableVTable *GetSampleOptionTableVTable(void) {
  static SampleOptionTableVTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.is_none = SampleOptionTable_is_none_impl;
  s_vtable.is_some = SampleOptionTable_is_some_impl;
  s_vtable.unwrap = SampleOptionTable_unwrap_impl;
  return &s_vtable;
}
bool SampleOptionTable_is_none_impl(SampleOptionTableType *this) {
  return mol2_option_is_none(&this->cur);
}
bool SampleOptionTable_is_some_impl(SampleOptionTableType *this) {
  return !mol2_option_is_none(&this->cur);
}
SampleTableType SampleOptionTable_unwrap_impl(SampleOptionTableType *this) {
  SampleTableType ret;
  mol2_cursor_t cur = this->cur;
  ret.cur = cur;
  ret.t = GetSampleTableVTable();
  return ret;
}
#endif  // MOLECULEC_C2_DECLARATION_ONLY

#ifdef __cplusplus
}
#endif /* __cplusplus */

#endif  // _SAMPLE_API2_H_
