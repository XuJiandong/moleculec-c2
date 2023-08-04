
#![allow(dead_code)]
#![allow(unused_imports)]
extern crate alloc;
use alloc::vec::Vec;
use core::convert::TryInto;
use molecule2::Cursor;
use molecule2::Error;

pub struct SampleByte2 {
    pub cursor: Cursor,
}
impl From<Cursor> for SampleByte2 {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}
impl SampleByte2 {
    pub fn len(&self) -> usize {
        2
    }
}
impl SampleByte2 {
    pub fn get(&self, index: usize) -> Result<u8, Error> {
        let cur = self.cursor.slice_by_offset(1 * index, 1)?;
        cur.try_into()
    }
}
pub struct Uint8 {
    pub cursor: Cursor,
}
impl From<Cursor> for Uint8 {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}
impl Uint8 {
    pub fn len(&self) -> usize {
        1
    }
}
impl Uint8 {
    pub fn get(&self, index: usize) -> Result<u8, Error> {
        let cur = self.cursor.slice_by_offset(1 * index, 1)?;
        cur.try_into()
    }
}
pub struct Int8 {
    pub cursor: Cursor,
}
impl From<Cursor> for Int8 {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}
impl Int8 {
    pub fn len(&self) -> usize {
        1
    }
}
impl Int8 {
    pub fn get(&self, index: usize) -> Result<u8, Error> {
        let cur = self.cursor.slice_by_offset(1 * index, 1)?;
        cur.try_into()
    }
}
pub struct Uint16 {
    pub cursor: Cursor,
}
impl From<Cursor> for Uint16 {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}
impl Uint16 {
    pub fn len(&self) -> usize {
        2
    }
}
impl Uint16 {
    pub fn get(&self, index: usize) -> Result<u8, Error> {
        let cur = self.cursor.slice_by_offset(1 * index, 1)?;
        cur.try_into()
    }
}
pub struct Int16 {
    pub cursor: Cursor,
}
impl From<Cursor> for Int16 {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}
impl Int16 {
    pub fn len(&self) -> usize {
        2
    }
}
impl Int16 {
    pub fn get(&self, index: usize) -> Result<u8, Error> {
        let cur = self.cursor.slice_by_offset(1 * index, 1)?;
        cur.try_into()
    }
}
pub struct Uint32 {
    pub cursor: Cursor,
}
impl From<Cursor> for Uint32 {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}
impl Uint32 {
    pub fn len(&self) -> usize {
        4
    }
}
impl Uint32 {
    pub fn get(&self, index: usize) -> Result<u8, Error> {
        let cur = self.cursor.slice_by_offset(1 * index, 1)?;
        cur.try_into()
    }
}
pub struct Int32 {
    pub cursor: Cursor,
}
impl From<Cursor> for Int32 {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}
impl Int32 {
    pub fn len(&self) -> usize {
        4
    }
}
impl Int32 {
    pub fn get(&self, index: usize) -> Result<u8, Error> {
        let cur = self.cursor.slice_by_offset(1 * index, 1)?;
        cur.try_into()
    }
}
pub struct Uint64 {
    pub cursor: Cursor,
}
impl From<Cursor> for Uint64 {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}
impl Uint64 {
    pub fn len(&self) -> usize {
        8
    }
}
impl Uint64 {
    pub fn get(&self, index: usize) -> Result<u8, Error> {
        let cur = self.cursor.slice_by_offset(1 * index, 1)?;
        cur.try_into()
    }
}
pub struct Int64 {
    pub cursor: Cursor,
}
impl From<Cursor> for Int64 {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}
impl Int64 {
    pub fn len(&self) -> usize {
        8
    }
}
impl Int64 {
    pub fn get(&self, index: usize) -> Result<u8, Error> {
        let cur = self.cursor.slice_by_offset(1 * index, 1)?;
        cur.try_into()
    }
}
pub struct SampleFixedVector {
    pub cursor: Cursor,
}
impl From<Cursor> for SampleFixedVector {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}
impl SampleFixedVector {
    pub fn len(&self) -> Result<usize, Error> {
        self.cursor.fixvec_length()
    }
}
impl SampleFixedVector {
    pub fn get(&self, index: usize) -> Result<u8, Error> {
        let cur = self.cursor.fixvec_slice_by_index(1, index)?;
        cur.try_into()
    }
}
pub struct SampleDynVector {
    pub cursor: Cursor,
}
impl From<Cursor> for SampleDynVector {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}
impl SampleDynVector {
    pub fn len(&self) -> Result<usize, Error> {
        self.cursor.dynvec_length()
    }
}
impl SampleDynVector {
    pub fn get(&self, index: usize) -> Result<Cursor, Error> {
        let cur = self.cursor.dynvec_slice_by_index(index)?;
        cur.convert_to_rawbytes()
    }
}
pub struct SampleUint64Vector {
    pub cursor: Cursor,
}
impl From<Cursor> for SampleUint64Vector {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}
impl SampleUint64Vector {
    pub fn len(&self) -> Result<usize, Error> {
        self.cursor.fixvec_length()
    }
}
impl SampleUint64Vector {
    pub fn get(&self, index: usize) -> Result<u64, Error> {
        let cur = self.cursor.fixvec_slice_by_index(8, index)?;
        cur.try_into()
    }
}
pub struct SampleStruct {
    pub cursor: Cursor,
}
impl From<Cursor> for SampleStruct {
    fn from(cursor: Cursor) -> Self {
        SampleStruct { cursor }
    }
}
impl SampleStruct {
    pub fn u32(&self) -> Result<u32, Error> {
        let cur = self.cursor.slice_by_offset(0, 4)?;
        cur.try_into()
    }
}

impl SampleStruct {
    pub fn byte2(&self) -> Result<Cursor, Error> {
        let cur = self.cursor.slice_by_offset(4, 2)?;
        Ok(cur)
    }
}
pub struct SampleTable {
    pub cursor: Cursor,
}
impl From<Cursor> for SampleTable {
    fn from(cursor: Cursor) -> Self {
        SampleTable { cursor }
    }
}
impl SampleTable {
    pub fn byte_2d_vector(&self) -> Result<SampleDynVector, Error> {
        let cur = self.cursor.table_slice_by_index(0)?;
        Ok(cur.into())
    }
}

impl SampleTable {
    pub fn byte2(&self) -> Result<Cursor, Error> {
        let cur = self.cursor.table_slice_by_index(1)?;
        Ok(cur)
    }
}

impl SampleTable {
    pub fn u64_vector(&self) -> Result<SampleUint64Vector, Error> {
        let cur = self.cursor.table_slice_by_index(2)?;
        Ok(cur.into())
    }
}
pub struct SampleUnion {
    pub cursor: Cursor,
}
impl From<Cursor> for SampleUnion {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}
impl SampleUnion {
    pub fn item_id(&self) -> Result<usize, Error> {
        let item = self.cursor.union_unpack()?;
        Ok(item.item_id)
    }
}
impl SampleUnion {
    pub fn as_samplestruct(&self) -> Result<SampleStruct, Error> {
        let item = self.cursor.union_unpack()?;
        let cur = item.cursor.clone();
        Ok(cur.into())
    }
}
impl SampleUnion {
    pub fn as_sampletable(&self) -> Result<SampleTable, Error> {
        let item = self.cursor.union_unpack()?;
        let cur = item.cursor.clone();
        Ok(cur.into())
    }
}
pub struct SampleOptionTable {
    pub cursor: Cursor,
}
impl From<Cursor> for SampleOptionTable {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}
