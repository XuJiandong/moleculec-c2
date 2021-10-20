
#ifndef _TYPES_API2_H_
#define _TYPES_API2_H_

#define MOLECULEC2_VERSION 7002
#define MOLECULE2_API_VERSION_MIN 5000

#include "molecule2_reader.h"

#ifdef __cplusplus
extern "C" {
#endif /* __cplusplus */

// ----forward declaration--------
struct Byte2Type;
struct Byte2VTable;
struct Byte2VTable *GetByte2VTable(void);
struct Byte2Type make_Byte2(mol2_cursor_t *cur);
uint32_t Byte2_len_impl(struct Byte2Type *);
uint8_t Byte2_get_impl(struct Byte2Type *, uint32_t, bool *);
struct Byte3Type;
struct Byte3VTable;
struct Byte3VTable *GetByte3VTable(void);
struct Byte3Type make_Byte3(mol2_cursor_t *cur);
uint32_t Byte3_len_impl(struct Byte3Type *);
uint8_t Byte3_get_impl(struct Byte3Type *, uint32_t, bool *);
struct Byte4Type;
struct Byte4VTable;
struct Byte4VTable *GetByte4VTable(void);
struct Byte4Type make_Byte4(mol2_cursor_t *cur);
uint32_t Byte4_len_impl(struct Byte4Type *);
uint8_t Byte4_get_impl(struct Byte4Type *, uint32_t, bool *);
struct Byte5Type;
struct Byte5VTable;
struct Byte5VTable *GetByte5VTable(void);
struct Byte5Type make_Byte5(mol2_cursor_t *cur);
uint32_t Byte5_len_impl(struct Byte5Type *);
uint8_t Byte5_get_impl(struct Byte5Type *, uint32_t, bool *);
struct Byte6Type;
struct Byte6VTable;
struct Byte6VTable *GetByte6VTable(void);
struct Byte6Type make_Byte6(mol2_cursor_t *cur);
uint32_t Byte6_len_impl(struct Byte6Type *);
uint8_t Byte6_get_impl(struct Byte6Type *, uint32_t, bool *);
struct Byte7Type;
struct Byte7VTable;
struct Byte7VTable *GetByte7VTable(void);
struct Byte7Type make_Byte7(mol2_cursor_t *cur);
uint32_t Byte7_len_impl(struct Byte7Type *);
uint8_t Byte7_get_impl(struct Byte7Type *, uint32_t, bool *);
struct Byte8Type;
struct Byte8VTable;
struct Byte8VTable *GetByte8VTable(void);
struct Byte8Type make_Byte8(mol2_cursor_t *cur);
uint32_t Byte8_len_impl(struct Byte8Type *);
uint8_t Byte8_get_impl(struct Byte8Type *, uint32_t, bool *);
struct Byte9Type;
struct Byte9VTable;
struct Byte9VTable *GetByte9VTable(void);
struct Byte9Type make_Byte9(mol2_cursor_t *cur);
uint32_t Byte9_len_impl(struct Byte9Type *);
uint8_t Byte9_get_impl(struct Byte9Type *, uint32_t, bool *);
struct Byte10Type;
struct Byte10VTable;
struct Byte10VTable *GetByte10VTable(void);
struct Byte10Type make_Byte10(mol2_cursor_t *cur);
uint32_t Byte10_len_impl(struct Byte10Type *);
uint8_t Byte10_get_impl(struct Byte10Type *, uint32_t, bool *);
struct Byte11Type;
struct Byte11VTable;
struct Byte11VTable *GetByte11VTable(void);
struct Byte11Type make_Byte11(mol2_cursor_t *cur);
uint32_t Byte11_len_impl(struct Byte11Type *);
uint8_t Byte11_get_impl(struct Byte11Type *, uint32_t, bool *);
struct Byte12Type;
struct Byte12VTable;
struct Byte12VTable *GetByte12VTable(void);
struct Byte12Type make_Byte12(mol2_cursor_t *cur);
uint32_t Byte12_len_impl(struct Byte12Type *);
uint8_t Byte12_get_impl(struct Byte12Type *, uint32_t, bool *);
struct Byte13Type;
struct Byte13VTable;
struct Byte13VTable *GetByte13VTable(void);
struct Byte13Type make_Byte13(mol2_cursor_t *cur);
uint32_t Byte13_len_impl(struct Byte13Type *);
uint8_t Byte13_get_impl(struct Byte13Type *, uint32_t, bool *);
struct Byte14Type;
struct Byte14VTable;
struct Byte14VTable *GetByte14VTable(void);
struct Byte14Type make_Byte14(mol2_cursor_t *cur);
uint32_t Byte14_len_impl(struct Byte14Type *);
uint8_t Byte14_get_impl(struct Byte14Type *, uint32_t, bool *);
struct Byte15Type;
struct Byte15VTable;
struct Byte15VTable *GetByte15VTable(void);
struct Byte15Type make_Byte15(mol2_cursor_t *cur);
uint32_t Byte15_len_impl(struct Byte15Type *);
uint8_t Byte15_get_impl(struct Byte15Type *, uint32_t, bool *);
struct Byte16Type;
struct Byte16VTable;
struct Byte16VTable *GetByte16VTable(void);
struct Byte16Type make_Byte16(mol2_cursor_t *cur);
uint32_t Byte16_len_impl(struct Byte16Type *);
uint8_t Byte16_get_impl(struct Byte16Type *, uint32_t, bool *);
struct WordType;
struct WordVTable;
struct WordVTable *GetWordVTable(void);
struct WordType make_Word(mol2_cursor_t *cur);
uint32_t Word_len_impl(struct WordType *);
uint8_t Word_get_impl(struct WordType *, uint32_t, bool *);
struct Word2Type;
struct Word2VTable;
struct Word2VTable *GetWord2VTable(void);
struct Word2Type make_Word2(mol2_cursor_t *cur);
uint32_t Word2_len_impl(struct Word2Type *);
mol2_cursor_t Word2_get_impl(struct Word2Type *, uint32_t, bool *);
struct Word3Type;
struct Word3VTable;
struct Word3VTable *GetWord3VTable(void);
struct Word3Type make_Word3(mol2_cursor_t *cur);
uint32_t Word3_len_impl(struct Word3Type *);
mol2_cursor_t Word3_get_impl(struct Word3Type *, uint32_t, bool *);
struct Word4Type;
struct Word4VTable;
struct Word4VTable *GetWord4VTable(void);
struct Word4Type make_Word4(mol2_cursor_t *cur);
uint32_t Word4_len_impl(struct Word4Type *);
mol2_cursor_t Word4_get_impl(struct Word4Type *, uint32_t, bool *);
struct Word5Type;
struct Word5VTable;
struct Word5VTable *GetWord5VTable(void);
struct Word5Type make_Word5(mol2_cursor_t *cur);
uint32_t Word5_len_impl(struct Word5Type *);
mol2_cursor_t Word5_get_impl(struct Word5Type *, uint32_t, bool *);
struct Word6Type;
struct Word6VTable;
struct Word6VTable *GetWord6VTable(void);
struct Word6Type make_Word6(mol2_cursor_t *cur);
uint32_t Word6_len_impl(struct Word6Type *);
mol2_cursor_t Word6_get_impl(struct Word6Type *, uint32_t, bool *);
struct Word7Type;
struct Word7VTable;
struct Word7VTable *GetWord7VTable(void);
struct Word7Type make_Word7(mol2_cursor_t *cur);
uint32_t Word7_len_impl(struct Word7Type *);
mol2_cursor_t Word7_get_impl(struct Word7Type *, uint32_t, bool *);
struct Word8Type;
struct Word8VTable;
struct Word8VTable *GetWord8VTable(void);
struct Word8Type make_Word8(mol2_cursor_t *cur);
uint32_t Word8_len_impl(struct Word8Type *);
mol2_cursor_t Word8_get_impl(struct Word8Type *, uint32_t, bool *);
struct Byte3x3Type;
struct Byte3x3VTable;
struct Byte3x3VTable *GetByte3x3VTable(void);
struct Byte3x3Type make_Byte3x3(mol2_cursor_t *cur);
uint32_t Byte3x3_len_impl(struct Byte3x3Type *);
mol2_cursor_t Byte3x3_get_impl(struct Byte3x3Type *, uint32_t, bool *);
struct Byte5x3Type;
struct Byte5x3VTable;
struct Byte5x3VTable *GetByte5x3VTable(void);
struct Byte5x3Type make_Byte5x3(mol2_cursor_t *cur);
uint32_t Byte5x3_len_impl(struct Byte5x3Type *);
mol2_cursor_t Byte5x3_get_impl(struct Byte5x3Type *, uint32_t, bool *);
struct Byte7x3Type;
struct Byte7x3VTable;
struct Byte7x3VTable *GetByte7x3VTable(void);
struct Byte7x3Type make_Byte7x3(mol2_cursor_t *cur);
uint32_t Byte7x3_len_impl(struct Byte7x3Type *);
mol2_cursor_t Byte7x3_get_impl(struct Byte7x3Type *, uint32_t, bool *);
struct Byte9x3Type;
struct Byte9x3VTable;
struct Byte9x3VTable *GetByte9x3VTable(void);
struct Byte9x3Type make_Byte9x3(mol2_cursor_t *cur);
uint32_t Byte9x3_len_impl(struct Byte9x3Type *);
mol2_cursor_t Byte9x3_get_impl(struct Byte9x3Type *, uint32_t, bool *);
struct StructAType;
struct StructAVTable;
struct StructAVTable *GetStructAVTable(void);
struct StructAType make_StructA(mol2_cursor_t *cur);
uint8_t StructA_get_f1_impl(struct StructAType *);
uint8_t StructA_get_f2_impl(struct StructAType *);
mol2_cursor_t StructA_get_f3_impl(struct StructAType *);
mol2_cursor_t StructA_get_f4_impl(struct StructAType *);
struct StructBType;
struct StructBVTable;
struct StructBVTable *GetStructBVTable(void);
struct StructBType make_StructB(mol2_cursor_t *cur);
uint8_t StructB_get_f1_impl(struct StructBType *);
uint8_t StructB_get_f2_impl(struct StructBType *);
mol2_cursor_t StructB_get_f3_impl(struct StructBType *);
mol2_cursor_t StructB_get_f4_impl(struct StructBType *);
struct StructCType;
struct StructCVTable;
struct StructCVTable *GetStructCVTable(void);
struct StructCType make_StructC(mol2_cursor_t *cur);
uint8_t StructC_get_f1_impl(struct StructCType *);
uint8_t StructC_get_f2_impl(struct StructCType *);
mol2_cursor_t StructC_get_f3_impl(struct StructCType *);
mol2_cursor_t StructC_get_f4_impl(struct StructCType *);
struct StructDType;
struct StructDVTable;
struct StructDVTable *GetStructDVTable(void);
struct StructDType make_StructD(mol2_cursor_t *cur);
uint8_t StructD_get_f1_impl(struct StructDType *);
uint8_t StructD_get_f2_impl(struct StructDType *);
mol2_cursor_t StructD_get_f3_impl(struct StructDType *);
mol2_cursor_t StructD_get_f4_impl(struct StructDType *);
struct StructEType;
struct StructEVTable;
struct StructEVTable *GetStructEVTable(void);
struct StructEType make_StructE(mol2_cursor_t *cur);
uint8_t StructE_get_f1_impl(struct StructEType *);
mol2_cursor_t StructE_get_f2_impl(struct StructEType *);
uint8_t StructE_get_f3_impl(struct StructEType *);
mol2_cursor_t StructE_get_f4_impl(struct StructEType *);
struct StructFType;
struct StructFVTable;
struct StructFVTable *GetStructFVTable(void);
struct StructFType make_StructF(mol2_cursor_t *cur);
uint8_t StructF_get_f1_impl(struct StructFType *);
mol2_cursor_t StructF_get_f2_impl(struct StructFType *);
uint8_t StructF_get_f3_impl(struct StructFType *);
struct StructGType;
struct StructGVTable;
struct StructGVTable *GetStructGVTable(void);
struct StructGType make_StructG(mol2_cursor_t *cur);
mol2_cursor_t StructG_get_f1_impl(struct StructGType *);
uint8_t StructG_get_f2_impl(struct StructGType *);
mol2_cursor_t StructG_get_f3_impl(struct StructGType *);
struct Word2Type StructG_get_f4_impl(struct StructGType *);
struct StructHType;
struct StructHVTable;
struct StructHVTable *GetStructHVTable(void);
struct StructHType make_StructH(mol2_cursor_t *cur);
mol2_cursor_t StructH_get_f1_impl(struct StructHType *);
uint8_t StructH_get_f2_impl(struct StructHType *);
mol2_cursor_t StructH_get_f3_impl(struct StructHType *);
mol2_cursor_t StructH_get_f4_impl(struct StructHType *);
struct StructIType;
struct StructIVTable;
struct StructIVTable *GetStructIVTable(void);
struct StructIType make_StructI(mol2_cursor_t *cur);
mol2_cursor_t StructI_get_f1_impl(struct StructIType *);
uint8_t StructI_get_f2_impl(struct StructIType *);
struct StructJType;
struct StructJVTable;
struct StructJVTable *GetStructJVTable(void);
struct StructJType make_StructJ(mol2_cursor_t *cur);
mol2_cursor_t StructJ_get_f1_impl(struct StructJType *);
uint8_t StructJ_get_f2_impl(struct StructJType *);
struct StructIx3Type;
struct StructIx3VTable;
struct StructIx3VTable *GetStructIx3VTable(void);
struct StructIx3Type make_StructIx3(mol2_cursor_t *cur);
uint32_t StructIx3_len_impl(struct StructIx3Type *);
struct StructIType StructIx3_get_impl(struct StructIx3Type *, uint32_t, bool *);
struct StructOType;
struct StructOVTable;
struct StructOVTable *GetStructOVTable(void);
struct StructOType make_StructO(mol2_cursor_t *cur);
struct StructIx3Type StructO_get_f1_impl(struct StructOType *);
uint8_t StructO_get_f2_impl(struct StructOType *);
struct StructPType;
struct StructPVTable;
struct StructPVTable *GetStructPVTable(void);
struct StructPType make_StructP(mol2_cursor_t *cur);
struct StructJType StructP_get_f1_impl(struct StructPType *);
uint8_t StructP_get_f2_impl(struct StructPType *);
struct BytesType;
struct BytesVTable;
struct BytesVTable *GetBytesVTable(void);
struct BytesType make_Bytes(mol2_cursor_t *cur);
uint32_t Bytes_len_impl(struct BytesType *);
uint8_t Bytes_get_impl(struct BytesType *, uint32_t, bool *);
struct WordsType;
struct WordsVTable;
struct WordsVTable *GetWordsVTable(void);
struct WordsType make_Words(mol2_cursor_t *cur);
uint32_t Words_len_impl(struct WordsType *);
mol2_cursor_t Words_get_impl(struct WordsType *, uint32_t, bool *);
struct Byte3VecType;
struct Byte3VecVTable;
struct Byte3VecVTable *GetByte3VecVTable(void);
struct Byte3VecType make_Byte3Vec(mol2_cursor_t *cur);
uint32_t Byte3Vec_len_impl(struct Byte3VecType *);
mol2_cursor_t Byte3Vec_get_impl(struct Byte3VecType *, uint32_t, bool *);
struct Byte7VecType;
struct Byte7VecVTable;
struct Byte7VecVTable *GetByte7VecVTable(void);
struct Byte7VecType make_Byte7Vec(mol2_cursor_t *cur);
uint32_t Byte7Vec_len_impl(struct Byte7VecType *);
mol2_cursor_t Byte7Vec_get_impl(struct Byte7VecType *, uint32_t, bool *);
struct StructIVecType;
struct StructIVecVTable;
struct StructIVecVTable *GetStructIVecVTable(void);
struct StructIVecType make_StructIVec(mol2_cursor_t *cur);
uint32_t StructIVec_len_impl(struct StructIVecType *);
struct StructIType StructIVec_get_impl(struct StructIVecType *, uint32_t,
                                       bool *);
struct StructJVecType;
struct StructJVecVTable;
struct StructJVecVTable *GetStructJVecVTable(void);
struct StructJVecType make_StructJVec(mol2_cursor_t *cur);
uint32_t StructJVec_len_impl(struct StructJVecType *);
struct StructJType StructJVec_get_impl(struct StructJVecType *, uint32_t,
                                       bool *);
struct StructPVecType;
struct StructPVecVTable;
struct StructPVecVTable *GetStructPVecVTable(void);
struct StructPVecType make_StructPVec(mol2_cursor_t *cur);
uint32_t StructPVec_len_impl(struct StructPVecType *);
struct StructPType StructPVec_get_impl(struct StructPVecType *, uint32_t,
                                       bool *);
struct BytesVecType;
struct BytesVecVTable;
struct BytesVecVTable *GetBytesVecVTable(void);
struct BytesVecType make_BytesVec(mol2_cursor_t *cur);
uint32_t BytesVec_len_impl(struct BytesVecType *);
mol2_cursor_t BytesVec_get_impl(struct BytesVecType *, uint32_t, bool *);
struct WordsVecType;
struct WordsVecVTable;
struct WordsVecVTable *GetWordsVecVTable(void);
struct WordsVecType make_WordsVec(mol2_cursor_t *cur);
uint32_t WordsVec_len_impl(struct WordsVecType *);
struct WordsType WordsVec_get_impl(struct WordsVecType *, uint32_t, bool *);
struct Table0Type;
struct Table0VTable;
struct Table0VTable *GetTable0VTable(void);
struct Table0Type make_Table0(mol2_cursor_t *cur);
struct Table1Type;
struct Table1VTable;
struct Table1VTable *GetTable1VTable(void);
struct Table1Type make_Table1(mol2_cursor_t *cur);
uint8_t Table1_get_f1_impl(struct Table1Type *);
struct Table2Type;
struct Table2VTable;
struct Table2VTable *GetTable2VTable(void);
struct Table2Type make_Table2(mol2_cursor_t *cur);
uint8_t Table2_get_f1_impl(struct Table2Type *);
struct Word2Type Table2_get_f2_impl(struct Table2Type *);
struct Table3Type;
struct Table3VTable;
struct Table3VTable *GetTable3VTable(void);
struct Table3Type make_Table3(mol2_cursor_t *cur);
uint8_t Table3_get_f1_impl(struct Table3Type *);
struct Word2Type Table3_get_f2_impl(struct Table3Type *);
struct StructAType Table3_get_f3_impl(struct Table3Type *);
struct Table4Type;
struct Table4VTable;
struct Table4VTable *GetTable4VTable(void);
struct Table4Type make_Table4(mol2_cursor_t *cur);
uint8_t Table4_get_f1_impl(struct Table4Type *);
struct Word2Type Table4_get_f2_impl(struct Table4Type *);
struct StructAType Table4_get_f3_impl(struct Table4Type *);
mol2_cursor_t Table4_get_f4_impl(struct Table4Type *);
struct Table5Type;
struct Table5VTable;
struct Table5VTable *GetTable5VTable(void);
struct Table5Type make_Table5(mol2_cursor_t *cur);
uint8_t Table5_get_f1_impl(struct Table5Type *);
struct Word2Type Table5_get_f2_impl(struct Table5Type *);
struct StructAType Table5_get_f3_impl(struct Table5Type *);
mol2_cursor_t Table5_get_f4_impl(struct Table5Type *);
struct BytesVecType Table5_get_f5_impl(struct Table5Type *);
struct Table6Type;
struct Table6VTable;
struct Table6VTable *GetTable6VTable(void);
struct Table6Type make_Table6(mol2_cursor_t *cur);
uint8_t Table6_get_f1_impl(struct Table6Type *);
struct Word2Type Table6_get_f2_impl(struct Table6Type *);
struct StructAType Table6_get_f3_impl(struct Table6Type *);
mol2_cursor_t Table6_get_f4_impl(struct Table6Type *);
struct BytesVecType Table6_get_f5_impl(struct Table6Type *);
struct Table5Type Table6_get_f6_impl(struct Table6Type *);
struct ByteOptType;
struct ByteOptVTable;
struct ByteOptVTable *GetByteOptVTable(void);
struct ByteOptType make_ByteOpt(mol2_cursor_t *cur);
bool ByteOpt_is_none_impl(struct ByteOptType *);
bool ByteOpt_is_some_impl(struct ByteOptType *);
uint8_t ByteOpt_unwrap_impl(struct ByteOptType *);
struct WordOptType;
struct WordOptVTable;
struct WordOptVTable *GetWordOptVTable(void);
struct WordOptType make_WordOpt(mol2_cursor_t *cur);
bool WordOpt_is_none_impl(struct WordOptType *);
bool WordOpt_is_some_impl(struct WordOptType *);
mol2_cursor_t WordOpt_unwrap_impl(struct WordOptType *);
struct StructAOptType;
struct StructAOptVTable;
struct StructAOptVTable *GetStructAOptVTable(void);
struct StructAOptType make_StructAOpt(mol2_cursor_t *cur);
bool StructAOpt_is_none_impl(struct StructAOptType *);
bool StructAOpt_is_some_impl(struct StructAOptType *);
struct StructAType StructAOpt_unwrap_impl(struct StructAOptType *);
struct StructPOptType;
struct StructPOptVTable;
struct StructPOptVTable *GetStructPOptVTable(void);
struct StructPOptType make_StructPOpt(mol2_cursor_t *cur);
bool StructPOpt_is_none_impl(struct StructPOptType *);
bool StructPOpt_is_some_impl(struct StructPOptType *);
struct StructPType StructPOpt_unwrap_impl(struct StructPOptType *);
struct BytesOptType;
struct BytesOptVTable;
struct BytesOptVTable *GetBytesOptVTable(void);
struct BytesOptType make_BytesOpt(mol2_cursor_t *cur);
bool BytesOpt_is_none_impl(struct BytesOptType *);
bool BytesOpt_is_some_impl(struct BytesOptType *);
mol2_cursor_t BytesOpt_unwrap_impl(struct BytesOptType *);
struct WordsOptType;
struct WordsOptVTable;
struct WordsOptVTable *GetWordsOptVTable(void);
struct WordsOptType make_WordsOpt(mol2_cursor_t *cur);
bool WordsOpt_is_none_impl(struct WordsOptType *);
bool WordsOpt_is_some_impl(struct WordsOptType *);
struct WordsType WordsOpt_unwrap_impl(struct WordsOptType *);
struct BytesVecOptType;
struct BytesVecOptVTable;
struct BytesVecOptVTable *GetBytesVecOptVTable(void);
struct BytesVecOptType make_BytesVecOpt(mol2_cursor_t *cur);
bool BytesVecOpt_is_none_impl(struct BytesVecOptType *);
bool BytesVecOpt_is_some_impl(struct BytesVecOptType *);
struct BytesVecType BytesVecOpt_unwrap_impl(struct BytesVecOptType *);
struct WordsVecOptType;
struct WordsVecOptVTable;
struct WordsVecOptVTable *GetWordsVecOptVTable(void);
struct WordsVecOptType make_WordsVecOpt(mol2_cursor_t *cur);
bool WordsVecOpt_is_none_impl(struct WordsVecOptType *);
bool WordsVecOpt_is_some_impl(struct WordsVecOptType *);
struct WordsVecType WordsVecOpt_unwrap_impl(struct WordsVecOptType *);
struct Table0OptType;
struct Table0OptVTable;
struct Table0OptVTable *GetTable0OptVTable(void);
struct Table0OptType make_Table0Opt(mol2_cursor_t *cur);
bool Table0Opt_is_none_impl(struct Table0OptType *);
bool Table0Opt_is_some_impl(struct Table0OptType *);
struct Table0Type Table0Opt_unwrap_impl(struct Table0OptType *);
struct Table6OptType;
struct Table6OptVTable;
struct Table6OptVTable *GetTable6OptVTable(void);
struct Table6OptType make_Table6Opt(mol2_cursor_t *cur);
bool Table6Opt_is_none_impl(struct Table6OptType *);
bool Table6Opt_is_some_impl(struct Table6OptType *);
struct Table6Type Table6Opt_unwrap_impl(struct Table6OptType *);
struct Table6OptOptType;
struct Table6OptOptVTable;
struct Table6OptOptVTable *GetTable6OptOptVTable(void);
struct Table6OptOptType make_Table6OptOpt(mol2_cursor_t *cur);
bool Table6OptOpt_is_none_impl(struct Table6OptOptType *);
bool Table6OptOpt_is_some_impl(struct Table6OptOptType *);
struct Table6OptType Table6OptOpt_unwrap_impl(struct Table6OptOptType *);
struct ByteOptVecType;
struct ByteOptVecVTable;
struct ByteOptVecVTable *GetByteOptVecVTable(void);
struct ByteOptVecType make_ByteOptVec(mol2_cursor_t *cur);
uint32_t ByteOptVec_len_impl(struct ByteOptVecType *);
struct ByteOptType ByteOptVec_get_impl(struct ByteOptVecType *, uint32_t,
                                       bool *);
struct WordOptVecType;
struct WordOptVecVTable;
struct WordOptVecVTable *GetWordOptVecVTable(void);
struct WordOptVecType make_WordOptVec(mol2_cursor_t *cur);
uint32_t WordOptVec_len_impl(struct WordOptVecType *);
struct WordOptType WordOptVec_get_impl(struct WordOptVecType *, uint32_t,
                                       bool *);
struct WordsOptVecType;
struct WordsOptVecVTable;
struct WordsOptVecVTable *GetWordsOptVecVTable(void);
struct WordsOptVecType make_WordsOptVec(mol2_cursor_t *cur);
uint32_t WordsOptVec_len_impl(struct WordsOptVecType *);
struct WordsOptType WordsOptVec_get_impl(struct WordsOptVecType *, uint32_t,
                                         bool *);
struct BytesOptVecType;
struct BytesOptVecVTable;
struct BytesOptVecVTable *GetBytesOptVecVTable(void);
struct BytesOptVecType make_BytesOptVec(mol2_cursor_t *cur);
uint32_t BytesOptVec_len_impl(struct BytesOptVecType *);
struct BytesOptType BytesOptVec_get_impl(struct BytesOptVecType *, uint32_t,
                                         bool *);
struct UnionAType;
struct UnionAVTable;
struct UnionAVTable *GetUnionAVTable(void);
struct UnionAType make_UnionA(mol2_cursor_t *cur);
uint32_t UnionA_item_id_impl(struct UnionAType *);
uint8_t UnionA_as_byte_impl(struct UnionAType *);
mol2_cursor_t UnionA_as_Word_impl(struct UnionAType *);
struct StructAType UnionA_as_StructA_impl(struct UnionAType *);
mol2_cursor_t UnionA_as_Bytes_impl(struct UnionAType *);
struct WordsType UnionA_as_Words_impl(struct UnionAType *);
struct Table0Type UnionA_as_Table0_impl(struct UnionAType *);
struct Table6Type UnionA_as_Table6_impl(struct UnionAType *);
struct Table6OptType UnionA_as_Table6Opt_impl(struct UnionAType *);
struct TableAType;
struct TableAVTable;
struct TableAVTable *GetTableAVTable(void);
struct TableAType make_TableA(mol2_cursor_t *cur);
struct Word2Type TableA_get_f1_impl(struct TableAType *);
struct StructAType TableA_get_f2_impl(struct TableAType *);
mol2_cursor_t TableA_get_f3_impl(struct TableAType *);
struct BytesVecType TableA_get_f4_impl(struct TableAType *);
struct Table1Type TableA_get_f5_impl(struct TableAType *);
struct BytesOptType TableA_get_f6_impl(struct TableAType *);
struct UnionAType TableA_get_f7_impl(struct TableAType *);
uint8_t TableA_get_f8_impl(struct TableAType *);
struct AllInOneType;
struct AllInOneVTable;
struct AllInOneVTable *GetAllInOneVTable(void);
struct AllInOneType make_AllInOne(mol2_cursor_t *cur);
uint8_t AllInOne_get_f0_impl(struct AllInOneType *);
mol2_cursor_t AllInOne_get_f1_impl(struct AllInOneType *);
mol2_cursor_t AllInOne_get_f2_impl(struct AllInOneType *);
mol2_cursor_t AllInOne_get_f3_impl(struct AllInOneType *);
mol2_cursor_t AllInOne_get_f4_impl(struct AllInOneType *);
mol2_cursor_t AllInOne_get_f5_impl(struct AllInOneType *);
mol2_cursor_t AllInOne_get_f6_impl(struct AllInOneType *);
mol2_cursor_t AllInOne_get_f7_impl(struct AllInOneType *);
mol2_cursor_t AllInOne_get_f8_impl(struct AllInOneType *);
mol2_cursor_t AllInOne_get_f9_impl(struct AllInOneType *);
mol2_cursor_t AllInOne_get_f10_impl(struct AllInOneType *);
mol2_cursor_t AllInOne_get_f11_impl(struct AllInOneType *);
mol2_cursor_t AllInOne_get_f12_impl(struct AllInOneType *);
mol2_cursor_t AllInOne_get_f13_impl(struct AllInOneType *);
mol2_cursor_t AllInOne_get_f14_impl(struct AllInOneType *);
mol2_cursor_t AllInOne_get_f15_impl(struct AllInOneType *);
mol2_cursor_t AllInOne_get_f16_impl(struct AllInOneType *);
struct Word2Type AllInOne_get_f17_impl(struct AllInOneType *);
struct Word3Type AllInOne_get_f18_impl(struct AllInOneType *);
struct Word4Type AllInOne_get_f19_impl(struct AllInOneType *);
struct Word5Type AllInOne_get_f20_impl(struct AllInOneType *);
struct Word6Type AllInOne_get_f21_impl(struct AllInOneType *);
struct Word7Type AllInOne_get_f22_impl(struct AllInOneType *);
struct Word8Type AllInOne_get_f23_impl(struct AllInOneType *);
struct Byte3x3Type AllInOne_get_f24_impl(struct AllInOneType *);
struct Byte5x3Type AllInOne_get_f25_impl(struct AllInOneType *);
struct Byte7x3Type AllInOne_get_f26_impl(struct AllInOneType *);
struct Byte9x3Type AllInOne_get_f27_impl(struct AllInOneType *);
struct StructAType AllInOne_get_f28_impl(struct AllInOneType *);
struct StructBType AllInOne_get_f29_impl(struct AllInOneType *);
struct StructCType AllInOne_get_f30_impl(struct AllInOneType *);
struct StructDType AllInOne_get_f31_impl(struct AllInOneType *);
struct StructEType AllInOne_get_f32_impl(struct AllInOneType *);
struct StructFType AllInOne_get_f33_impl(struct AllInOneType *);
struct StructGType AllInOne_get_f34_impl(struct AllInOneType *);
struct StructHType AllInOne_get_f35_impl(struct AllInOneType *);
struct StructIType AllInOne_get_f36_impl(struct AllInOneType *);
struct StructJType AllInOne_get_f37_impl(struct AllInOneType *);
struct StructIx3Type AllInOne_get_f38_impl(struct AllInOneType *);
struct StructOType AllInOne_get_f39_impl(struct AllInOneType *);
struct StructPType AllInOne_get_f40_impl(struct AllInOneType *);
mol2_cursor_t AllInOne_get_f41_impl(struct AllInOneType *);
struct WordsType AllInOne_get_f42_impl(struct AllInOneType *);
struct Byte3VecType AllInOne_get_f43_impl(struct AllInOneType *);
struct Byte7VecType AllInOne_get_f44_impl(struct AllInOneType *);
struct StructIVecType AllInOne_get_f45_impl(struct AllInOneType *);
struct StructJVecType AllInOne_get_f46_impl(struct AllInOneType *);
struct StructPVecType AllInOne_get_f47_impl(struct AllInOneType *);
struct BytesVecType AllInOne_get_f48_impl(struct AllInOneType *);
struct WordsVecType AllInOne_get_f49_impl(struct AllInOneType *);
struct Table0Type AllInOne_get_f50_impl(struct AllInOneType *);
struct Table1Type AllInOne_get_f51_impl(struct AllInOneType *);
struct Table2Type AllInOne_get_f52_impl(struct AllInOneType *);
struct Table3Type AllInOne_get_f53_impl(struct AllInOneType *);
struct Table4Type AllInOne_get_f54_impl(struct AllInOneType *);
struct Table5Type AllInOne_get_f55_impl(struct AllInOneType *);
struct Table6Type AllInOne_get_f56_impl(struct AllInOneType *);
struct ByteOptType AllInOne_get_f57_impl(struct AllInOneType *);
struct WordOptType AllInOne_get_f58_impl(struct AllInOneType *);
struct StructAOptType AllInOne_get_f59_impl(struct AllInOneType *);
struct StructPOptType AllInOne_get_f60_impl(struct AllInOneType *);
struct BytesOptType AllInOne_get_f61_impl(struct AllInOneType *);
struct WordsOptType AllInOne_get_f62_impl(struct AllInOneType *);
struct BytesVecOptType AllInOne_get_f63_impl(struct AllInOneType *);
struct WordsVecOptType AllInOne_get_f64_impl(struct AllInOneType *);
struct Table0OptType AllInOne_get_f65_impl(struct AllInOneType *);
struct Table6OptType AllInOne_get_f66_impl(struct AllInOneType *);
struct Table6OptOptType AllInOne_get_f67_impl(struct AllInOneType *);
struct ByteOptVecType AllInOne_get_f68_impl(struct AllInOneType *);
struct WordOptVecType AllInOne_get_f69_impl(struct AllInOneType *);
struct WordsOptVecType AllInOne_get_f70_impl(struct AllInOneType *);
struct BytesOptVecType AllInOne_get_f71_impl(struct AllInOneType *);
struct UnionAType AllInOne_get_f72_impl(struct AllInOneType *);
struct TableAType AllInOne_get_f73_impl(struct AllInOneType *);

// ----definition-----------------
typedef struct Byte2VTable {
  uint32_t (*len)(struct Byte2Type *);
  uint8_t (*get)(struct Byte2Type *, uint32_t, bool *);
} Byte2VTable;
typedef struct Byte2Type {
  mol2_cursor_t cur;
  Byte2VTable *t;
} Byte2Type;

typedef struct Byte3VTable {
  uint32_t (*len)(struct Byte3Type *);
  uint8_t (*get)(struct Byte3Type *, uint32_t, bool *);
} Byte3VTable;
typedef struct Byte3Type {
  mol2_cursor_t cur;
  Byte3VTable *t;
} Byte3Type;

typedef struct Byte4VTable {
  uint32_t (*len)(struct Byte4Type *);
  uint8_t (*get)(struct Byte4Type *, uint32_t, bool *);
} Byte4VTable;
typedef struct Byte4Type {
  mol2_cursor_t cur;
  Byte4VTable *t;
} Byte4Type;

typedef struct Byte5VTable {
  uint32_t (*len)(struct Byte5Type *);
  uint8_t (*get)(struct Byte5Type *, uint32_t, bool *);
} Byte5VTable;
typedef struct Byte5Type {
  mol2_cursor_t cur;
  Byte5VTable *t;
} Byte5Type;

typedef struct Byte6VTable {
  uint32_t (*len)(struct Byte6Type *);
  uint8_t (*get)(struct Byte6Type *, uint32_t, bool *);
} Byte6VTable;
typedef struct Byte6Type {
  mol2_cursor_t cur;
  Byte6VTable *t;
} Byte6Type;

typedef struct Byte7VTable {
  uint32_t (*len)(struct Byte7Type *);
  uint8_t (*get)(struct Byte7Type *, uint32_t, bool *);
} Byte7VTable;
typedef struct Byte7Type {
  mol2_cursor_t cur;
  Byte7VTable *t;
} Byte7Type;

typedef struct Byte8VTable {
  uint32_t (*len)(struct Byte8Type *);
  uint8_t (*get)(struct Byte8Type *, uint32_t, bool *);
} Byte8VTable;
typedef struct Byte8Type {
  mol2_cursor_t cur;
  Byte8VTable *t;
} Byte8Type;

typedef struct Byte9VTable {
  uint32_t (*len)(struct Byte9Type *);
  uint8_t (*get)(struct Byte9Type *, uint32_t, bool *);
} Byte9VTable;
typedef struct Byte9Type {
  mol2_cursor_t cur;
  Byte9VTable *t;
} Byte9Type;

typedef struct Byte10VTable {
  uint32_t (*len)(struct Byte10Type *);
  uint8_t (*get)(struct Byte10Type *, uint32_t, bool *);
} Byte10VTable;
typedef struct Byte10Type {
  mol2_cursor_t cur;
  Byte10VTable *t;
} Byte10Type;

typedef struct Byte11VTable {
  uint32_t (*len)(struct Byte11Type *);
  uint8_t (*get)(struct Byte11Type *, uint32_t, bool *);
} Byte11VTable;
typedef struct Byte11Type {
  mol2_cursor_t cur;
  Byte11VTable *t;
} Byte11Type;

typedef struct Byte12VTable {
  uint32_t (*len)(struct Byte12Type *);
  uint8_t (*get)(struct Byte12Type *, uint32_t, bool *);
} Byte12VTable;
typedef struct Byte12Type {
  mol2_cursor_t cur;
  Byte12VTable *t;
} Byte12Type;

typedef struct Byte13VTable {
  uint32_t (*len)(struct Byte13Type *);
  uint8_t (*get)(struct Byte13Type *, uint32_t, bool *);
} Byte13VTable;
typedef struct Byte13Type {
  mol2_cursor_t cur;
  Byte13VTable *t;
} Byte13Type;

typedef struct Byte14VTable {
  uint32_t (*len)(struct Byte14Type *);
  uint8_t (*get)(struct Byte14Type *, uint32_t, bool *);
} Byte14VTable;
typedef struct Byte14Type {
  mol2_cursor_t cur;
  Byte14VTable *t;
} Byte14Type;

typedef struct Byte15VTable {
  uint32_t (*len)(struct Byte15Type *);
  uint8_t (*get)(struct Byte15Type *, uint32_t, bool *);
} Byte15VTable;
typedef struct Byte15Type {
  mol2_cursor_t cur;
  Byte15VTable *t;
} Byte15Type;

typedef struct Byte16VTable {
  uint32_t (*len)(struct Byte16Type *);
  uint8_t (*get)(struct Byte16Type *, uint32_t, bool *);
} Byte16VTable;
typedef struct Byte16Type {
  mol2_cursor_t cur;
  Byte16VTable *t;
} Byte16Type;

typedef struct WordVTable {
  uint32_t (*len)(struct WordType *);
  uint8_t (*get)(struct WordType *, uint32_t, bool *);
} WordVTable;
typedef struct WordType {
  mol2_cursor_t cur;
  WordVTable *t;
} WordType;

typedef struct Word2VTable {
  uint32_t (*len)(struct Word2Type *);
  mol2_cursor_t (*get)(struct Word2Type *, uint32_t, bool *);
} Word2VTable;
typedef struct Word2Type {
  mol2_cursor_t cur;
  Word2VTable *t;
} Word2Type;

typedef struct Word3VTable {
  uint32_t (*len)(struct Word3Type *);
  mol2_cursor_t (*get)(struct Word3Type *, uint32_t, bool *);
} Word3VTable;
typedef struct Word3Type {
  mol2_cursor_t cur;
  Word3VTable *t;
} Word3Type;

typedef struct Word4VTable {
  uint32_t (*len)(struct Word4Type *);
  mol2_cursor_t (*get)(struct Word4Type *, uint32_t, bool *);
} Word4VTable;
typedef struct Word4Type {
  mol2_cursor_t cur;
  Word4VTable *t;
} Word4Type;

typedef struct Word5VTable {
  uint32_t (*len)(struct Word5Type *);
  mol2_cursor_t (*get)(struct Word5Type *, uint32_t, bool *);
} Word5VTable;
typedef struct Word5Type {
  mol2_cursor_t cur;
  Word5VTable *t;
} Word5Type;

typedef struct Word6VTable {
  uint32_t (*len)(struct Word6Type *);
  mol2_cursor_t (*get)(struct Word6Type *, uint32_t, bool *);
} Word6VTable;
typedef struct Word6Type {
  mol2_cursor_t cur;
  Word6VTable *t;
} Word6Type;

typedef struct Word7VTable {
  uint32_t (*len)(struct Word7Type *);
  mol2_cursor_t (*get)(struct Word7Type *, uint32_t, bool *);
} Word7VTable;
typedef struct Word7Type {
  mol2_cursor_t cur;
  Word7VTable *t;
} Word7Type;

typedef struct Word8VTable {
  uint32_t (*len)(struct Word8Type *);
  mol2_cursor_t (*get)(struct Word8Type *, uint32_t, bool *);
} Word8VTable;
typedef struct Word8Type {
  mol2_cursor_t cur;
  Word8VTable *t;
} Word8Type;

typedef struct Byte3x3VTable {
  uint32_t (*len)(struct Byte3x3Type *);
  mol2_cursor_t (*get)(struct Byte3x3Type *, uint32_t, bool *);
} Byte3x3VTable;
typedef struct Byte3x3Type {
  mol2_cursor_t cur;
  Byte3x3VTable *t;
} Byte3x3Type;

typedef struct Byte5x3VTable {
  uint32_t (*len)(struct Byte5x3Type *);
  mol2_cursor_t (*get)(struct Byte5x3Type *, uint32_t, bool *);
} Byte5x3VTable;
typedef struct Byte5x3Type {
  mol2_cursor_t cur;
  Byte5x3VTable *t;
} Byte5x3Type;

typedef struct Byte7x3VTable {
  uint32_t (*len)(struct Byte7x3Type *);
  mol2_cursor_t (*get)(struct Byte7x3Type *, uint32_t, bool *);
} Byte7x3VTable;
typedef struct Byte7x3Type {
  mol2_cursor_t cur;
  Byte7x3VTable *t;
} Byte7x3Type;

typedef struct Byte9x3VTable {
  uint32_t (*len)(struct Byte9x3Type *);
  mol2_cursor_t (*get)(struct Byte9x3Type *, uint32_t, bool *);
} Byte9x3VTable;
typedef struct Byte9x3Type {
  mol2_cursor_t cur;
  Byte9x3VTable *t;
} Byte9x3Type;

typedef struct StructAVTable {
  uint8_t (*f1)(struct StructAType *);
  uint8_t (*f2)(struct StructAType *);
  mol2_cursor_t (*f3)(struct StructAType *);
  mol2_cursor_t (*f4)(struct StructAType *);
} StructAVTable;
typedef struct StructAType {
  mol2_cursor_t cur;
  StructAVTable *t;
} StructAType;

typedef struct StructBVTable {
  uint8_t (*f1)(struct StructBType *);
  uint8_t (*f2)(struct StructBType *);
  mol2_cursor_t (*f3)(struct StructBType *);
  mol2_cursor_t (*f4)(struct StructBType *);
} StructBVTable;
typedef struct StructBType {
  mol2_cursor_t cur;
  StructBVTable *t;
} StructBType;

typedef struct StructCVTable {
  uint8_t (*f1)(struct StructCType *);
  uint8_t (*f2)(struct StructCType *);
  mol2_cursor_t (*f3)(struct StructCType *);
  mol2_cursor_t (*f4)(struct StructCType *);
} StructCVTable;
typedef struct StructCType {
  mol2_cursor_t cur;
  StructCVTable *t;
} StructCType;

typedef struct StructDVTable {
  uint8_t (*f1)(struct StructDType *);
  uint8_t (*f2)(struct StructDType *);
  mol2_cursor_t (*f3)(struct StructDType *);
  mol2_cursor_t (*f4)(struct StructDType *);
} StructDVTable;
typedef struct StructDType {
  mol2_cursor_t cur;
  StructDVTable *t;
} StructDType;

typedef struct StructEVTable {
  uint8_t (*f1)(struct StructEType *);
  mol2_cursor_t (*f2)(struct StructEType *);
  uint8_t (*f3)(struct StructEType *);
  mol2_cursor_t (*f4)(struct StructEType *);
} StructEVTable;
typedef struct StructEType {
  mol2_cursor_t cur;
  StructEVTable *t;
} StructEType;

typedef struct StructFVTable {
  uint8_t (*f1)(struct StructFType *);
  mol2_cursor_t (*f2)(struct StructFType *);
  uint8_t (*f3)(struct StructFType *);
} StructFVTable;
typedef struct StructFType {
  mol2_cursor_t cur;
  StructFVTable *t;
} StructFType;

typedef struct StructGVTable {
  mol2_cursor_t (*f1)(struct StructGType *);
  uint8_t (*f2)(struct StructGType *);
  mol2_cursor_t (*f3)(struct StructGType *);
  struct Word2Type (*f4)(struct StructGType *);
} StructGVTable;
typedef struct StructGType {
  mol2_cursor_t cur;
  StructGVTable *t;
} StructGType;

typedef struct StructHVTable {
  mol2_cursor_t (*f1)(struct StructHType *);
  uint8_t (*f2)(struct StructHType *);
  mol2_cursor_t (*f3)(struct StructHType *);
  mol2_cursor_t (*f4)(struct StructHType *);
} StructHVTable;
typedef struct StructHType {
  mol2_cursor_t cur;
  StructHVTable *t;
} StructHType;

typedef struct StructIVTable {
  mol2_cursor_t (*f1)(struct StructIType *);
  uint8_t (*f2)(struct StructIType *);
} StructIVTable;
typedef struct StructIType {
  mol2_cursor_t cur;
  StructIVTable *t;
} StructIType;

typedef struct StructJVTable {
  mol2_cursor_t (*f1)(struct StructJType *);
  uint8_t (*f2)(struct StructJType *);
} StructJVTable;
typedef struct StructJType {
  mol2_cursor_t cur;
  StructJVTable *t;
} StructJType;

typedef struct StructIx3VTable {
  uint32_t (*len)(struct StructIx3Type *);
  struct StructIType (*get)(struct StructIx3Type *, uint32_t, bool *);
} StructIx3VTable;
typedef struct StructIx3Type {
  mol2_cursor_t cur;
  StructIx3VTable *t;
} StructIx3Type;

typedef struct StructOVTable {
  struct StructIx3Type (*f1)(struct StructOType *);
  uint8_t (*f2)(struct StructOType *);
} StructOVTable;
typedef struct StructOType {
  mol2_cursor_t cur;
  StructOVTable *t;
} StructOType;

typedef struct StructPVTable {
  struct StructJType (*f1)(struct StructPType *);
  uint8_t (*f2)(struct StructPType *);
} StructPVTable;
typedef struct StructPType {
  mol2_cursor_t cur;
  StructPVTable *t;
} StructPType;

typedef struct BytesVTable {
  uint32_t (*len)(struct BytesType *);
  uint8_t (*get)(struct BytesType *, uint32_t, bool *);
} BytesVTable;
typedef struct BytesType {
  mol2_cursor_t cur;
  BytesVTable *t;
} BytesType;

typedef struct WordsVTable {
  uint32_t (*len)(struct WordsType *);
  mol2_cursor_t (*get)(struct WordsType *, uint32_t, bool *);
} WordsVTable;
typedef struct WordsType {
  mol2_cursor_t cur;
  WordsVTable *t;
} WordsType;

typedef struct Byte3VecVTable {
  uint32_t (*len)(struct Byte3VecType *);
  mol2_cursor_t (*get)(struct Byte3VecType *, uint32_t, bool *);
} Byte3VecVTable;
typedef struct Byte3VecType {
  mol2_cursor_t cur;
  Byte3VecVTable *t;
} Byte3VecType;

typedef struct Byte7VecVTable {
  uint32_t (*len)(struct Byte7VecType *);
  mol2_cursor_t (*get)(struct Byte7VecType *, uint32_t, bool *);
} Byte7VecVTable;
typedef struct Byte7VecType {
  mol2_cursor_t cur;
  Byte7VecVTable *t;
} Byte7VecType;

typedef struct StructIVecVTable {
  uint32_t (*len)(struct StructIVecType *);
  struct StructIType (*get)(struct StructIVecType *, uint32_t, bool *);
} StructIVecVTable;
typedef struct StructIVecType {
  mol2_cursor_t cur;
  StructIVecVTable *t;
} StructIVecType;

typedef struct StructJVecVTable {
  uint32_t (*len)(struct StructJVecType *);
  struct StructJType (*get)(struct StructJVecType *, uint32_t, bool *);
} StructJVecVTable;
typedef struct StructJVecType {
  mol2_cursor_t cur;
  StructJVecVTable *t;
} StructJVecType;

typedef struct StructPVecVTable {
  uint32_t (*len)(struct StructPVecType *);
  struct StructPType (*get)(struct StructPVecType *, uint32_t, bool *);
} StructPVecVTable;
typedef struct StructPVecType {
  mol2_cursor_t cur;
  StructPVecVTable *t;
} StructPVecType;

typedef struct BytesVecVTable {
  uint32_t (*len)(struct BytesVecType *);
  mol2_cursor_t (*get)(struct BytesVecType *, uint32_t, bool *);
} BytesVecVTable;
typedef struct BytesVecType {
  mol2_cursor_t cur;
  BytesVecVTable *t;
} BytesVecType;

typedef struct WordsVecVTable {
  uint32_t (*len)(struct WordsVecType *);
  struct WordsType (*get)(struct WordsVecType *, uint32_t, bool *);
} WordsVecVTable;
typedef struct WordsVecType {
  mol2_cursor_t cur;
  WordsVecVTable *t;
} WordsVecType;

typedef struct Table0VTable {
} Table0VTable;
typedef struct Table0Type {
  mol2_cursor_t cur;
  Table0VTable *t;
} Table0Type;

typedef struct Table1VTable {
  uint8_t (*f1)(struct Table1Type *);
} Table1VTable;
typedef struct Table1Type {
  mol2_cursor_t cur;
  Table1VTable *t;
} Table1Type;

typedef struct Table2VTable {
  uint8_t (*f1)(struct Table2Type *);
  struct Word2Type (*f2)(struct Table2Type *);
} Table2VTable;
typedef struct Table2Type {
  mol2_cursor_t cur;
  Table2VTable *t;
} Table2Type;

typedef struct Table3VTable {
  uint8_t (*f1)(struct Table3Type *);
  struct Word2Type (*f2)(struct Table3Type *);
  struct StructAType (*f3)(struct Table3Type *);
} Table3VTable;
typedef struct Table3Type {
  mol2_cursor_t cur;
  Table3VTable *t;
} Table3Type;

typedef struct Table4VTable {
  uint8_t (*f1)(struct Table4Type *);
  struct Word2Type (*f2)(struct Table4Type *);
  struct StructAType (*f3)(struct Table4Type *);
  mol2_cursor_t (*f4)(struct Table4Type *);
} Table4VTable;
typedef struct Table4Type {
  mol2_cursor_t cur;
  Table4VTable *t;
} Table4Type;

typedef struct Table5VTable {
  uint8_t (*f1)(struct Table5Type *);
  struct Word2Type (*f2)(struct Table5Type *);
  struct StructAType (*f3)(struct Table5Type *);
  mol2_cursor_t (*f4)(struct Table5Type *);
  struct BytesVecType (*f5)(struct Table5Type *);
} Table5VTable;
typedef struct Table5Type {
  mol2_cursor_t cur;
  Table5VTable *t;
} Table5Type;

typedef struct Table6VTable {
  uint8_t (*f1)(struct Table6Type *);
  struct Word2Type (*f2)(struct Table6Type *);
  struct StructAType (*f3)(struct Table6Type *);
  mol2_cursor_t (*f4)(struct Table6Type *);
  struct BytesVecType (*f5)(struct Table6Type *);
  struct Table5Type (*f6)(struct Table6Type *);
} Table6VTable;
typedef struct Table6Type {
  mol2_cursor_t cur;
  Table6VTable *t;
} Table6Type;

typedef struct ByteOptVTable {
  bool (*is_none)(struct ByteOptType *);
  bool (*is_some)(struct ByteOptType *);
  uint8_t (*unwrap)(struct ByteOptType *);
} ByteOptVTable;
typedef struct ByteOptType {
  mol2_cursor_t cur;
  ByteOptVTable *t;
} ByteOptType;

typedef struct WordOptVTable {
  bool (*is_none)(struct WordOptType *);
  bool (*is_some)(struct WordOptType *);
  mol2_cursor_t (*unwrap)(struct WordOptType *);
} WordOptVTable;
typedef struct WordOptType {
  mol2_cursor_t cur;
  WordOptVTable *t;
} WordOptType;

typedef struct StructAOptVTable {
  bool (*is_none)(struct StructAOptType *);
  bool (*is_some)(struct StructAOptType *);
  struct StructAType (*unwrap)(struct StructAOptType *);
} StructAOptVTable;
typedef struct StructAOptType {
  mol2_cursor_t cur;
  StructAOptVTable *t;
} StructAOptType;

typedef struct StructPOptVTable {
  bool (*is_none)(struct StructPOptType *);
  bool (*is_some)(struct StructPOptType *);
  struct StructPType (*unwrap)(struct StructPOptType *);
} StructPOptVTable;
typedef struct StructPOptType {
  mol2_cursor_t cur;
  StructPOptVTable *t;
} StructPOptType;

typedef struct BytesOptVTable {
  bool (*is_none)(struct BytesOptType *);
  bool (*is_some)(struct BytesOptType *);
  mol2_cursor_t (*unwrap)(struct BytesOptType *);
} BytesOptVTable;
typedef struct BytesOptType {
  mol2_cursor_t cur;
  BytesOptVTable *t;
} BytesOptType;

typedef struct WordsOptVTable {
  bool (*is_none)(struct WordsOptType *);
  bool (*is_some)(struct WordsOptType *);
  struct WordsType (*unwrap)(struct WordsOptType *);
} WordsOptVTable;
typedef struct WordsOptType {
  mol2_cursor_t cur;
  WordsOptVTable *t;
} WordsOptType;

typedef struct BytesVecOptVTable {
  bool (*is_none)(struct BytesVecOptType *);
  bool (*is_some)(struct BytesVecOptType *);
  struct BytesVecType (*unwrap)(struct BytesVecOptType *);
} BytesVecOptVTable;
typedef struct BytesVecOptType {
  mol2_cursor_t cur;
  BytesVecOptVTable *t;
} BytesVecOptType;

typedef struct WordsVecOptVTable {
  bool (*is_none)(struct WordsVecOptType *);
  bool (*is_some)(struct WordsVecOptType *);
  struct WordsVecType (*unwrap)(struct WordsVecOptType *);
} WordsVecOptVTable;
typedef struct WordsVecOptType {
  mol2_cursor_t cur;
  WordsVecOptVTable *t;
} WordsVecOptType;

typedef struct Table0OptVTable {
  bool (*is_none)(struct Table0OptType *);
  bool (*is_some)(struct Table0OptType *);
  struct Table0Type (*unwrap)(struct Table0OptType *);
} Table0OptVTable;
typedef struct Table0OptType {
  mol2_cursor_t cur;
  Table0OptVTable *t;
} Table0OptType;

typedef struct Table6OptVTable {
  bool (*is_none)(struct Table6OptType *);
  bool (*is_some)(struct Table6OptType *);
  struct Table6Type (*unwrap)(struct Table6OptType *);
} Table6OptVTable;
typedef struct Table6OptType {
  mol2_cursor_t cur;
  Table6OptVTable *t;
} Table6OptType;

typedef struct Table6OptOptVTable {
  bool (*is_none)(struct Table6OptOptType *);
  bool (*is_some)(struct Table6OptOptType *);
  struct Table6OptType (*unwrap)(struct Table6OptOptType *);
} Table6OptOptVTable;
typedef struct Table6OptOptType {
  mol2_cursor_t cur;
  Table6OptOptVTable *t;
} Table6OptOptType;

typedef struct ByteOptVecVTable {
  uint32_t (*len)(struct ByteOptVecType *);
  struct ByteOptType (*get)(struct ByteOptVecType *, uint32_t, bool *);
} ByteOptVecVTable;
typedef struct ByteOptVecType {
  mol2_cursor_t cur;
  ByteOptVecVTable *t;
} ByteOptVecType;

typedef struct WordOptVecVTable {
  uint32_t (*len)(struct WordOptVecType *);
  struct WordOptType (*get)(struct WordOptVecType *, uint32_t, bool *);
} WordOptVecVTable;
typedef struct WordOptVecType {
  mol2_cursor_t cur;
  WordOptVecVTable *t;
} WordOptVecType;

typedef struct WordsOptVecVTable {
  uint32_t (*len)(struct WordsOptVecType *);
  struct WordsOptType (*get)(struct WordsOptVecType *, uint32_t, bool *);
} WordsOptVecVTable;
typedef struct WordsOptVecType {
  mol2_cursor_t cur;
  WordsOptVecVTable *t;
} WordsOptVecType;

typedef struct BytesOptVecVTable {
  uint32_t (*len)(struct BytesOptVecType *);
  struct BytesOptType (*get)(struct BytesOptVecType *, uint32_t, bool *);
} BytesOptVecVTable;
typedef struct BytesOptVecType {
  mol2_cursor_t cur;
  BytesOptVecVTable *t;
} BytesOptVecType;

typedef struct UnionAVTable {
  uint32_t (*item_id)(struct UnionAType *);
  uint8_t (*as_byte)(struct UnionAType *);
  mol2_cursor_t (*as_Word)(struct UnionAType *);
  struct StructAType (*as_StructA)(struct UnionAType *);
  mol2_cursor_t (*as_Bytes)(struct UnionAType *);
  struct WordsType (*as_Words)(struct UnionAType *);
  struct Table0Type (*as_Table0)(struct UnionAType *);
  struct Table6Type (*as_Table6)(struct UnionAType *);
  struct Table6OptType (*as_Table6Opt)(struct UnionAType *);
} UnionAVTable;
typedef struct UnionAType {
  mol2_cursor_t cur;
  UnionAVTable *t;
} UnionAType;

typedef struct TableAVTable {
  struct Word2Type (*f1)(struct TableAType *);
  struct StructAType (*f2)(struct TableAType *);
  mol2_cursor_t (*f3)(struct TableAType *);
  struct BytesVecType (*f4)(struct TableAType *);
  struct Table1Type (*f5)(struct TableAType *);
  struct BytesOptType (*f6)(struct TableAType *);
  struct UnionAType (*f7)(struct TableAType *);
  uint8_t (*f8)(struct TableAType *);
} TableAVTable;
typedef struct TableAType {
  mol2_cursor_t cur;
  TableAVTable *t;
} TableAType;

typedef struct AllInOneVTable {
  uint8_t (*f0)(struct AllInOneType *);
  mol2_cursor_t (*f1)(struct AllInOneType *);
  mol2_cursor_t (*f2)(struct AllInOneType *);
  mol2_cursor_t (*f3)(struct AllInOneType *);
  mol2_cursor_t (*f4)(struct AllInOneType *);
  mol2_cursor_t (*f5)(struct AllInOneType *);
  mol2_cursor_t (*f6)(struct AllInOneType *);
  mol2_cursor_t (*f7)(struct AllInOneType *);
  mol2_cursor_t (*f8)(struct AllInOneType *);
  mol2_cursor_t (*f9)(struct AllInOneType *);
  mol2_cursor_t (*f10)(struct AllInOneType *);
  mol2_cursor_t (*f11)(struct AllInOneType *);
  mol2_cursor_t (*f12)(struct AllInOneType *);
  mol2_cursor_t (*f13)(struct AllInOneType *);
  mol2_cursor_t (*f14)(struct AllInOneType *);
  mol2_cursor_t (*f15)(struct AllInOneType *);
  mol2_cursor_t (*f16)(struct AllInOneType *);
  struct Word2Type (*f17)(struct AllInOneType *);
  struct Word3Type (*f18)(struct AllInOneType *);
  struct Word4Type (*f19)(struct AllInOneType *);
  struct Word5Type (*f20)(struct AllInOneType *);
  struct Word6Type (*f21)(struct AllInOneType *);
  struct Word7Type (*f22)(struct AllInOneType *);
  struct Word8Type (*f23)(struct AllInOneType *);
  struct Byte3x3Type (*f24)(struct AllInOneType *);
  struct Byte5x3Type (*f25)(struct AllInOneType *);
  struct Byte7x3Type (*f26)(struct AllInOneType *);
  struct Byte9x3Type (*f27)(struct AllInOneType *);
  struct StructAType (*f28)(struct AllInOneType *);
  struct StructBType (*f29)(struct AllInOneType *);
  struct StructCType (*f30)(struct AllInOneType *);
  struct StructDType (*f31)(struct AllInOneType *);
  struct StructEType (*f32)(struct AllInOneType *);
  struct StructFType (*f33)(struct AllInOneType *);
  struct StructGType (*f34)(struct AllInOneType *);
  struct StructHType (*f35)(struct AllInOneType *);
  struct StructIType (*f36)(struct AllInOneType *);
  struct StructJType (*f37)(struct AllInOneType *);
  struct StructIx3Type (*f38)(struct AllInOneType *);
  struct StructOType (*f39)(struct AllInOneType *);
  struct StructPType (*f40)(struct AllInOneType *);
  mol2_cursor_t (*f41)(struct AllInOneType *);
  struct WordsType (*f42)(struct AllInOneType *);
  struct Byte3VecType (*f43)(struct AllInOneType *);
  struct Byte7VecType (*f44)(struct AllInOneType *);
  struct StructIVecType (*f45)(struct AllInOneType *);
  struct StructJVecType (*f46)(struct AllInOneType *);
  struct StructPVecType (*f47)(struct AllInOneType *);
  struct BytesVecType (*f48)(struct AllInOneType *);
  struct WordsVecType (*f49)(struct AllInOneType *);
  struct Table0Type (*f50)(struct AllInOneType *);
  struct Table1Type (*f51)(struct AllInOneType *);
  struct Table2Type (*f52)(struct AllInOneType *);
  struct Table3Type (*f53)(struct AllInOneType *);
  struct Table4Type (*f54)(struct AllInOneType *);
  struct Table5Type (*f55)(struct AllInOneType *);
  struct Table6Type (*f56)(struct AllInOneType *);
  struct ByteOptType (*f57)(struct AllInOneType *);
  struct WordOptType (*f58)(struct AllInOneType *);
  struct StructAOptType (*f59)(struct AllInOneType *);
  struct StructPOptType (*f60)(struct AllInOneType *);
  struct BytesOptType (*f61)(struct AllInOneType *);
  struct WordsOptType (*f62)(struct AllInOneType *);
  struct BytesVecOptType (*f63)(struct AllInOneType *);
  struct WordsVecOptType (*f64)(struct AllInOneType *);
  struct Table0OptType (*f65)(struct AllInOneType *);
  struct Table6OptType (*f66)(struct AllInOneType *);
  struct Table6OptOptType (*f67)(struct AllInOneType *);
  struct ByteOptVecType (*f68)(struct AllInOneType *);
  struct WordOptVecType (*f69)(struct AllInOneType *);
  struct WordsOptVecType (*f70)(struct AllInOneType *);
  struct BytesOptVecType (*f71)(struct AllInOneType *);
  struct UnionAType (*f72)(struct AllInOneType *);
  struct TableAType (*f73)(struct AllInOneType *);
} AllInOneVTable;
typedef struct AllInOneType {
  mol2_cursor_t cur;
  AllInOneVTable *t;
} AllInOneType;

#ifndef MOLECULEC_C2_DECLARATION_ONLY

// ----implementation-------------
struct Byte2Type make_Byte2(mol2_cursor_t *cur) {
  Byte2Type ret;
  ret.cur = *cur;
  ret.t = GetByte2VTable();
  return ret;
}
struct Byte2VTable *GetByte2VTable(void) {
  static Byte2VTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.len = Byte2_len_impl;
  s_vtable.get = Byte2_get_impl;
  return &s_vtable;
}
uint32_t Byte2_len_impl(Byte2Type *this) { return 2; }
uint8_t Byte2_get_impl(Byte2Type *this, uint32_t index, bool *existing) {
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
struct Byte3Type make_Byte3(mol2_cursor_t *cur) {
  Byte3Type ret;
  ret.cur = *cur;
  ret.t = GetByte3VTable();
  return ret;
}
struct Byte3VTable *GetByte3VTable(void) {
  static Byte3VTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.len = Byte3_len_impl;
  s_vtable.get = Byte3_get_impl;
  return &s_vtable;
}
uint32_t Byte3_len_impl(Byte3Type *this) { return 3; }
uint8_t Byte3_get_impl(Byte3Type *this, uint32_t index, bool *existing) {
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
struct Byte4Type make_Byte4(mol2_cursor_t *cur) {
  Byte4Type ret;
  ret.cur = *cur;
  ret.t = GetByte4VTable();
  return ret;
}
struct Byte4VTable *GetByte4VTable(void) {
  static Byte4VTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.len = Byte4_len_impl;
  s_vtable.get = Byte4_get_impl;
  return &s_vtable;
}
uint32_t Byte4_len_impl(Byte4Type *this) { return 4; }
uint8_t Byte4_get_impl(Byte4Type *this, uint32_t index, bool *existing) {
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
struct Byte5Type make_Byte5(mol2_cursor_t *cur) {
  Byte5Type ret;
  ret.cur = *cur;
  ret.t = GetByte5VTable();
  return ret;
}
struct Byte5VTable *GetByte5VTable(void) {
  static Byte5VTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.len = Byte5_len_impl;
  s_vtable.get = Byte5_get_impl;
  return &s_vtable;
}
uint32_t Byte5_len_impl(Byte5Type *this) { return 5; }
uint8_t Byte5_get_impl(Byte5Type *this, uint32_t index, bool *existing) {
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
struct Byte6Type make_Byte6(mol2_cursor_t *cur) {
  Byte6Type ret;
  ret.cur = *cur;
  ret.t = GetByte6VTable();
  return ret;
}
struct Byte6VTable *GetByte6VTable(void) {
  static Byte6VTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.len = Byte6_len_impl;
  s_vtable.get = Byte6_get_impl;
  return &s_vtable;
}
uint32_t Byte6_len_impl(Byte6Type *this) { return 6; }
uint8_t Byte6_get_impl(Byte6Type *this, uint32_t index, bool *existing) {
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
struct Byte7Type make_Byte7(mol2_cursor_t *cur) {
  Byte7Type ret;
  ret.cur = *cur;
  ret.t = GetByte7VTable();
  return ret;
}
struct Byte7VTable *GetByte7VTable(void) {
  static Byte7VTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.len = Byte7_len_impl;
  s_vtable.get = Byte7_get_impl;
  return &s_vtable;
}
uint32_t Byte7_len_impl(Byte7Type *this) { return 7; }
uint8_t Byte7_get_impl(Byte7Type *this, uint32_t index, bool *existing) {
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
struct Byte8Type make_Byte8(mol2_cursor_t *cur) {
  Byte8Type ret;
  ret.cur = *cur;
  ret.t = GetByte8VTable();
  return ret;
}
struct Byte8VTable *GetByte8VTable(void) {
  static Byte8VTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.len = Byte8_len_impl;
  s_vtable.get = Byte8_get_impl;
  return &s_vtable;
}
uint32_t Byte8_len_impl(Byte8Type *this) { return 8; }
uint8_t Byte8_get_impl(Byte8Type *this, uint32_t index, bool *existing) {
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
struct Byte9Type make_Byte9(mol2_cursor_t *cur) {
  Byte9Type ret;
  ret.cur = *cur;
  ret.t = GetByte9VTable();
  return ret;
}
struct Byte9VTable *GetByte9VTable(void) {
  static Byte9VTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.len = Byte9_len_impl;
  s_vtable.get = Byte9_get_impl;
  return &s_vtable;
}
uint32_t Byte9_len_impl(Byte9Type *this) { return 9; }
uint8_t Byte9_get_impl(Byte9Type *this, uint32_t index, bool *existing) {
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
struct Byte10Type make_Byte10(mol2_cursor_t *cur) {
  Byte10Type ret;
  ret.cur = *cur;
  ret.t = GetByte10VTable();
  return ret;
}
struct Byte10VTable *GetByte10VTable(void) {
  static Byte10VTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.len = Byte10_len_impl;
  s_vtable.get = Byte10_get_impl;
  return &s_vtable;
}
uint32_t Byte10_len_impl(Byte10Type *this) { return 10; }
uint8_t Byte10_get_impl(Byte10Type *this, uint32_t index, bool *existing) {
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
struct Byte11Type make_Byte11(mol2_cursor_t *cur) {
  Byte11Type ret;
  ret.cur = *cur;
  ret.t = GetByte11VTable();
  return ret;
}
struct Byte11VTable *GetByte11VTable(void) {
  static Byte11VTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.len = Byte11_len_impl;
  s_vtable.get = Byte11_get_impl;
  return &s_vtable;
}
uint32_t Byte11_len_impl(Byte11Type *this) { return 11; }
uint8_t Byte11_get_impl(Byte11Type *this, uint32_t index, bool *existing) {
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
struct Byte12Type make_Byte12(mol2_cursor_t *cur) {
  Byte12Type ret;
  ret.cur = *cur;
  ret.t = GetByte12VTable();
  return ret;
}
struct Byte12VTable *GetByte12VTable(void) {
  static Byte12VTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.len = Byte12_len_impl;
  s_vtable.get = Byte12_get_impl;
  return &s_vtable;
}
uint32_t Byte12_len_impl(Byte12Type *this) { return 12; }
uint8_t Byte12_get_impl(Byte12Type *this, uint32_t index, bool *existing) {
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
struct Byte13Type make_Byte13(mol2_cursor_t *cur) {
  Byte13Type ret;
  ret.cur = *cur;
  ret.t = GetByte13VTable();
  return ret;
}
struct Byte13VTable *GetByte13VTable(void) {
  static Byte13VTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.len = Byte13_len_impl;
  s_vtable.get = Byte13_get_impl;
  return &s_vtable;
}
uint32_t Byte13_len_impl(Byte13Type *this) { return 13; }
uint8_t Byte13_get_impl(Byte13Type *this, uint32_t index, bool *existing) {
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
struct Byte14Type make_Byte14(mol2_cursor_t *cur) {
  Byte14Type ret;
  ret.cur = *cur;
  ret.t = GetByte14VTable();
  return ret;
}
struct Byte14VTable *GetByte14VTable(void) {
  static Byte14VTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.len = Byte14_len_impl;
  s_vtable.get = Byte14_get_impl;
  return &s_vtable;
}
uint32_t Byte14_len_impl(Byte14Type *this) { return 14; }
uint8_t Byte14_get_impl(Byte14Type *this, uint32_t index, bool *existing) {
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
struct Byte15Type make_Byte15(mol2_cursor_t *cur) {
  Byte15Type ret;
  ret.cur = *cur;
  ret.t = GetByte15VTable();
  return ret;
}
struct Byte15VTable *GetByte15VTable(void) {
  static Byte15VTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.len = Byte15_len_impl;
  s_vtable.get = Byte15_get_impl;
  return &s_vtable;
}
uint32_t Byte15_len_impl(Byte15Type *this) { return 15; }
uint8_t Byte15_get_impl(Byte15Type *this, uint32_t index, bool *existing) {
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
struct Byte16Type make_Byte16(mol2_cursor_t *cur) {
  Byte16Type ret;
  ret.cur = *cur;
  ret.t = GetByte16VTable();
  return ret;
}
struct Byte16VTable *GetByte16VTable(void) {
  static Byte16VTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.len = Byte16_len_impl;
  s_vtable.get = Byte16_get_impl;
  return &s_vtable;
}
uint32_t Byte16_len_impl(Byte16Type *this) { return 16; }
uint8_t Byte16_get_impl(Byte16Type *this, uint32_t index, bool *existing) {
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
struct WordType make_Word(mol2_cursor_t *cur) {
  WordType ret;
  ret.cur = *cur;
  ret.t = GetWordVTable();
  return ret;
}
struct WordVTable *GetWordVTable(void) {
  static WordVTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.len = Word_len_impl;
  s_vtable.get = Word_get_impl;
  return &s_vtable;
}
uint32_t Word_len_impl(WordType *this) { return 2; }
uint8_t Word_get_impl(WordType *this, uint32_t index, bool *existing) {
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
struct Word2Type make_Word2(mol2_cursor_t *cur) {
  Word2Type ret;
  ret.cur = *cur;
  ret.t = GetWord2VTable();
  return ret;
}
struct Word2VTable *GetWord2VTable(void) {
  static Word2VTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.len = Word2_len_impl;
  s_vtable.get = Word2_get_impl;
  return &s_vtable;
}
uint32_t Word2_len_impl(Word2Type *this) { return 2; }
mol2_cursor_t Word2_get_impl(Word2Type *this, uint32_t index, bool *existing) {
  mol2_cursor_t ret = {0};
  mol2_cursor_res_t res = mol2_slice_by_offset2(&this->cur, 2 * index, 2);
  if (res.errno != MOL2_OK) {
    *existing = false;
    return ret;
  } else {
    *existing = true;
  }
  ret = convert_to_array(&res.cur);
  return ret;
}
struct Word3Type make_Word3(mol2_cursor_t *cur) {
  Word3Type ret;
  ret.cur = *cur;
  ret.t = GetWord3VTable();
  return ret;
}
struct Word3VTable *GetWord3VTable(void) {
  static Word3VTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.len = Word3_len_impl;
  s_vtable.get = Word3_get_impl;
  return &s_vtable;
}
uint32_t Word3_len_impl(Word3Type *this) { return 3; }
mol2_cursor_t Word3_get_impl(Word3Type *this, uint32_t index, bool *existing) {
  mol2_cursor_t ret = {0};
  mol2_cursor_res_t res = mol2_slice_by_offset2(&this->cur, 2 * index, 2);
  if (res.errno != MOL2_OK) {
    *existing = false;
    return ret;
  } else {
    *existing = true;
  }
  ret = convert_to_array(&res.cur);
  return ret;
}
struct Word4Type make_Word4(mol2_cursor_t *cur) {
  Word4Type ret;
  ret.cur = *cur;
  ret.t = GetWord4VTable();
  return ret;
}
struct Word4VTable *GetWord4VTable(void) {
  static Word4VTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.len = Word4_len_impl;
  s_vtable.get = Word4_get_impl;
  return &s_vtable;
}
uint32_t Word4_len_impl(Word4Type *this) { return 4; }
mol2_cursor_t Word4_get_impl(Word4Type *this, uint32_t index, bool *existing) {
  mol2_cursor_t ret = {0};
  mol2_cursor_res_t res = mol2_slice_by_offset2(&this->cur, 2 * index, 2);
  if (res.errno != MOL2_OK) {
    *existing = false;
    return ret;
  } else {
    *existing = true;
  }
  ret = convert_to_array(&res.cur);
  return ret;
}
struct Word5Type make_Word5(mol2_cursor_t *cur) {
  Word5Type ret;
  ret.cur = *cur;
  ret.t = GetWord5VTable();
  return ret;
}
struct Word5VTable *GetWord5VTable(void) {
  static Word5VTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.len = Word5_len_impl;
  s_vtable.get = Word5_get_impl;
  return &s_vtable;
}
uint32_t Word5_len_impl(Word5Type *this) { return 5; }
mol2_cursor_t Word5_get_impl(Word5Type *this, uint32_t index, bool *existing) {
  mol2_cursor_t ret = {0};
  mol2_cursor_res_t res = mol2_slice_by_offset2(&this->cur, 2 * index, 2);
  if (res.errno != MOL2_OK) {
    *existing = false;
    return ret;
  } else {
    *existing = true;
  }
  ret = convert_to_array(&res.cur);
  return ret;
}
struct Word6Type make_Word6(mol2_cursor_t *cur) {
  Word6Type ret;
  ret.cur = *cur;
  ret.t = GetWord6VTable();
  return ret;
}
struct Word6VTable *GetWord6VTable(void) {
  static Word6VTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.len = Word6_len_impl;
  s_vtable.get = Word6_get_impl;
  return &s_vtable;
}
uint32_t Word6_len_impl(Word6Type *this) { return 6; }
mol2_cursor_t Word6_get_impl(Word6Type *this, uint32_t index, bool *existing) {
  mol2_cursor_t ret = {0};
  mol2_cursor_res_t res = mol2_slice_by_offset2(&this->cur, 2 * index, 2);
  if (res.errno != MOL2_OK) {
    *existing = false;
    return ret;
  } else {
    *existing = true;
  }
  ret = convert_to_array(&res.cur);
  return ret;
}
struct Word7Type make_Word7(mol2_cursor_t *cur) {
  Word7Type ret;
  ret.cur = *cur;
  ret.t = GetWord7VTable();
  return ret;
}
struct Word7VTable *GetWord7VTable(void) {
  static Word7VTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.len = Word7_len_impl;
  s_vtable.get = Word7_get_impl;
  return &s_vtable;
}
uint32_t Word7_len_impl(Word7Type *this) { return 7; }
mol2_cursor_t Word7_get_impl(Word7Type *this, uint32_t index, bool *existing) {
  mol2_cursor_t ret = {0};
  mol2_cursor_res_t res = mol2_slice_by_offset2(&this->cur, 2 * index, 2);
  if (res.errno != MOL2_OK) {
    *existing = false;
    return ret;
  } else {
    *existing = true;
  }
  ret = convert_to_array(&res.cur);
  return ret;
}
struct Word8Type make_Word8(mol2_cursor_t *cur) {
  Word8Type ret;
  ret.cur = *cur;
  ret.t = GetWord8VTable();
  return ret;
}
struct Word8VTable *GetWord8VTable(void) {
  static Word8VTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.len = Word8_len_impl;
  s_vtable.get = Word8_get_impl;
  return &s_vtable;
}
uint32_t Word8_len_impl(Word8Type *this) { return 8; }
mol2_cursor_t Word8_get_impl(Word8Type *this, uint32_t index, bool *existing) {
  mol2_cursor_t ret = {0};
  mol2_cursor_res_t res = mol2_slice_by_offset2(&this->cur, 2 * index, 2);
  if (res.errno != MOL2_OK) {
    *existing = false;
    return ret;
  } else {
    *existing = true;
  }
  ret = convert_to_array(&res.cur);
  return ret;
}
struct Byte3x3Type make_Byte3x3(mol2_cursor_t *cur) {
  Byte3x3Type ret;
  ret.cur = *cur;
  ret.t = GetByte3x3VTable();
  return ret;
}
struct Byte3x3VTable *GetByte3x3VTable(void) {
  static Byte3x3VTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.len = Byte3x3_len_impl;
  s_vtable.get = Byte3x3_get_impl;
  return &s_vtable;
}
uint32_t Byte3x3_len_impl(Byte3x3Type *this) { return 3; }
mol2_cursor_t Byte3x3_get_impl(Byte3x3Type *this, uint32_t index,
                               bool *existing) {
  mol2_cursor_t ret = {0};
  mol2_cursor_res_t res = mol2_slice_by_offset2(&this->cur, 3 * index, 3);
  if (res.errno != MOL2_OK) {
    *existing = false;
    return ret;
  } else {
    *existing = true;
  }
  ret = convert_to_array(&res.cur);
  return ret;
}
struct Byte5x3Type make_Byte5x3(mol2_cursor_t *cur) {
  Byte5x3Type ret;
  ret.cur = *cur;
  ret.t = GetByte5x3VTable();
  return ret;
}
struct Byte5x3VTable *GetByte5x3VTable(void) {
  static Byte5x3VTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.len = Byte5x3_len_impl;
  s_vtable.get = Byte5x3_get_impl;
  return &s_vtable;
}
uint32_t Byte5x3_len_impl(Byte5x3Type *this) { return 3; }
mol2_cursor_t Byte5x3_get_impl(Byte5x3Type *this, uint32_t index,
                               bool *existing) {
  mol2_cursor_t ret = {0};
  mol2_cursor_res_t res = mol2_slice_by_offset2(&this->cur, 5 * index, 5);
  if (res.errno != MOL2_OK) {
    *existing = false;
    return ret;
  } else {
    *existing = true;
  }
  ret = convert_to_array(&res.cur);
  return ret;
}
struct Byte7x3Type make_Byte7x3(mol2_cursor_t *cur) {
  Byte7x3Type ret;
  ret.cur = *cur;
  ret.t = GetByte7x3VTable();
  return ret;
}
struct Byte7x3VTable *GetByte7x3VTable(void) {
  static Byte7x3VTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.len = Byte7x3_len_impl;
  s_vtable.get = Byte7x3_get_impl;
  return &s_vtable;
}
uint32_t Byte7x3_len_impl(Byte7x3Type *this) { return 3; }
mol2_cursor_t Byte7x3_get_impl(Byte7x3Type *this, uint32_t index,
                               bool *existing) {
  mol2_cursor_t ret = {0};
  mol2_cursor_res_t res = mol2_slice_by_offset2(&this->cur, 7 * index, 7);
  if (res.errno != MOL2_OK) {
    *existing = false;
    return ret;
  } else {
    *existing = true;
  }
  ret = convert_to_array(&res.cur);
  return ret;
}
struct Byte9x3Type make_Byte9x3(mol2_cursor_t *cur) {
  Byte9x3Type ret;
  ret.cur = *cur;
  ret.t = GetByte9x3VTable();
  return ret;
}
struct Byte9x3VTable *GetByte9x3VTable(void) {
  static Byte9x3VTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.len = Byte9x3_len_impl;
  s_vtable.get = Byte9x3_get_impl;
  return &s_vtable;
}
uint32_t Byte9x3_len_impl(Byte9x3Type *this) { return 3; }
mol2_cursor_t Byte9x3_get_impl(Byte9x3Type *this, uint32_t index,
                               bool *existing) {
  mol2_cursor_t ret = {0};
  mol2_cursor_res_t res = mol2_slice_by_offset2(&this->cur, 9 * index, 9);
  if (res.errno != MOL2_OK) {
    *existing = false;
    return ret;
  } else {
    *existing = true;
  }
  ret = convert_to_array(&res.cur);
  return ret;
}
struct StructAType make_StructA(mol2_cursor_t *cur) {
  StructAType ret;
  ret.cur = *cur;
  ret.t = GetStructAVTable();
  return ret;
}
struct StructAVTable *GetStructAVTable(void) {
  static StructAVTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.f1 = StructA_get_f1_impl;
  s_vtable.f2 = StructA_get_f2_impl;
  s_vtable.f3 = StructA_get_f3_impl;
  s_vtable.f4 = StructA_get_f4_impl;
  return &s_vtable;
}
uint8_t StructA_get_f1_impl(StructAType *this) {
  uint8_t ret;
  mol2_cursor_t ret2 = mol2_slice_by_offset(&this->cur, 0, 1);
  ret = convert_to_Uint8(&ret2);
  return ret;
}
uint8_t StructA_get_f2_impl(StructAType *this) {
  uint8_t ret;
  mol2_cursor_t ret2 = mol2_slice_by_offset(&this->cur, 1, 1);
  ret = convert_to_Uint8(&ret2);
  return ret;
}
mol2_cursor_t StructA_get_f3_impl(StructAType *this) {
  mol2_cursor_t ret;
  mol2_cursor_t ret2 = mol2_slice_by_offset(&this->cur, 2, 2);
  ret = convert_to_array(&ret2);
  return ret;
}
mol2_cursor_t StructA_get_f4_impl(StructAType *this) {
  mol2_cursor_t ret;
  mol2_cursor_t ret2 = mol2_slice_by_offset(&this->cur, 4, 2);
  ret = convert_to_array(&ret2);
  return ret;
}
struct StructBType make_StructB(mol2_cursor_t *cur) {
  StructBType ret;
  ret.cur = *cur;
  ret.t = GetStructBVTable();
  return ret;
}
struct StructBVTable *GetStructBVTable(void) {
  static StructBVTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.f1 = StructB_get_f1_impl;
  s_vtable.f2 = StructB_get_f2_impl;
  s_vtable.f3 = StructB_get_f3_impl;
  s_vtable.f4 = StructB_get_f4_impl;
  return &s_vtable;
}
uint8_t StructB_get_f1_impl(StructBType *this) {
  uint8_t ret;
  mol2_cursor_t ret2 = mol2_slice_by_offset(&this->cur, 0, 1);
  ret = convert_to_Uint8(&ret2);
  return ret;
}
uint8_t StructB_get_f2_impl(StructBType *this) {
  uint8_t ret;
  mol2_cursor_t ret2 = mol2_slice_by_offset(&this->cur, 1, 1);
  ret = convert_to_Uint8(&ret2);
  return ret;
}
mol2_cursor_t StructB_get_f3_impl(StructBType *this) {
  mol2_cursor_t ret;
  mol2_cursor_t ret2 = mol2_slice_by_offset(&this->cur, 2, 2);
  ret = convert_to_array(&ret2);
  return ret;
}
mol2_cursor_t StructB_get_f4_impl(StructBType *this) {
  mol2_cursor_t ret;
  mol2_cursor_t ret2 = mol2_slice_by_offset(&this->cur, 4, 3);
  ret = convert_to_array(&ret2);
  return ret;
}
struct StructCType make_StructC(mol2_cursor_t *cur) {
  StructCType ret;
  ret.cur = *cur;
  ret.t = GetStructCVTable();
  return ret;
}
struct StructCVTable *GetStructCVTable(void) {
  static StructCVTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.f1 = StructC_get_f1_impl;
  s_vtable.f2 = StructC_get_f2_impl;
  s_vtable.f3 = StructC_get_f3_impl;
  s_vtable.f4 = StructC_get_f4_impl;
  return &s_vtable;
}
uint8_t StructC_get_f1_impl(StructCType *this) {
  uint8_t ret;
  mol2_cursor_t ret2 = mol2_slice_by_offset(&this->cur, 0, 1);
  ret = convert_to_Uint8(&ret2);
  return ret;
}
uint8_t StructC_get_f2_impl(StructCType *this) {
  uint8_t ret;
  mol2_cursor_t ret2 = mol2_slice_by_offset(&this->cur, 1, 1);
  ret = convert_to_Uint8(&ret2);
  return ret;
}
mol2_cursor_t StructC_get_f3_impl(StructCType *this) {
  mol2_cursor_t ret;
  mol2_cursor_t ret2 = mol2_slice_by_offset(&this->cur, 2, 2);
  ret = convert_to_array(&ret2);
  return ret;
}
mol2_cursor_t StructC_get_f4_impl(StructCType *this) {
  mol2_cursor_t ret;
  mol2_cursor_t ret2 = mol2_slice_by_offset(&this->cur, 4, 4);
  ret = convert_to_array(&ret2);
  return ret;
}
struct StructDType make_StructD(mol2_cursor_t *cur) {
  StructDType ret;
  ret.cur = *cur;
  ret.t = GetStructDVTable();
  return ret;
}
struct StructDVTable *GetStructDVTable(void) {
  static StructDVTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.f1 = StructD_get_f1_impl;
  s_vtable.f2 = StructD_get_f2_impl;
  s_vtable.f3 = StructD_get_f3_impl;
  s_vtable.f4 = StructD_get_f4_impl;
  return &s_vtable;
}
uint8_t StructD_get_f1_impl(StructDType *this) {
  uint8_t ret;
  mol2_cursor_t ret2 = mol2_slice_by_offset(&this->cur, 0, 1);
  ret = convert_to_Uint8(&ret2);
  return ret;
}
uint8_t StructD_get_f2_impl(StructDType *this) {
  uint8_t ret;
  mol2_cursor_t ret2 = mol2_slice_by_offset(&this->cur, 1, 1);
  ret = convert_to_Uint8(&ret2);
  return ret;
}
mol2_cursor_t StructD_get_f3_impl(StructDType *this) {
  mol2_cursor_t ret;
  mol2_cursor_t ret2 = mol2_slice_by_offset(&this->cur, 2, 2);
  ret = convert_to_array(&ret2);
  return ret;
}
mol2_cursor_t StructD_get_f4_impl(StructDType *this) {
  mol2_cursor_t ret;
  mol2_cursor_t ret2 = mol2_slice_by_offset(&this->cur, 4, 5);
  ret = convert_to_array(&ret2);
  return ret;
}
struct StructEType make_StructE(mol2_cursor_t *cur) {
  StructEType ret;
  ret.cur = *cur;
  ret.t = GetStructEVTable();
  return ret;
}
struct StructEVTable *GetStructEVTable(void) {
  static StructEVTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.f1 = StructE_get_f1_impl;
  s_vtable.f2 = StructE_get_f2_impl;
  s_vtable.f3 = StructE_get_f3_impl;
  s_vtable.f4 = StructE_get_f4_impl;
  return &s_vtable;
}
uint8_t StructE_get_f1_impl(StructEType *this) {
  uint8_t ret;
  mol2_cursor_t ret2 = mol2_slice_by_offset(&this->cur, 0, 1);
  ret = convert_to_Uint8(&ret2);
  return ret;
}
mol2_cursor_t StructE_get_f2_impl(StructEType *this) {
  mol2_cursor_t ret;
  mol2_cursor_t ret2 = mol2_slice_by_offset(&this->cur, 1, 2);
  ret = convert_to_array(&ret2);
  return ret;
}
uint8_t StructE_get_f3_impl(StructEType *this) {
  uint8_t ret;
  mol2_cursor_t ret2 = mol2_slice_by_offset(&this->cur, 3, 1);
  ret = convert_to_Uint8(&ret2);
  return ret;
}
mol2_cursor_t StructE_get_f4_impl(StructEType *this) {
  mol2_cursor_t ret;
  mol2_cursor_t ret2 = mol2_slice_by_offset(&this->cur, 4, 2);
  ret = convert_to_array(&ret2);
  return ret;
}
struct StructFType make_StructF(mol2_cursor_t *cur) {
  StructFType ret;
  ret.cur = *cur;
  ret.t = GetStructFVTable();
  return ret;
}
struct StructFVTable *GetStructFVTable(void) {
  static StructFVTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.f1 = StructF_get_f1_impl;
  s_vtable.f2 = StructF_get_f2_impl;
  s_vtable.f3 = StructF_get_f3_impl;
  return &s_vtable;
}
uint8_t StructF_get_f1_impl(StructFType *this) {
  uint8_t ret;
  mol2_cursor_t ret2 = mol2_slice_by_offset(&this->cur, 0, 1);
  ret = convert_to_Uint8(&ret2);
  return ret;
}
mol2_cursor_t StructF_get_f2_impl(StructFType *this) {
  mol2_cursor_t ret;
  mol2_cursor_t ret2 = mol2_slice_by_offset(&this->cur, 1, 3);
  ret = convert_to_array(&ret2);
  return ret;
}
uint8_t StructF_get_f3_impl(StructFType *this) {
  uint8_t ret;
  mol2_cursor_t ret2 = mol2_slice_by_offset(&this->cur, 4, 1);
  ret = convert_to_Uint8(&ret2);
  return ret;
}
struct StructGType make_StructG(mol2_cursor_t *cur) {
  StructGType ret;
  ret.cur = *cur;
  ret.t = GetStructGVTable();
  return ret;
}
struct StructGVTable *GetStructGVTable(void) {
  static StructGVTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.f1 = StructG_get_f1_impl;
  s_vtable.f2 = StructG_get_f2_impl;
  s_vtable.f3 = StructG_get_f3_impl;
  s_vtable.f4 = StructG_get_f4_impl;
  return &s_vtable;
}
mol2_cursor_t StructG_get_f1_impl(StructGType *this) {
  mol2_cursor_t ret;
  mol2_cursor_t ret2 = mol2_slice_by_offset(&this->cur, 0, 3);
  ret = convert_to_array(&ret2);
  return ret;
}
uint8_t StructG_get_f2_impl(StructGType *this) {
  uint8_t ret;
  mol2_cursor_t ret2 = mol2_slice_by_offset(&this->cur, 3, 1);
  ret = convert_to_Uint8(&ret2);
  return ret;
}
mol2_cursor_t StructG_get_f3_impl(StructGType *this) {
  mol2_cursor_t ret;
  mol2_cursor_t ret2 = mol2_slice_by_offset(&this->cur, 4, 2);
  ret = convert_to_array(&ret2);
  return ret;
}
Word2Type StructG_get_f4_impl(StructGType *this) {
  Word2Type ret;
  mol2_cursor_t cur = mol2_slice_by_offset(&this->cur, 6, 4);
  ret.cur = cur;
  ret.t = GetWord2VTable();
  return ret;
}
struct StructHType make_StructH(mol2_cursor_t *cur) {
  StructHType ret;
  ret.cur = *cur;
  ret.t = GetStructHVTable();
  return ret;
}
struct StructHVTable *GetStructHVTable(void) {
  static StructHVTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.f1 = StructH_get_f1_impl;
  s_vtable.f2 = StructH_get_f2_impl;
  s_vtable.f3 = StructH_get_f3_impl;
  s_vtable.f4 = StructH_get_f4_impl;
  return &s_vtable;
}
mol2_cursor_t StructH_get_f1_impl(StructHType *this) {
  mol2_cursor_t ret;
  mol2_cursor_t ret2 = mol2_slice_by_offset(&this->cur, 0, 3);
  ret = convert_to_array(&ret2);
  return ret;
}
uint8_t StructH_get_f2_impl(StructHType *this) {
  uint8_t ret;
  mol2_cursor_t ret2 = mol2_slice_by_offset(&this->cur, 3, 1);
  ret = convert_to_Uint8(&ret2);
  return ret;
}
mol2_cursor_t StructH_get_f3_impl(StructHType *this) {
  mol2_cursor_t ret;
  mol2_cursor_t ret2 = mol2_slice_by_offset(&this->cur, 4, 2);
  ret = convert_to_array(&ret2);
  return ret;
}
mol2_cursor_t StructH_get_f4_impl(StructHType *this) {
  mol2_cursor_t ret;
  mol2_cursor_t ret2 = mol2_slice_by_offset(&this->cur, 6, 4);
  ret = convert_to_array(&ret2);
  return ret;
}
struct StructIType make_StructI(mol2_cursor_t *cur) {
  StructIType ret;
  ret.cur = *cur;
  ret.t = GetStructIVTable();
  return ret;
}
struct StructIVTable *GetStructIVTable(void) {
  static StructIVTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.f1 = StructI_get_f1_impl;
  s_vtable.f2 = StructI_get_f2_impl;
  return &s_vtable;
}
mol2_cursor_t StructI_get_f1_impl(StructIType *this) {
  mol2_cursor_t ret;
  mol2_cursor_t ret2 = mol2_slice_by_offset(&this->cur, 0, 3);
  ret = convert_to_array(&ret2);
  return ret;
}
uint8_t StructI_get_f2_impl(StructIType *this) {
  uint8_t ret;
  mol2_cursor_t ret2 = mol2_slice_by_offset(&this->cur, 3, 1);
  ret = convert_to_Uint8(&ret2);
  return ret;
}
struct StructJType make_StructJ(mol2_cursor_t *cur) {
  StructJType ret;
  ret.cur = *cur;
  ret.t = GetStructJVTable();
  return ret;
}
struct StructJVTable *GetStructJVTable(void) {
  static StructJVTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.f1 = StructJ_get_f1_impl;
  s_vtable.f2 = StructJ_get_f2_impl;
  return &s_vtable;
}
mol2_cursor_t StructJ_get_f1_impl(StructJType *this) {
  mol2_cursor_t ret;
  mol2_cursor_t ret2 = mol2_slice_by_offset(&this->cur, 0, 6);
  ret = convert_to_array(&ret2);
  return ret;
}
uint8_t StructJ_get_f2_impl(StructJType *this) {
  uint8_t ret;
  mol2_cursor_t ret2 = mol2_slice_by_offset(&this->cur, 6, 1);
  ret = convert_to_Uint8(&ret2);
  return ret;
}
struct StructIx3Type make_StructIx3(mol2_cursor_t *cur) {
  StructIx3Type ret;
  ret.cur = *cur;
  ret.t = GetStructIx3VTable();
  return ret;
}
struct StructIx3VTable *GetStructIx3VTable(void) {
  static StructIx3VTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.len = StructIx3_len_impl;
  s_vtable.get = StructIx3_get_impl;
  return &s_vtable;
}
uint32_t StructIx3_len_impl(StructIx3Type *this) { return 3; }
StructIType StructIx3_get_impl(StructIx3Type *this, uint32_t index,
                               bool *existing) {
  StructIType ret = {0};
  mol2_cursor_res_t res = mol2_slice_by_offset2(&this->cur, 4 * index, 4);
  if (res.errno != MOL2_OK) {
    *existing = false;
    return ret;
  } else {
    *existing = true;
  }
  ret.cur = res.cur;
  ret.t = GetStructIVTable();
  return ret;
}
struct StructOType make_StructO(mol2_cursor_t *cur) {
  StructOType ret;
  ret.cur = *cur;
  ret.t = GetStructOVTable();
  return ret;
}
struct StructOVTable *GetStructOVTable(void) {
  static StructOVTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.f1 = StructO_get_f1_impl;
  s_vtable.f2 = StructO_get_f2_impl;
  return &s_vtable;
}
StructIx3Type StructO_get_f1_impl(StructOType *this) {
  StructIx3Type ret;
  mol2_cursor_t cur = mol2_slice_by_offset(&this->cur, 0, 12);
  ret.cur = cur;
  ret.t = GetStructIx3VTable();
  return ret;
}
uint8_t StructO_get_f2_impl(StructOType *this) {
  uint8_t ret;
  mol2_cursor_t ret2 = mol2_slice_by_offset(&this->cur, 12, 1);
  ret = convert_to_Uint8(&ret2);
  return ret;
}
struct StructPType make_StructP(mol2_cursor_t *cur) {
  StructPType ret;
  ret.cur = *cur;
  ret.t = GetStructPVTable();
  return ret;
}
struct StructPVTable *GetStructPVTable(void) {
  static StructPVTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.f1 = StructP_get_f1_impl;
  s_vtable.f2 = StructP_get_f2_impl;
  return &s_vtable;
}
StructJType StructP_get_f1_impl(StructPType *this) {
  StructJType ret;
  mol2_cursor_t cur = mol2_slice_by_offset(&this->cur, 0, 7);
  ret.cur = cur;
  ret.t = GetStructJVTable();
  return ret;
}
uint8_t StructP_get_f2_impl(StructPType *this) {
  uint8_t ret;
  mol2_cursor_t ret2 = mol2_slice_by_offset(&this->cur, 7, 1);
  ret = convert_to_Uint8(&ret2);
  return ret;
}
struct BytesType make_Bytes(mol2_cursor_t *cur) {
  BytesType ret;
  ret.cur = *cur;
  ret.t = GetBytesVTable();
  return ret;
}
struct BytesVTable *GetBytesVTable(void) {
  static BytesVTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.len = Bytes_len_impl;
  s_vtable.get = Bytes_get_impl;
  return &s_vtable;
}
uint32_t Bytes_len_impl(BytesType *this) {
  return mol2_fixvec_length(&this->cur);
}
uint8_t Bytes_get_impl(BytesType *this, uint32_t index, bool *existing) {
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
struct WordsType make_Words(mol2_cursor_t *cur) {
  WordsType ret;
  ret.cur = *cur;
  ret.t = GetWordsVTable();
  return ret;
}
struct WordsVTable *GetWordsVTable(void) {
  static WordsVTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.len = Words_len_impl;
  s_vtable.get = Words_get_impl;
  return &s_vtable;
}
uint32_t Words_len_impl(WordsType *this) {
  return mol2_fixvec_length(&this->cur);
}
mol2_cursor_t Words_get_impl(WordsType *this, uint32_t index, bool *existing) {
  mol2_cursor_t ret = {0};
  mol2_cursor_res_t res = mol2_fixvec_slice_by_index(&this->cur, 2, index);
  if (res.errno != MOL2_OK) {
    *existing = false;
    return ret;
  } else {
    *existing = true;
  }
  ret = convert_to_array(&res.cur);
  return ret;
}
struct Byte3VecType make_Byte3Vec(mol2_cursor_t *cur) {
  Byte3VecType ret;
  ret.cur = *cur;
  ret.t = GetByte3VecVTable();
  return ret;
}
struct Byte3VecVTable *GetByte3VecVTable(void) {
  static Byte3VecVTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.len = Byte3Vec_len_impl;
  s_vtable.get = Byte3Vec_get_impl;
  return &s_vtable;
}
uint32_t Byte3Vec_len_impl(Byte3VecType *this) {
  return mol2_fixvec_length(&this->cur);
}
mol2_cursor_t Byte3Vec_get_impl(Byte3VecType *this, uint32_t index,
                                bool *existing) {
  mol2_cursor_t ret = {0};
  mol2_cursor_res_t res = mol2_fixvec_slice_by_index(&this->cur, 3, index);
  if (res.errno != MOL2_OK) {
    *existing = false;
    return ret;
  } else {
    *existing = true;
  }
  ret = convert_to_array(&res.cur);
  return ret;
}
struct Byte7VecType make_Byte7Vec(mol2_cursor_t *cur) {
  Byte7VecType ret;
  ret.cur = *cur;
  ret.t = GetByte7VecVTable();
  return ret;
}
struct Byte7VecVTable *GetByte7VecVTable(void) {
  static Byte7VecVTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.len = Byte7Vec_len_impl;
  s_vtable.get = Byte7Vec_get_impl;
  return &s_vtable;
}
uint32_t Byte7Vec_len_impl(Byte7VecType *this) {
  return mol2_fixvec_length(&this->cur);
}
mol2_cursor_t Byte7Vec_get_impl(Byte7VecType *this, uint32_t index,
                                bool *existing) {
  mol2_cursor_t ret = {0};
  mol2_cursor_res_t res = mol2_fixvec_slice_by_index(&this->cur, 7, index);
  if (res.errno != MOL2_OK) {
    *existing = false;
    return ret;
  } else {
    *existing = true;
  }
  ret = convert_to_array(&res.cur);
  return ret;
}
struct StructIVecType make_StructIVec(mol2_cursor_t *cur) {
  StructIVecType ret;
  ret.cur = *cur;
  ret.t = GetStructIVecVTable();
  return ret;
}
struct StructIVecVTable *GetStructIVecVTable(void) {
  static StructIVecVTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.len = StructIVec_len_impl;
  s_vtable.get = StructIVec_get_impl;
  return &s_vtable;
}
uint32_t StructIVec_len_impl(StructIVecType *this) {
  return mol2_fixvec_length(&this->cur);
}
StructIType StructIVec_get_impl(StructIVecType *this, uint32_t index,
                                bool *existing) {
  StructIType ret = {0};
  mol2_cursor_res_t res = mol2_fixvec_slice_by_index(&this->cur, 4, index);
  if (res.errno != MOL2_OK) {
    *existing = false;
    return ret;
  } else {
    *existing = true;
  }
  ret.cur = res.cur;
  ret.t = GetStructIVTable();
  return ret;
}
struct StructJVecType make_StructJVec(mol2_cursor_t *cur) {
  StructJVecType ret;
  ret.cur = *cur;
  ret.t = GetStructJVecVTable();
  return ret;
}
struct StructJVecVTable *GetStructJVecVTable(void) {
  static StructJVecVTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.len = StructJVec_len_impl;
  s_vtable.get = StructJVec_get_impl;
  return &s_vtable;
}
uint32_t StructJVec_len_impl(StructJVecType *this) {
  return mol2_fixvec_length(&this->cur);
}
StructJType StructJVec_get_impl(StructJVecType *this, uint32_t index,
                                bool *existing) {
  StructJType ret = {0};
  mol2_cursor_res_t res = mol2_fixvec_slice_by_index(&this->cur, 7, index);
  if (res.errno != MOL2_OK) {
    *existing = false;
    return ret;
  } else {
    *existing = true;
  }
  ret.cur = res.cur;
  ret.t = GetStructJVTable();
  return ret;
}
struct StructPVecType make_StructPVec(mol2_cursor_t *cur) {
  StructPVecType ret;
  ret.cur = *cur;
  ret.t = GetStructPVecVTable();
  return ret;
}
struct StructPVecVTable *GetStructPVecVTable(void) {
  static StructPVecVTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.len = StructPVec_len_impl;
  s_vtable.get = StructPVec_get_impl;
  return &s_vtable;
}
uint32_t StructPVec_len_impl(StructPVecType *this) {
  return mol2_fixvec_length(&this->cur);
}
StructPType StructPVec_get_impl(StructPVecType *this, uint32_t index,
                                bool *existing) {
  StructPType ret = {0};
  mol2_cursor_res_t res = mol2_fixvec_slice_by_index(&this->cur, 8, index);
  if (res.errno != MOL2_OK) {
    *existing = false;
    return ret;
  } else {
    *existing = true;
  }
  ret.cur = res.cur;
  ret.t = GetStructPVTable();
  return ret;
}
struct BytesVecType make_BytesVec(mol2_cursor_t *cur) {
  BytesVecType ret;
  ret.cur = *cur;
  ret.t = GetBytesVecVTable();
  return ret;
}
struct BytesVecVTable *GetBytesVecVTable(void) {
  static BytesVecVTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.len = BytesVec_len_impl;
  s_vtable.get = BytesVec_get_impl;
  return &s_vtable;
}
uint32_t BytesVec_len_impl(BytesVecType *this) {
  return mol2_dynvec_length(&this->cur);
}
mol2_cursor_t BytesVec_get_impl(BytesVecType *this, uint32_t index,
                                bool *existing) {
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
struct WordsVecType make_WordsVec(mol2_cursor_t *cur) {
  WordsVecType ret;
  ret.cur = *cur;
  ret.t = GetWordsVecVTable();
  return ret;
}
struct WordsVecVTable *GetWordsVecVTable(void) {
  static WordsVecVTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.len = WordsVec_len_impl;
  s_vtable.get = WordsVec_get_impl;
  return &s_vtable;
}
uint32_t WordsVec_len_impl(WordsVecType *this) {
  return mol2_dynvec_length(&this->cur);
}
WordsType WordsVec_get_impl(WordsVecType *this, uint32_t index,
                            bool *existing) {
  WordsType ret = {0};
  mol2_cursor_res_t res = mol2_dynvec_slice_by_index(&this->cur, index);
  if (res.errno != MOL2_OK) {
    *existing = false;
    return ret;
  } else {
    *existing = true;
  }
  ret.cur = res.cur;
  ret.t = GetWordsVTable();
  return ret;
}
struct Table0Type make_Table0(mol2_cursor_t *cur) {
  Table0Type ret;
  ret.cur = *cur;
  ret.t = GetTable0VTable();
  return ret;
}
struct Table0VTable *GetTable0VTable(void) {
  static Table0VTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  return &s_vtable;
}
struct Table1Type make_Table1(mol2_cursor_t *cur) {
  Table1Type ret;
  ret.cur = *cur;
  ret.t = GetTable1VTable();
  return ret;
}
struct Table1VTable *GetTable1VTable(void) {
  static Table1VTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.f1 = Table1_get_f1_impl;
  return &s_vtable;
}
uint8_t Table1_get_f1_impl(Table1Type *this) {
  uint8_t ret;
  mol2_cursor_t ret2 = mol2_table_slice_by_index(&this->cur, 0);
  ret = convert_to_Uint8(&ret2);
  return ret;
}
struct Table2Type make_Table2(mol2_cursor_t *cur) {
  Table2Type ret;
  ret.cur = *cur;
  ret.t = GetTable2VTable();
  return ret;
}
struct Table2VTable *GetTable2VTable(void) {
  static Table2VTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.f1 = Table2_get_f1_impl;
  s_vtable.f2 = Table2_get_f2_impl;
  return &s_vtable;
}
uint8_t Table2_get_f1_impl(Table2Type *this) {
  uint8_t ret;
  mol2_cursor_t ret2 = mol2_table_slice_by_index(&this->cur, 0);
  ret = convert_to_Uint8(&ret2);
  return ret;
}
Word2Type Table2_get_f2_impl(Table2Type *this) {
  Word2Type ret;
  mol2_cursor_t cur = mol2_table_slice_by_index(&this->cur, 1);
  ret.cur = cur;
  ret.t = GetWord2VTable();
  return ret;
}
struct Table3Type make_Table3(mol2_cursor_t *cur) {
  Table3Type ret;
  ret.cur = *cur;
  ret.t = GetTable3VTable();
  return ret;
}
struct Table3VTable *GetTable3VTable(void) {
  static Table3VTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.f1 = Table3_get_f1_impl;
  s_vtable.f2 = Table3_get_f2_impl;
  s_vtable.f3 = Table3_get_f3_impl;
  return &s_vtable;
}
uint8_t Table3_get_f1_impl(Table3Type *this) {
  uint8_t ret;
  mol2_cursor_t ret2 = mol2_table_slice_by_index(&this->cur, 0);
  ret = convert_to_Uint8(&ret2);
  return ret;
}
Word2Type Table3_get_f2_impl(Table3Type *this) {
  Word2Type ret;
  mol2_cursor_t cur = mol2_table_slice_by_index(&this->cur, 1);
  ret.cur = cur;
  ret.t = GetWord2VTable();
  return ret;
}
StructAType Table3_get_f3_impl(Table3Type *this) {
  StructAType ret;
  mol2_cursor_t cur = mol2_table_slice_by_index(&this->cur, 2);
  ret.cur = cur;
  ret.t = GetStructAVTable();
  return ret;
}
struct Table4Type make_Table4(mol2_cursor_t *cur) {
  Table4Type ret;
  ret.cur = *cur;
  ret.t = GetTable4VTable();
  return ret;
}
struct Table4VTable *GetTable4VTable(void) {
  static Table4VTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.f1 = Table4_get_f1_impl;
  s_vtable.f2 = Table4_get_f2_impl;
  s_vtable.f3 = Table4_get_f3_impl;
  s_vtable.f4 = Table4_get_f4_impl;
  return &s_vtable;
}
uint8_t Table4_get_f1_impl(Table4Type *this) {
  uint8_t ret;
  mol2_cursor_t ret2 = mol2_table_slice_by_index(&this->cur, 0);
  ret = convert_to_Uint8(&ret2);
  return ret;
}
Word2Type Table4_get_f2_impl(Table4Type *this) {
  Word2Type ret;
  mol2_cursor_t cur = mol2_table_slice_by_index(&this->cur, 1);
  ret.cur = cur;
  ret.t = GetWord2VTable();
  return ret;
}
StructAType Table4_get_f3_impl(Table4Type *this) {
  StructAType ret;
  mol2_cursor_t cur = mol2_table_slice_by_index(&this->cur, 2);
  ret.cur = cur;
  ret.t = GetStructAVTable();
  return ret;
}
mol2_cursor_t Table4_get_f4_impl(Table4Type *this) {
  mol2_cursor_t ret;
  mol2_cursor_t re2 = mol2_table_slice_by_index(&this->cur, 3);
  ret = convert_to_rawbytes(&re2);
  return ret;
}
struct Table5Type make_Table5(mol2_cursor_t *cur) {
  Table5Type ret;
  ret.cur = *cur;
  ret.t = GetTable5VTable();
  return ret;
}
struct Table5VTable *GetTable5VTable(void) {
  static Table5VTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.f1 = Table5_get_f1_impl;
  s_vtable.f2 = Table5_get_f2_impl;
  s_vtable.f3 = Table5_get_f3_impl;
  s_vtable.f4 = Table5_get_f4_impl;
  s_vtable.f5 = Table5_get_f5_impl;
  return &s_vtable;
}
uint8_t Table5_get_f1_impl(Table5Type *this) {
  uint8_t ret;
  mol2_cursor_t ret2 = mol2_table_slice_by_index(&this->cur, 0);
  ret = convert_to_Uint8(&ret2);
  return ret;
}
Word2Type Table5_get_f2_impl(Table5Type *this) {
  Word2Type ret;
  mol2_cursor_t cur = mol2_table_slice_by_index(&this->cur, 1);
  ret.cur = cur;
  ret.t = GetWord2VTable();
  return ret;
}
StructAType Table5_get_f3_impl(Table5Type *this) {
  StructAType ret;
  mol2_cursor_t cur = mol2_table_slice_by_index(&this->cur, 2);
  ret.cur = cur;
  ret.t = GetStructAVTable();
  return ret;
}
mol2_cursor_t Table5_get_f4_impl(Table5Type *this) {
  mol2_cursor_t ret;
  mol2_cursor_t re2 = mol2_table_slice_by_index(&this->cur, 3);
  ret = convert_to_rawbytes(&re2);
  return ret;
}
BytesVecType Table5_get_f5_impl(Table5Type *this) {
  BytesVecType ret;
  mol2_cursor_t cur = mol2_table_slice_by_index(&this->cur, 4);
  ret.cur = cur;
  ret.t = GetBytesVecVTable();
  return ret;
}
struct Table6Type make_Table6(mol2_cursor_t *cur) {
  Table6Type ret;
  ret.cur = *cur;
  ret.t = GetTable6VTable();
  return ret;
}
struct Table6VTable *GetTable6VTable(void) {
  static Table6VTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.f1 = Table6_get_f1_impl;
  s_vtable.f2 = Table6_get_f2_impl;
  s_vtable.f3 = Table6_get_f3_impl;
  s_vtable.f4 = Table6_get_f4_impl;
  s_vtable.f5 = Table6_get_f5_impl;
  s_vtable.f6 = Table6_get_f6_impl;
  return &s_vtable;
}
uint8_t Table6_get_f1_impl(Table6Type *this) {
  uint8_t ret;
  mol2_cursor_t ret2 = mol2_table_slice_by_index(&this->cur, 0);
  ret = convert_to_Uint8(&ret2);
  return ret;
}
Word2Type Table6_get_f2_impl(Table6Type *this) {
  Word2Type ret;
  mol2_cursor_t cur = mol2_table_slice_by_index(&this->cur, 1);
  ret.cur = cur;
  ret.t = GetWord2VTable();
  return ret;
}
StructAType Table6_get_f3_impl(Table6Type *this) {
  StructAType ret;
  mol2_cursor_t cur = mol2_table_slice_by_index(&this->cur, 2);
  ret.cur = cur;
  ret.t = GetStructAVTable();
  return ret;
}
mol2_cursor_t Table6_get_f4_impl(Table6Type *this) {
  mol2_cursor_t ret;
  mol2_cursor_t re2 = mol2_table_slice_by_index(&this->cur, 3);
  ret = convert_to_rawbytes(&re2);
  return ret;
}
BytesVecType Table6_get_f5_impl(Table6Type *this) {
  BytesVecType ret;
  mol2_cursor_t cur = mol2_table_slice_by_index(&this->cur, 4);
  ret.cur = cur;
  ret.t = GetBytesVecVTable();
  return ret;
}
Table5Type Table6_get_f6_impl(Table6Type *this) {
  Table5Type ret;
  mol2_cursor_t cur = mol2_table_slice_by_index(&this->cur, 5);
  ret.cur = cur;
  ret.t = GetTable5VTable();
  return ret;
}
struct ByteOptType make_ByteOpt(mol2_cursor_t *cur) {
  ByteOptType ret;
  ret.cur = *cur;
  ret.t = GetByteOptVTable();
  return ret;
}
struct ByteOptVTable *GetByteOptVTable(void) {
  static ByteOptVTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.is_none = ByteOpt_is_none_impl;
  s_vtable.is_some = ByteOpt_is_some_impl;
  s_vtable.unwrap = ByteOpt_unwrap_impl;
  return &s_vtable;
}
bool ByteOpt_is_none_impl(ByteOptType *this) {
  return mol2_option_is_none(&this->cur);
}
bool ByteOpt_is_some_impl(ByteOptType *this) {
  return !mol2_option_is_none(&this->cur);
}
uint8_t ByteOpt_unwrap_impl(ByteOptType *this) {
  uint8_t ret;
  ret = convert_to_Uint8(&this->cur);
  return ret;
}
struct WordOptType make_WordOpt(mol2_cursor_t *cur) {
  WordOptType ret;
  ret.cur = *cur;
  ret.t = GetWordOptVTable();
  return ret;
}
struct WordOptVTable *GetWordOptVTable(void) {
  static WordOptVTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.is_none = WordOpt_is_none_impl;
  s_vtable.is_some = WordOpt_is_some_impl;
  s_vtable.unwrap = WordOpt_unwrap_impl;
  return &s_vtable;
}
bool WordOpt_is_none_impl(WordOptType *this) {
  return mol2_option_is_none(&this->cur);
}
bool WordOpt_is_some_impl(WordOptType *this) {
  return !mol2_option_is_none(&this->cur);
}
mol2_cursor_t WordOpt_unwrap_impl(WordOptType *this) {
  mol2_cursor_t ret;
  ret = convert_to_array(&this->cur);
  return ret;
}
struct StructAOptType make_StructAOpt(mol2_cursor_t *cur) {
  StructAOptType ret;
  ret.cur = *cur;
  ret.t = GetStructAOptVTable();
  return ret;
}
struct StructAOptVTable *GetStructAOptVTable(void) {
  static StructAOptVTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.is_none = StructAOpt_is_none_impl;
  s_vtable.is_some = StructAOpt_is_some_impl;
  s_vtable.unwrap = StructAOpt_unwrap_impl;
  return &s_vtable;
}
bool StructAOpt_is_none_impl(StructAOptType *this) {
  return mol2_option_is_none(&this->cur);
}
bool StructAOpt_is_some_impl(StructAOptType *this) {
  return !mol2_option_is_none(&this->cur);
}
StructAType StructAOpt_unwrap_impl(StructAOptType *this) {
  StructAType ret;
  mol2_cursor_t cur = this->cur;
  ret.cur = cur;
  ret.t = GetStructAVTable();
  return ret;
}
struct StructPOptType make_StructPOpt(mol2_cursor_t *cur) {
  StructPOptType ret;
  ret.cur = *cur;
  ret.t = GetStructPOptVTable();
  return ret;
}
struct StructPOptVTable *GetStructPOptVTable(void) {
  static StructPOptVTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.is_none = StructPOpt_is_none_impl;
  s_vtable.is_some = StructPOpt_is_some_impl;
  s_vtable.unwrap = StructPOpt_unwrap_impl;
  return &s_vtable;
}
bool StructPOpt_is_none_impl(StructPOptType *this) {
  return mol2_option_is_none(&this->cur);
}
bool StructPOpt_is_some_impl(StructPOptType *this) {
  return !mol2_option_is_none(&this->cur);
}
StructPType StructPOpt_unwrap_impl(StructPOptType *this) {
  StructPType ret;
  mol2_cursor_t cur = this->cur;
  ret.cur = cur;
  ret.t = GetStructPVTable();
  return ret;
}
struct BytesOptType make_BytesOpt(mol2_cursor_t *cur) {
  BytesOptType ret;
  ret.cur = *cur;
  ret.t = GetBytesOptVTable();
  return ret;
}
struct BytesOptVTable *GetBytesOptVTable(void) {
  static BytesOptVTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.is_none = BytesOpt_is_none_impl;
  s_vtable.is_some = BytesOpt_is_some_impl;
  s_vtable.unwrap = BytesOpt_unwrap_impl;
  return &s_vtable;
}
bool BytesOpt_is_none_impl(BytesOptType *this) {
  return mol2_option_is_none(&this->cur);
}
bool BytesOpt_is_some_impl(BytesOptType *this) {
  return !mol2_option_is_none(&this->cur);
}
mol2_cursor_t BytesOpt_unwrap_impl(BytesOptType *this) {
  mol2_cursor_t ret;
  ret = convert_to_rawbytes(&this->cur);
  return ret;
}
struct WordsOptType make_WordsOpt(mol2_cursor_t *cur) {
  WordsOptType ret;
  ret.cur = *cur;
  ret.t = GetWordsOptVTable();
  return ret;
}
struct WordsOptVTable *GetWordsOptVTable(void) {
  static WordsOptVTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.is_none = WordsOpt_is_none_impl;
  s_vtable.is_some = WordsOpt_is_some_impl;
  s_vtable.unwrap = WordsOpt_unwrap_impl;
  return &s_vtable;
}
bool WordsOpt_is_none_impl(WordsOptType *this) {
  return mol2_option_is_none(&this->cur);
}
bool WordsOpt_is_some_impl(WordsOptType *this) {
  return !mol2_option_is_none(&this->cur);
}
WordsType WordsOpt_unwrap_impl(WordsOptType *this) {
  WordsType ret;
  mol2_cursor_t cur = this->cur;
  ret.cur = cur;
  ret.t = GetWordsVTable();
  return ret;
}
struct BytesVecOptType make_BytesVecOpt(mol2_cursor_t *cur) {
  BytesVecOptType ret;
  ret.cur = *cur;
  ret.t = GetBytesVecOptVTable();
  return ret;
}
struct BytesVecOptVTable *GetBytesVecOptVTable(void) {
  static BytesVecOptVTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.is_none = BytesVecOpt_is_none_impl;
  s_vtable.is_some = BytesVecOpt_is_some_impl;
  s_vtable.unwrap = BytesVecOpt_unwrap_impl;
  return &s_vtable;
}
bool BytesVecOpt_is_none_impl(BytesVecOptType *this) {
  return mol2_option_is_none(&this->cur);
}
bool BytesVecOpt_is_some_impl(BytesVecOptType *this) {
  return !mol2_option_is_none(&this->cur);
}
BytesVecType BytesVecOpt_unwrap_impl(BytesVecOptType *this) {
  BytesVecType ret;
  mol2_cursor_t cur = this->cur;
  ret.cur = cur;
  ret.t = GetBytesVecVTable();
  return ret;
}
struct WordsVecOptType make_WordsVecOpt(mol2_cursor_t *cur) {
  WordsVecOptType ret;
  ret.cur = *cur;
  ret.t = GetWordsVecOptVTable();
  return ret;
}
struct WordsVecOptVTable *GetWordsVecOptVTable(void) {
  static WordsVecOptVTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.is_none = WordsVecOpt_is_none_impl;
  s_vtable.is_some = WordsVecOpt_is_some_impl;
  s_vtable.unwrap = WordsVecOpt_unwrap_impl;
  return &s_vtable;
}
bool WordsVecOpt_is_none_impl(WordsVecOptType *this) {
  return mol2_option_is_none(&this->cur);
}
bool WordsVecOpt_is_some_impl(WordsVecOptType *this) {
  return !mol2_option_is_none(&this->cur);
}
WordsVecType WordsVecOpt_unwrap_impl(WordsVecOptType *this) {
  WordsVecType ret;
  mol2_cursor_t cur = this->cur;
  ret.cur = cur;
  ret.t = GetWordsVecVTable();
  return ret;
}
struct Table0OptType make_Table0Opt(mol2_cursor_t *cur) {
  Table0OptType ret;
  ret.cur = *cur;
  ret.t = GetTable0OptVTable();
  return ret;
}
struct Table0OptVTable *GetTable0OptVTable(void) {
  static Table0OptVTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.is_none = Table0Opt_is_none_impl;
  s_vtable.is_some = Table0Opt_is_some_impl;
  s_vtable.unwrap = Table0Opt_unwrap_impl;
  return &s_vtable;
}
bool Table0Opt_is_none_impl(Table0OptType *this) {
  return mol2_option_is_none(&this->cur);
}
bool Table0Opt_is_some_impl(Table0OptType *this) {
  return !mol2_option_is_none(&this->cur);
}
Table0Type Table0Opt_unwrap_impl(Table0OptType *this) {
  Table0Type ret;
  mol2_cursor_t cur = this->cur;
  ret.cur = cur;
  ret.t = GetTable0VTable();
  return ret;
}
struct Table6OptType make_Table6Opt(mol2_cursor_t *cur) {
  Table6OptType ret;
  ret.cur = *cur;
  ret.t = GetTable6OptVTable();
  return ret;
}
struct Table6OptVTable *GetTable6OptVTable(void) {
  static Table6OptVTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.is_none = Table6Opt_is_none_impl;
  s_vtable.is_some = Table6Opt_is_some_impl;
  s_vtable.unwrap = Table6Opt_unwrap_impl;
  return &s_vtable;
}
bool Table6Opt_is_none_impl(Table6OptType *this) {
  return mol2_option_is_none(&this->cur);
}
bool Table6Opt_is_some_impl(Table6OptType *this) {
  return !mol2_option_is_none(&this->cur);
}
Table6Type Table6Opt_unwrap_impl(Table6OptType *this) {
  Table6Type ret;
  mol2_cursor_t cur = this->cur;
  ret.cur = cur;
  ret.t = GetTable6VTable();
  return ret;
}
struct Table6OptOptType make_Table6OptOpt(mol2_cursor_t *cur) {
  Table6OptOptType ret;
  ret.cur = *cur;
  ret.t = GetTable6OptOptVTable();
  return ret;
}
struct Table6OptOptVTable *GetTable6OptOptVTable(void) {
  static Table6OptOptVTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.is_none = Table6OptOpt_is_none_impl;
  s_vtable.is_some = Table6OptOpt_is_some_impl;
  s_vtable.unwrap = Table6OptOpt_unwrap_impl;
  return &s_vtable;
}
bool Table6OptOpt_is_none_impl(Table6OptOptType *this) {
  return mol2_option_is_none(&this->cur);
}
bool Table6OptOpt_is_some_impl(Table6OptOptType *this) {
  return !mol2_option_is_none(&this->cur);
}
Table6OptType Table6OptOpt_unwrap_impl(Table6OptOptType *this) {
  Table6OptType ret;
  mol2_cursor_t cur = this->cur;
  ret.cur = cur;
  ret.t = GetTable6OptVTable();
  return ret;
}
struct ByteOptVecType make_ByteOptVec(mol2_cursor_t *cur) {
  ByteOptVecType ret;
  ret.cur = *cur;
  ret.t = GetByteOptVecVTable();
  return ret;
}
struct ByteOptVecVTable *GetByteOptVecVTable(void) {
  static ByteOptVecVTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.len = ByteOptVec_len_impl;
  s_vtable.get = ByteOptVec_get_impl;
  return &s_vtable;
}
uint32_t ByteOptVec_len_impl(ByteOptVecType *this) {
  return mol2_dynvec_length(&this->cur);
}
ByteOptType ByteOptVec_get_impl(ByteOptVecType *this, uint32_t index,
                                bool *existing) {
  ByteOptType ret = {0};
  mol2_cursor_res_t res = mol2_dynvec_slice_by_index(&this->cur, index);
  if (res.errno != MOL2_OK) {
    *existing = false;
    return ret;
  } else {
    *existing = true;
  }
  ret.cur = res.cur;
  ret.t = GetByteOptVTable();
  return ret;
}
struct WordOptVecType make_WordOptVec(mol2_cursor_t *cur) {
  WordOptVecType ret;
  ret.cur = *cur;
  ret.t = GetWordOptVecVTable();
  return ret;
}
struct WordOptVecVTable *GetWordOptVecVTable(void) {
  static WordOptVecVTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.len = WordOptVec_len_impl;
  s_vtable.get = WordOptVec_get_impl;
  return &s_vtable;
}
uint32_t WordOptVec_len_impl(WordOptVecType *this) {
  return mol2_dynvec_length(&this->cur);
}
WordOptType WordOptVec_get_impl(WordOptVecType *this, uint32_t index,
                                bool *existing) {
  WordOptType ret = {0};
  mol2_cursor_res_t res = mol2_dynvec_slice_by_index(&this->cur, index);
  if (res.errno != MOL2_OK) {
    *existing = false;
    return ret;
  } else {
    *existing = true;
  }
  ret.cur = res.cur;
  ret.t = GetWordOptVTable();
  return ret;
}
struct WordsOptVecType make_WordsOptVec(mol2_cursor_t *cur) {
  WordsOptVecType ret;
  ret.cur = *cur;
  ret.t = GetWordsOptVecVTable();
  return ret;
}
struct WordsOptVecVTable *GetWordsOptVecVTable(void) {
  static WordsOptVecVTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.len = WordsOptVec_len_impl;
  s_vtable.get = WordsOptVec_get_impl;
  return &s_vtable;
}
uint32_t WordsOptVec_len_impl(WordsOptVecType *this) {
  return mol2_dynvec_length(&this->cur);
}
WordsOptType WordsOptVec_get_impl(WordsOptVecType *this, uint32_t index,
                                  bool *existing) {
  WordsOptType ret = {0};
  mol2_cursor_res_t res = mol2_dynvec_slice_by_index(&this->cur, index);
  if (res.errno != MOL2_OK) {
    *existing = false;
    return ret;
  } else {
    *existing = true;
  }
  ret.cur = res.cur;
  ret.t = GetWordsOptVTable();
  return ret;
}
struct BytesOptVecType make_BytesOptVec(mol2_cursor_t *cur) {
  BytesOptVecType ret;
  ret.cur = *cur;
  ret.t = GetBytesOptVecVTable();
  return ret;
}
struct BytesOptVecVTable *GetBytesOptVecVTable(void) {
  static BytesOptVecVTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.len = BytesOptVec_len_impl;
  s_vtable.get = BytesOptVec_get_impl;
  return &s_vtable;
}
uint32_t BytesOptVec_len_impl(BytesOptVecType *this) {
  return mol2_dynvec_length(&this->cur);
}
BytesOptType BytesOptVec_get_impl(BytesOptVecType *this, uint32_t index,
                                  bool *existing) {
  BytesOptType ret = {0};
  mol2_cursor_res_t res = mol2_dynvec_slice_by_index(&this->cur, index);
  if (res.errno != MOL2_OK) {
    *existing = false;
    return ret;
  } else {
    *existing = true;
  }
  ret.cur = res.cur;
  ret.t = GetBytesOptVTable();
  return ret;
}
struct UnionAType make_UnionA(mol2_cursor_t *cur) {
  UnionAType ret;
  ret.cur = *cur;
  ret.t = GetUnionAVTable();
  return ret;
}
struct UnionAVTable *GetUnionAVTable(void) {
  static UnionAVTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.item_id = UnionA_item_id_impl;
  s_vtable.as_byte = UnionA_as_byte_impl;
  s_vtable.as_Word = UnionA_as_Word_impl;
  s_vtable.as_StructA = UnionA_as_StructA_impl;
  s_vtable.as_Bytes = UnionA_as_Bytes_impl;
  s_vtable.as_Words = UnionA_as_Words_impl;
  s_vtable.as_Table0 = UnionA_as_Table0_impl;
  s_vtable.as_Table6 = UnionA_as_Table6_impl;
  s_vtable.as_Table6Opt = UnionA_as_Table6Opt_impl;
  return &s_vtable;
}
uint32_t UnionA_item_id_impl(UnionAType *this) {
  return mol2_unpack_number(&this->cur);
}
uint8_t UnionA_as_byte_impl(UnionAType *this) {
  uint8_t ret;
  mol2_union_t u = mol2_union_unpack(&this->cur);
  ret = convert_to_Uint8(&u.cursor);
  return ret;
}
mol2_cursor_t UnionA_as_Word_impl(UnionAType *this) {
  mol2_cursor_t ret;
  mol2_union_t u = mol2_union_unpack(&this->cur);
  ret = convert_to_array(&u.cursor);
  return ret;
}
StructAType UnionA_as_StructA_impl(UnionAType *this) {
  StructAType ret;
  mol2_union_t u = mol2_union_unpack(&this->cur);
  ret.cur = u.cursor;
  ret.t = GetStructAVTable();
  return ret;
}
mol2_cursor_t UnionA_as_Bytes_impl(UnionAType *this) {
  mol2_cursor_t ret;
  mol2_union_t u = mol2_union_unpack(&this->cur);
  ret = convert_to_rawbytes(&u.cursor);
  return ret;
}
WordsType UnionA_as_Words_impl(UnionAType *this) {
  WordsType ret;
  mol2_union_t u = mol2_union_unpack(&this->cur);
  ret.cur = u.cursor;
  ret.t = GetWordsVTable();
  return ret;
}
Table0Type UnionA_as_Table0_impl(UnionAType *this) {
  Table0Type ret;
  mol2_union_t u = mol2_union_unpack(&this->cur);
  ret.cur = u.cursor;
  ret.t = GetTable0VTable();
  return ret;
}
Table6Type UnionA_as_Table6_impl(UnionAType *this) {
  Table6Type ret;
  mol2_union_t u = mol2_union_unpack(&this->cur);
  ret.cur = u.cursor;
  ret.t = GetTable6VTable();
  return ret;
}
Table6OptType UnionA_as_Table6Opt_impl(UnionAType *this) {
  Table6OptType ret;
  mol2_union_t u = mol2_union_unpack(&this->cur);
  ret.cur = u.cursor;
  ret.t = GetTable6OptVTable();
  return ret;
}
struct TableAType make_TableA(mol2_cursor_t *cur) {
  TableAType ret;
  ret.cur = *cur;
  ret.t = GetTableAVTable();
  return ret;
}
struct TableAVTable *GetTableAVTable(void) {
  static TableAVTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.f1 = TableA_get_f1_impl;
  s_vtable.f2 = TableA_get_f2_impl;
  s_vtable.f3 = TableA_get_f3_impl;
  s_vtable.f4 = TableA_get_f4_impl;
  s_vtable.f5 = TableA_get_f5_impl;
  s_vtable.f6 = TableA_get_f6_impl;
  s_vtable.f7 = TableA_get_f7_impl;
  s_vtable.f8 = TableA_get_f8_impl;
  return &s_vtable;
}
Word2Type TableA_get_f1_impl(TableAType *this) {
  Word2Type ret;
  mol2_cursor_t cur = mol2_table_slice_by_index(&this->cur, 0);
  ret.cur = cur;
  ret.t = GetWord2VTable();
  return ret;
}
StructAType TableA_get_f2_impl(TableAType *this) {
  StructAType ret;
  mol2_cursor_t cur = mol2_table_slice_by_index(&this->cur, 1);
  ret.cur = cur;
  ret.t = GetStructAVTable();
  return ret;
}
mol2_cursor_t TableA_get_f3_impl(TableAType *this) {
  mol2_cursor_t ret;
  mol2_cursor_t re2 = mol2_table_slice_by_index(&this->cur, 2);
  ret = convert_to_rawbytes(&re2);
  return ret;
}
BytesVecType TableA_get_f4_impl(TableAType *this) {
  BytesVecType ret;
  mol2_cursor_t cur = mol2_table_slice_by_index(&this->cur, 3);
  ret.cur = cur;
  ret.t = GetBytesVecVTable();
  return ret;
}
Table1Type TableA_get_f5_impl(TableAType *this) {
  Table1Type ret;
  mol2_cursor_t cur = mol2_table_slice_by_index(&this->cur, 4);
  ret.cur = cur;
  ret.t = GetTable1VTable();
  return ret;
}
BytesOptType TableA_get_f6_impl(TableAType *this) {
  BytesOptType ret;
  mol2_cursor_t cur = mol2_table_slice_by_index(&this->cur, 5);
  ret.cur = cur;
  ret.t = GetBytesOptVTable();
  return ret;
}
UnionAType TableA_get_f7_impl(TableAType *this) {
  UnionAType ret;
  mol2_cursor_t cur = mol2_table_slice_by_index(&this->cur, 6);
  ret.cur = cur;
  ret.t = GetUnionAVTable();
  return ret;
}
uint8_t TableA_get_f8_impl(TableAType *this) {
  uint8_t ret;
  mol2_cursor_t ret2 = mol2_table_slice_by_index(&this->cur, 7);
  ret = convert_to_Uint8(&ret2);
  return ret;
}
struct AllInOneType make_AllInOne(mol2_cursor_t *cur) {
  AllInOneType ret;
  ret.cur = *cur;
  ret.t = GetAllInOneVTable();
  return ret;
}
struct AllInOneVTable *GetAllInOneVTable(void) {
  static AllInOneVTable s_vtable;
  static int inited = 0;
  if (inited) return &s_vtable;
  s_vtable.f0 = AllInOne_get_f0_impl;
  s_vtable.f1 = AllInOne_get_f1_impl;
  s_vtable.f2 = AllInOne_get_f2_impl;
  s_vtable.f3 = AllInOne_get_f3_impl;
  s_vtable.f4 = AllInOne_get_f4_impl;
  s_vtable.f5 = AllInOne_get_f5_impl;
  s_vtable.f6 = AllInOne_get_f6_impl;
  s_vtable.f7 = AllInOne_get_f7_impl;
  s_vtable.f8 = AllInOne_get_f8_impl;
  s_vtable.f9 = AllInOne_get_f9_impl;
  s_vtable.f10 = AllInOne_get_f10_impl;
  s_vtable.f11 = AllInOne_get_f11_impl;
  s_vtable.f12 = AllInOne_get_f12_impl;
  s_vtable.f13 = AllInOne_get_f13_impl;
  s_vtable.f14 = AllInOne_get_f14_impl;
  s_vtable.f15 = AllInOne_get_f15_impl;
  s_vtable.f16 = AllInOne_get_f16_impl;
  s_vtable.f17 = AllInOne_get_f17_impl;
  s_vtable.f18 = AllInOne_get_f18_impl;
  s_vtable.f19 = AllInOne_get_f19_impl;
  s_vtable.f20 = AllInOne_get_f20_impl;
  s_vtable.f21 = AllInOne_get_f21_impl;
  s_vtable.f22 = AllInOne_get_f22_impl;
  s_vtable.f23 = AllInOne_get_f23_impl;
  s_vtable.f24 = AllInOne_get_f24_impl;
  s_vtable.f25 = AllInOne_get_f25_impl;
  s_vtable.f26 = AllInOne_get_f26_impl;
  s_vtable.f27 = AllInOne_get_f27_impl;
  s_vtable.f28 = AllInOne_get_f28_impl;
  s_vtable.f29 = AllInOne_get_f29_impl;
  s_vtable.f30 = AllInOne_get_f30_impl;
  s_vtable.f31 = AllInOne_get_f31_impl;
  s_vtable.f32 = AllInOne_get_f32_impl;
  s_vtable.f33 = AllInOne_get_f33_impl;
  s_vtable.f34 = AllInOne_get_f34_impl;
  s_vtable.f35 = AllInOne_get_f35_impl;
  s_vtable.f36 = AllInOne_get_f36_impl;
  s_vtable.f37 = AllInOne_get_f37_impl;
  s_vtable.f38 = AllInOne_get_f38_impl;
  s_vtable.f39 = AllInOne_get_f39_impl;
  s_vtable.f40 = AllInOne_get_f40_impl;
  s_vtable.f41 = AllInOne_get_f41_impl;
  s_vtable.f42 = AllInOne_get_f42_impl;
  s_vtable.f43 = AllInOne_get_f43_impl;
  s_vtable.f44 = AllInOne_get_f44_impl;
  s_vtable.f45 = AllInOne_get_f45_impl;
  s_vtable.f46 = AllInOne_get_f46_impl;
  s_vtable.f47 = AllInOne_get_f47_impl;
  s_vtable.f48 = AllInOne_get_f48_impl;
  s_vtable.f49 = AllInOne_get_f49_impl;
  s_vtable.f50 = AllInOne_get_f50_impl;
  s_vtable.f51 = AllInOne_get_f51_impl;
  s_vtable.f52 = AllInOne_get_f52_impl;
  s_vtable.f53 = AllInOne_get_f53_impl;
  s_vtable.f54 = AllInOne_get_f54_impl;
  s_vtable.f55 = AllInOne_get_f55_impl;
  s_vtable.f56 = AllInOne_get_f56_impl;
  s_vtable.f57 = AllInOne_get_f57_impl;
  s_vtable.f58 = AllInOne_get_f58_impl;
  s_vtable.f59 = AllInOne_get_f59_impl;
  s_vtable.f60 = AllInOne_get_f60_impl;
  s_vtable.f61 = AllInOne_get_f61_impl;
  s_vtable.f62 = AllInOne_get_f62_impl;
  s_vtable.f63 = AllInOne_get_f63_impl;
  s_vtable.f64 = AllInOne_get_f64_impl;
  s_vtable.f65 = AllInOne_get_f65_impl;
  s_vtable.f66 = AllInOne_get_f66_impl;
  s_vtable.f67 = AllInOne_get_f67_impl;
  s_vtable.f68 = AllInOne_get_f68_impl;
  s_vtable.f69 = AllInOne_get_f69_impl;
  s_vtable.f70 = AllInOne_get_f70_impl;
  s_vtable.f71 = AllInOne_get_f71_impl;
  s_vtable.f72 = AllInOne_get_f72_impl;
  s_vtable.f73 = AllInOne_get_f73_impl;
  return &s_vtable;
}
uint8_t AllInOne_get_f0_impl(AllInOneType *this) {
  uint8_t ret;
  mol2_cursor_t ret2 = mol2_table_slice_by_index(&this->cur, 0);
  ret = convert_to_Uint8(&ret2);
  return ret;
}
mol2_cursor_t AllInOne_get_f1_impl(AllInOneType *this) {
  mol2_cursor_t ret;
  mol2_cursor_t ret2 = mol2_table_slice_by_index(&this->cur, 1);
  ret = convert_to_array(&ret2);
  return ret;
}
mol2_cursor_t AllInOne_get_f2_impl(AllInOneType *this) {
  mol2_cursor_t ret;
  mol2_cursor_t ret2 = mol2_table_slice_by_index(&this->cur, 2);
  ret = convert_to_array(&ret2);
  return ret;
}
mol2_cursor_t AllInOne_get_f3_impl(AllInOneType *this) {
  mol2_cursor_t ret;
  mol2_cursor_t ret2 = mol2_table_slice_by_index(&this->cur, 3);
  ret = convert_to_array(&ret2);
  return ret;
}
mol2_cursor_t AllInOne_get_f4_impl(AllInOneType *this) {
  mol2_cursor_t ret;
  mol2_cursor_t ret2 = mol2_table_slice_by_index(&this->cur, 4);
  ret = convert_to_array(&ret2);
  return ret;
}
mol2_cursor_t AllInOne_get_f5_impl(AllInOneType *this) {
  mol2_cursor_t ret;
  mol2_cursor_t ret2 = mol2_table_slice_by_index(&this->cur, 5);
  ret = convert_to_array(&ret2);
  return ret;
}
mol2_cursor_t AllInOne_get_f6_impl(AllInOneType *this) {
  mol2_cursor_t ret;
  mol2_cursor_t ret2 = mol2_table_slice_by_index(&this->cur, 6);
  ret = convert_to_array(&ret2);
  return ret;
}
mol2_cursor_t AllInOne_get_f7_impl(AllInOneType *this) {
  mol2_cursor_t ret;
  mol2_cursor_t ret2 = mol2_table_slice_by_index(&this->cur, 7);
  ret = convert_to_array(&ret2);
  return ret;
}
mol2_cursor_t AllInOne_get_f8_impl(AllInOneType *this) {
  mol2_cursor_t ret;
  mol2_cursor_t ret2 = mol2_table_slice_by_index(&this->cur, 8);
  ret = convert_to_array(&ret2);
  return ret;
}
mol2_cursor_t AllInOne_get_f9_impl(AllInOneType *this) {
  mol2_cursor_t ret;
  mol2_cursor_t ret2 = mol2_table_slice_by_index(&this->cur, 9);
  ret = convert_to_array(&ret2);
  return ret;
}
mol2_cursor_t AllInOne_get_f10_impl(AllInOneType *this) {
  mol2_cursor_t ret;
  mol2_cursor_t ret2 = mol2_table_slice_by_index(&this->cur, 10);
  ret = convert_to_array(&ret2);
  return ret;
}
mol2_cursor_t AllInOne_get_f11_impl(AllInOneType *this) {
  mol2_cursor_t ret;
  mol2_cursor_t ret2 = mol2_table_slice_by_index(&this->cur, 11);
  ret = convert_to_array(&ret2);
  return ret;
}
mol2_cursor_t AllInOne_get_f12_impl(AllInOneType *this) {
  mol2_cursor_t ret;
  mol2_cursor_t ret2 = mol2_table_slice_by_index(&this->cur, 12);
  ret = convert_to_array(&ret2);
  return ret;
}
mol2_cursor_t AllInOne_get_f13_impl(AllInOneType *this) {
  mol2_cursor_t ret;
  mol2_cursor_t ret2 = mol2_table_slice_by_index(&this->cur, 13);
  ret = convert_to_array(&ret2);
  return ret;
}
mol2_cursor_t AllInOne_get_f14_impl(AllInOneType *this) {
  mol2_cursor_t ret;
  mol2_cursor_t ret2 = mol2_table_slice_by_index(&this->cur, 14);
  ret = convert_to_array(&ret2);
  return ret;
}
mol2_cursor_t AllInOne_get_f15_impl(AllInOneType *this) {
  mol2_cursor_t ret;
  mol2_cursor_t ret2 = mol2_table_slice_by_index(&this->cur, 15);
  ret = convert_to_array(&ret2);
  return ret;
}
mol2_cursor_t AllInOne_get_f16_impl(AllInOneType *this) {
  mol2_cursor_t ret;
  mol2_cursor_t ret2 = mol2_table_slice_by_index(&this->cur, 16);
  ret = convert_to_array(&ret2);
  return ret;
}
Word2Type AllInOne_get_f17_impl(AllInOneType *this) {
  Word2Type ret;
  mol2_cursor_t cur = mol2_table_slice_by_index(&this->cur, 17);
  ret.cur = cur;
  ret.t = GetWord2VTable();
  return ret;
}
Word3Type AllInOne_get_f18_impl(AllInOneType *this) {
  Word3Type ret;
  mol2_cursor_t cur = mol2_table_slice_by_index(&this->cur, 18);
  ret.cur = cur;
  ret.t = GetWord3VTable();
  return ret;
}
Word4Type AllInOne_get_f19_impl(AllInOneType *this) {
  Word4Type ret;
  mol2_cursor_t cur = mol2_table_slice_by_index(&this->cur, 19);
  ret.cur = cur;
  ret.t = GetWord4VTable();
  return ret;
}
Word5Type AllInOne_get_f20_impl(AllInOneType *this) {
  Word5Type ret;
  mol2_cursor_t cur = mol2_table_slice_by_index(&this->cur, 20);
  ret.cur = cur;
  ret.t = GetWord5VTable();
  return ret;
}
Word6Type AllInOne_get_f21_impl(AllInOneType *this) {
  Word6Type ret;
  mol2_cursor_t cur = mol2_table_slice_by_index(&this->cur, 21);
  ret.cur = cur;
  ret.t = GetWord6VTable();
  return ret;
}
Word7Type AllInOne_get_f22_impl(AllInOneType *this) {
  Word7Type ret;
  mol2_cursor_t cur = mol2_table_slice_by_index(&this->cur, 22);
  ret.cur = cur;
  ret.t = GetWord7VTable();
  return ret;
}
Word8Type AllInOne_get_f23_impl(AllInOneType *this) {
  Word8Type ret;
  mol2_cursor_t cur = mol2_table_slice_by_index(&this->cur, 23);
  ret.cur = cur;
  ret.t = GetWord8VTable();
  return ret;
}
Byte3x3Type AllInOne_get_f24_impl(AllInOneType *this) {
  Byte3x3Type ret;
  mol2_cursor_t cur = mol2_table_slice_by_index(&this->cur, 24);
  ret.cur = cur;
  ret.t = GetByte3x3VTable();
  return ret;
}
Byte5x3Type AllInOne_get_f25_impl(AllInOneType *this) {
  Byte5x3Type ret;
  mol2_cursor_t cur = mol2_table_slice_by_index(&this->cur, 25);
  ret.cur = cur;
  ret.t = GetByte5x3VTable();
  return ret;
}
Byte7x3Type AllInOne_get_f26_impl(AllInOneType *this) {
  Byte7x3Type ret;
  mol2_cursor_t cur = mol2_table_slice_by_index(&this->cur, 26);
  ret.cur = cur;
  ret.t = GetByte7x3VTable();
  return ret;
}
Byte9x3Type AllInOne_get_f27_impl(AllInOneType *this) {
  Byte9x3Type ret;
  mol2_cursor_t cur = mol2_table_slice_by_index(&this->cur, 27);
  ret.cur = cur;
  ret.t = GetByte9x3VTable();
  return ret;
}
StructAType AllInOne_get_f28_impl(AllInOneType *this) {
  StructAType ret;
  mol2_cursor_t cur = mol2_table_slice_by_index(&this->cur, 28);
  ret.cur = cur;
  ret.t = GetStructAVTable();
  return ret;
}
StructBType AllInOne_get_f29_impl(AllInOneType *this) {
  StructBType ret;
  mol2_cursor_t cur = mol2_table_slice_by_index(&this->cur, 29);
  ret.cur = cur;
  ret.t = GetStructBVTable();
  return ret;
}
StructCType AllInOne_get_f30_impl(AllInOneType *this) {
  StructCType ret;
  mol2_cursor_t cur = mol2_table_slice_by_index(&this->cur, 30);
  ret.cur = cur;
  ret.t = GetStructCVTable();
  return ret;
}
StructDType AllInOne_get_f31_impl(AllInOneType *this) {
  StructDType ret;
  mol2_cursor_t cur = mol2_table_slice_by_index(&this->cur, 31);
  ret.cur = cur;
  ret.t = GetStructDVTable();
  return ret;
}
StructEType AllInOne_get_f32_impl(AllInOneType *this) {
  StructEType ret;
  mol2_cursor_t cur = mol2_table_slice_by_index(&this->cur, 32);
  ret.cur = cur;
  ret.t = GetStructEVTable();
  return ret;
}
StructFType AllInOne_get_f33_impl(AllInOneType *this) {
  StructFType ret;
  mol2_cursor_t cur = mol2_table_slice_by_index(&this->cur, 33);
  ret.cur = cur;
  ret.t = GetStructFVTable();
  return ret;
}
StructGType AllInOne_get_f34_impl(AllInOneType *this) {
  StructGType ret;
  mol2_cursor_t cur = mol2_table_slice_by_index(&this->cur, 34);
  ret.cur = cur;
  ret.t = GetStructGVTable();
  return ret;
}
StructHType AllInOne_get_f35_impl(AllInOneType *this) {
  StructHType ret;
  mol2_cursor_t cur = mol2_table_slice_by_index(&this->cur, 35);
  ret.cur = cur;
  ret.t = GetStructHVTable();
  return ret;
}
StructIType AllInOne_get_f36_impl(AllInOneType *this) {
  StructIType ret;
  mol2_cursor_t cur = mol2_table_slice_by_index(&this->cur, 36);
  ret.cur = cur;
  ret.t = GetStructIVTable();
  return ret;
}
StructJType AllInOne_get_f37_impl(AllInOneType *this) {
  StructJType ret;
  mol2_cursor_t cur = mol2_table_slice_by_index(&this->cur, 37);
  ret.cur = cur;
  ret.t = GetStructJVTable();
  return ret;
}
StructIx3Type AllInOne_get_f38_impl(AllInOneType *this) {
  StructIx3Type ret;
  mol2_cursor_t cur = mol2_table_slice_by_index(&this->cur, 38);
  ret.cur = cur;
  ret.t = GetStructIx3VTable();
  return ret;
}
StructOType AllInOne_get_f39_impl(AllInOneType *this) {
  StructOType ret;
  mol2_cursor_t cur = mol2_table_slice_by_index(&this->cur, 39);
  ret.cur = cur;
  ret.t = GetStructOVTable();
  return ret;
}
StructPType AllInOne_get_f40_impl(AllInOneType *this) {
  StructPType ret;
  mol2_cursor_t cur = mol2_table_slice_by_index(&this->cur, 40);
  ret.cur = cur;
  ret.t = GetStructPVTable();
  return ret;
}
mol2_cursor_t AllInOne_get_f41_impl(AllInOneType *this) {
  mol2_cursor_t ret;
  mol2_cursor_t re2 = mol2_table_slice_by_index(&this->cur, 41);
  ret = convert_to_rawbytes(&re2);
  return ret;
}
WordsType AllInOne_get_f42_impl(AllInOneType *this) {
  WordsType ret;
  mol2_cursor_t cur = mol2_table_slice_by_index(&this->cur, 42);
  ret.cur = cur;
  ret.t = GetWordsVTable();
  return ret;
}
Byte3VecType AllInOne_get_f43_impl(AllInOneType *this) {
  Byte3VecType ret;
  mol2_cursor_t cur = mol2_table_slice_by_index(&this->cur, 43);
  ret.cur = cur;
  ret.t = GetByte3VecVTable();
  return ret;
}
Byte7VecType AllInOne_get_f44_impl(AllInOneType *this) {
  Byte7VecType ret;
  mol2_cursor_t cur = mol2_table_slice_by_index(&this->cur, 44);
  ret.cur = cur;
  ret.t = GetByte7VecVTable();
  return ret;
}
StructIVecType AllInOne_get_f45_impl(AllInOneType *this) {
  StructIVecType ret;
  mol2_cursor_t cur = mol2_table_slice_by_index(&this->cur, 45);
  ret.cur = cur;
  ret.t = GetStructIVecVTable();
  return ret;
}
StructJVecType AllInOne_get_f46_impl(AllInOneType *this) {
  StructJVecType ret;
  mol2_cursor_t cur = mol2_table_slice_by_index(&this->cur, 46);
  ret.cur = cur;
  ret.t = GetStructJVecVTable();
  return ret;
}
StructPVecType AllInOne_get_f47_impl(AllInOneType *this) {
  StructPVecType ret;
  mol2_cursor_t cur = mol2_table_slice_by_index(&this->cur, 47);
  ret.cur = cur;
  ret.t = GetStructPVecVTable();
  return ret;
}
BytesVecType AllInOne_get_f48_impl(AllInOneType *this) {
  BytesVecType ret;
  mol2_cursor_t cur = mol2_table_slice_by_index(&this->cur, 48);
  ret.cur = cur;
  ret.t = GetBytesVecVTable();
  return ret;
}
WordsVecType AllInOne_get_f49_impl(AllInOneType *this) {
  WordsVecType ret;
  mol2_cursor_t cur = mol2_table_slice_by_index(&this->cur, 49);
  ret.cur = cur;
  ret.t = GetWordsVecVTable();
  return ret;
}
Table0Type AllInOne_get_f50_impl(AllInOneType *this) {
  Table0Type ret;
  mol2_cursor_t cur = mol2_table_slice_by_index(&this->cur, 50);
  ret.cur = cur;
  ret.t = GetTable0VTable();
  return ret;
}
Table1Type AllInOne_get_f51_impl(AllInOneType *this) {
  Table1Type ret;
  mol2_cursor_t cur = mol2_table_slice_by_index(&this->cur, 51);
  ret.cur = cur;
  ret.t = GetTable1VTable();
  return ret;
}
Table2Type AllInOne_get_f52_impl(AllInOneType *this) {
  Table2Type ret;
  mol2_cursor_t cur = mol2_table_slice_by_index(&this->cur, 52);
  ret.cur = cur;
  ret.t = GetTable2VTable();
  return ret;
}
Table3Type AllInOne_get_f53_impl(AllInOneType *this) {
  Table3Type ret;
  mol2_cursor_t cur = mol2_table_slice_by_index(&this->cur, 53);
  ret.cur = cur;
  ret.t = GetTable3VTable();
  return ret;
}
Table4Type AllInOne_get_f54_impl(AllInOneType *this) {
  Table4Type ret;
  mol2_cursor_t cur = mol2_table_slice_by_index(&this->cur, 54);
  ret.cur = cur;
  ret.t = GetTable4VTable();
  return ret;
}
Table5Type AllInOne_get_f55_impl(AllInOneType *this) {
  Table5Type ret;
  mol2_cursor_t cur = mol2_table_slice_by_index(&this->cur, 55);
  ret.cur = cur;
  ret.t = GetTable5VTable();
  return ret;
}
Table6Type AllInOne_get_f56_impl(AllInOneType *this) {
  Table6Type ret;
  mol2_cursor_t cur = mol2_table_slice_by_index(&this->cur, 56);
  ret.cur = cur;
  ret.t = GetTable6VTable();
  return ret;
}
ByteOptType AllInOne_get_f57_impl(AllInOneType *this) {
  ByteOptType ret;
  mol2_cursor_t cur = mol2_table_slice_by_index(&this->cur, 57);
  ret.cur = cur;
  ret.t = GetByteOptVTable();
  return ret;
}
WordOptType AllInOne_get_f58_impl(AllInOneType *this) {
  WordOptType ret;
  mol2_cursor_t cur = mol2_table_slice_by_index(&this->cur, 58);
  ret.cur = cur;
  ret.t = GetWordOptVTable();
  return ret;
}
StructAOptType AllInOne_get_f59_impl(AllInOneType *this) {
  StructAOptType ret;
  mol2_cursor_t cur = mol2_table_slice_by_index(&this->cur, 59);
  ret.cur = cur;
  ret.t = GetStructAOptVTable();
  return ret;
}
StructPOptType AllInOne_get_f60_impl(AllInOneType *this) {
  StructPOptType ret;
  mol2_cursor_t cur = mol2_table_slice_by_index(&this->cur, 60);
  ret.cur = cur;
  ret.t = GetStructPOptVTable();
  return ret;
}
BytesOptType AllInOne_get_f61_impl(AllInOneType *this) {
  BytesOptType ret;
  mol2_cursor_t cur = mol2_table_slice_by_index(&this->cur, 61);
  ret.cur = cur;
  ret.t = GetBytesOptVTable();
  return ret;
}
WordsOptType AllInOne_get_f62_impl(AllInOneType *this) {
  WordsOptType ret;
  mol2_cursor_t cur = mol2_table_slice_by_index(&this->cur, 62);
  ret.cur = cur;
  ret.t = GetWordsOptVTable();
  return ret;
}
BytesVecOptType AllInOne_get_f63_impl(AllInOneType *this) {
  BytesVecOptType ret;
  mol2_cursor_t cur = mol2_table_slice_by_index(&this->cur, 63);
  ret.cur = cur;
  ret.t = GetBytesVecOptVTable();
  return ret;
}
WordsVecOptType AllInOne_get_f64_impl(AllInOneType *this) {
  WordsVecOptType ret;
  mol2_cursor_t cur = mol2_table_slice_by_index(&this->cur, 64);
  ret.cur = cur;
  ret.t = GetWordsVecOptVTable();
  return ret;
}
Table0OptType AllInOne_get_f65_impl(AllInOneType *this) {
  Table0OptType ret;
  mol2_cursor_t cur = mol2_table_slice_by_index(&this->cur, 65);
  ret.cur = cur;
  ret.t = GetTable0OptVTable();
  return ret;
}
Table6OptType AllInOne_get_f66_impl(AllInOneType *this) {
  Table6OptType ret;
  mol2_cursor_t cur = mol2_table_slice_by_index(&this->cur, 66);
  ret.cur = cur;
  ret.t = GetTable6OptVTable();
  return ret;
}
Table6OptOptType AllInOne_get_f67_impl(AllInOneType *this) {
  Table6OptOptType ret;
  mol2_cursor_t cur = mol2_table_slice_by_index(&this->cur, 67);
  ret.cur = cur;
  ret.t = GetTable6OptOptVTable();
  return ret;
}
ByteOptVecType AllInOne_get_f68_impl(AllInOneType *this) {
  ByteOptVecType ret;
  mol2_cursor_t cur = mol2_table_slice_by_index(&this->cur, 68);
  ret.cur = cur;
  ret.t = GetByteOptVecVTable();
  return ret;
}
WordOptVecType AllInOne_get_f69_impl(AllInOneType *this) {
  WordOptVecType ret;
  mol2_cursor_t cur = mol2_table_slice_by_index(&this->cur, 69);
  ret.cur = cur;
  ret.t = GetWordOptVecVTable();
  return ret;
}
WordsOptVecType AllInOne_get_f70_impl(AllInOneType *this) {
  WordsOptVecType ret;
  mol2_cursor_t cur = mol2_table_slice_by_index(&this->cur, 70);
  ret.cur = cur;
  ret.t = GetWordsOptVecVTable();
  return ret;
}
BytesOptVecType AllInOne_get_f71_impl(AllInOneType *this) {
  BytesOptVecType ret;
  mol2_cursor_t cur = mol2_table_slice_by_index(&this->cur, 71);
  ret.cur = cur;
  ret.t = GetBytesOptVecVTable();
  return ret;
}
UnionAType AllInOne_get_f72_impl(AllInOneType *this) {
  UnionAType ret;
  mol2_cursor_t cur = mol2_table_slice_by_index(&this->cur, 72);
  ret.cur = cur;
  ret.t = GetUnionAVTable();
  return ret;
}
TableAType AllInOne_get_f73_impl(AllInOneType *this) {
  TableAType ret;
  mol2_cursor_t cur = mol2_table_slice_by_index(&this->cur, 73);
  ret.cur = cur;
  ret.t = GetTableAVTable();
  return ret;
}
#endif  // MOLECULEC_C2_DECLARATION_ONLY

#ifdef __cplusplus
}
#endif /* __cplusplus */

#endif  // _TYPES_API2_H_
