#![allow(dead_code)]
#![allow(unused_imports)]
extern crate alloc;
use alloc::vec::Vec;
use core::convert::TryInto;
use molecule2::Cursor;

pub struct Byte2 {
    pub cursor: Cursor,
}

impl From<Cursor> for Byte2 {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}

impl Byte2 {
    pub fn len(&self) -> usize {
        2
    }
}

impl Byte2 {
    pub fn get(&self, index: usize) -> u8 {
        let cur = self.cursor.slice_by_offset(1 * index, 1).unwrap();
        cur.into()
    }
}

pub struct Byte3 {
    pub cursor: Cursor,
}

impl From<Cursor> for Byte3 {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}

impl Byte3 {
    pub fn len(&self) -> usize {
        3
    }
}

impl Byte3 {
    pub fn get(&self, index: usize) -> u8 {
        let cur = self.cursor.slice_by_offset(1 * index, 1).unwrap();
        cur.into()
    }
}

pub struct Byte4 {
    pub cursor: Cursor,
}

impl From<Cursor> for Byte4 {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}

impl Byte4 {
    pub fn len(&self) -> usize {
        4
    }
}

impl Byte4 {
    pub fn get(&self, index: usize) -> u8 {
        let cur = self.cursor.slice_by_offset(1 * index, 1).unwrap();
        cur.into()
    }
}

pub struct Byte5 {
    pub cursor: Cursor,
}

impl From<Cursor> for Byte5 {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}

impl Byte5 {
    pub fn len(&self) -> usize {
        5
    }
}

impl Byte5 {
    pub fn get(&self, index: usize) -> u8 {
        let cur = self.cursor.slice_by_offset(1 * index, 1).unwrap();
        cur.into()
    }
}

pub struct Byte6 {
    pub cursor: Cursor,
}

impl From<Cursor> for Byte6 {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}

impl Byte6 {
    pub fn len(&self) -> usize {
        6
    }
}

impl Byte6 {
    pub fn get(&self, index: usize) -> u8 {
        let cur = self.cursor.slice_by_offset(1 * index, 1).unwrap();
        cur.into()
    }
}

pub struct Byte7 {
    pub cursor: Cursor,
}

impl From<Cursor> for Byte7 {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}

impl Byte7 {
    pub fn len(&self) -> usize {
        7
    }
}

impl Byte7 {
    pub fn get(&self, index: usize) -> u8 {
        let cur = self.cursor.slice_by_offset(1 * index, 1).unwrap();
        cur.into()
    }
}

pub struct Byte8 {
    pub cursor: Cursor,
}

impl From<Cursor> for Byte8 {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}

impl Byte8 {
    pub fn len(&self) -> usize {
        8
    }
}

impl Byte8 {
    pub fn get(&self, index: usize) -> u8 {
        let cur = self.cursor.slice_by_offset(1 * index, 1).unwrap();
        cur.into()
    }
}

pub struct Byte9 {
    pub cursor: Cursor,
}

impl From<Cursor> for Byte9 {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}

impl Byte9 {
    pub fn len(&self) -> usize {
        9
    }
}

impl Byte9 {
    pub fn get(&self, index: usize) -> u8 {
        let cur = self.cursor.slice_by_offset(1 * index, 1).unwrap();
        cur.into()
    }
}

pub struct Byte10 {
    pub cursor: Cursor,
}

impl From<Cursor> for Byte10 {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}

impl Byte10 {
    pub fn len(&self) -> usize {
        10
    }
}

impl Byte10 {
    pub fn get(&self, index: usize) -> u8 {
        let cur = self.cursor.slice_by_offset(1 * index, 1).unwrap();
        cur.into()
    }
}

pub struct Byte11 {
    pub cursor: Cursor,
}

impl From<Cursor> for Byte11 {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}

impl Byte11 {
    pub fn len(&self) -> usize {
        11
    }
}

impl Byte11 {
    pub fn get(&self, index: usize) -> u8 {
        let cur = self.cursor.slice_by_offset(1 * index, 1).unwrap();
        cur.into()
    }
}

pub struct Byte12 {
    pub cursor: Cursor,
}

impl From<Cursor> for Byte12 {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}

impl Byte12 {
    pub fn len(&self) -> usize {
        12
    }
}

impl Byte12 {
    pub fn get(&self, index: usize) -> u8 {
        let cur = self.cursor.slice_by_offset(1 * index, 1).unwrap();
        cur.into()
    }
}

pub struct Byte13 {
    pub cursor: Cursor,
}

impl From<Cursor> for Byte13 {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}

impl Byte13 {
    pub fn len(&self) -> usize {
        13
    }
}

impl Byte13 {
    pub fn get(&self, index: usize) -> u8 {
        let cur = self.cursor.slice_by_offset(1 * index, 1).unwrap();
        cur.into()
    }
}

pub struct Byte14 {
    pub cursor: Cursor,
}

impl From<Cursor> for Byte14 {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}

impl Byte14 {
    pub fn len(&self) -> usize {
        14
    }
}

impl Byte14 {
    pub fn get(&self, index: usize) -> u8 {
        let cur = self.cursor.slice_by_offset(1 * index, 1).unwrap();
        cur.into()
    }
}

pub struct Byte15 {
    pub cursor: Cursor,
}

impl From<Cursor> for Byte15 {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}

impl Byte15 {
    pub fn len(&self) -> usize {
        15
    }
}

impl Byte15 {
    pub fn get(&self, index: usize) -> u8 {
        let cur = self.cursor.slice_by_offset(1 * index, 1).unwrap();
        cur.into()
    }
}

pub struct Byte16 {
    pub cursor: Cursor,
}

impl From<Cursor> for Byte16 {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}

impl Byte16 {
    pub fn len(&self) -> usize {
        16
    }
}

impl Byte16 {
    pub fn get(&self, index: usize) -> u8 {
        let cur = self.cursor.slice_by_offset(1 * index, 1).unwrap();
        cur.into()
    }
}

pub struct Word {
    pub cursor: Cursor,
}

impl From<Cursor> for Word {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}

impl Word {
    pub fn len(&self) -> usize {
        2
    }
}

impl Word {
    pub fn get(&self, index: usize) -> u8 {
        let cur = self.cursor.slice_by_offset(1 * index, 1).unwrap();
        cur.into()
    }
}

pub struct Word2 {
    pub cursor: Cursor,
}

impl From<Cursor> for Word2 {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}

impl Word2 {
    pub fn len(&self) -> usize {
        2
    }
}

impl Word2 {
    pub fn get(&self, index: usize) -> Cursor {
        let cur = self.cursor.slice_by_offset(2 * index, 2).unwrap();
        cur.into()
    }
}

pub struct Word3 {
    pub cursor: Cursor,
}

impl From<Cursor> for Word3 {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}

impl Word3 {
    pub fn len(&self) -> usize {
        3
    }
}

impl Word3 {
    pub fn get(&self, index: usize) -> Cursor {
        let cur = self.cursor.slice_by_offset(2 * index, 2).unwrap();
        cur.into()
    }
}

pub struct Word4 {
    pub cursor: Cursor,
}

impl From<Cursor> for Word4 {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}

impl Word4 {
    pub fn len(&self) -> usize {
        4
    }
}

impl Word4 {
    pub fn get(&self, index: usize) -> Cursor {
        let cur = self.cursor.slice_by_offset(2 * index, 2).unwrap();
        cur.into()
    }
}

pub struct Word5 {
    pub cursor: Cursor,
}

impl From<Cursor> for Word5 {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}

impl Word5 {
    pub fn len(&self) -> usize {
        5
    }
}

impl Word5 {
    pub fn get(&self, index: usize) -> Cursor {
        let cur = self.cursor.slice_by_offset(2 * index, 2).unwrap();
        cur.into()
    }
}

pub struct Word6 {
    pub cursor: Cursor,
}

impl From<Cursor> for Word6 {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}

impl Word6 {
    pub fn len(&self) -> usize {
        6
    }
}

impl Word6 {
    pub fn get(&self, index: usize) -> Cursor {
        let cur = self.cursor.slice_by_offset(2 * index, 2).unwrap();
        cur.into()
    }
}

pub struct Word7 {
    pub cursor: Cursor,
}

impl From<Cursor> for Word7 {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}

impl Word7 {
    pub fn len(&self) -> usize {
        7
    }
}

impl Word7 {
    pub fn get(&self, index: usize) -> Cursor {
        let cur = self.cursor.slice_by_offset(2 * index, 2).unwrap();
        cur.into()
    }
}

pub struct Word8 {
    pub cursor: Cursor,
}

impl From<Cursor> for Word8 {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}

impl Word8 {
    pub fn len(&self) -> usize {
        8
    }
}

impl Word8 {
    pub fn get(&self, index: usize) -> Cursor {
        let cur = self.cursor.slice_by_offset(2 * index, 2).unwrap();
        cur.into()
    }
}

pub struct Byte3x3 {
    pub cursor: Cursor,
}

impl From<Cursor> for Byte3x3 {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}

impl Byte3x3 {
    pub fn len(&self) -> usize {
        3
    }
}

impl Byte3x3 {
    pub fn get(&self, index: usize) -> Cursor {
        let cur = self.cursor.slice_by_offset(3 * index, 3).unwrap();
        cur.into()
    }
}

pub struct Byte5x3 {
    pub cursor: Cursor,
}

impl From<Cursor> for Byte5x3 {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}

impl Byte5x3 {
    pub fn len(&self) -> usize {
        3
    }
}

impl Byte5x3 {
    pub fn get(&self, index: usize) -> Cursor {
        let cur = self.cursor.slice_by_offset(5 * index, 5).unwrap();
        cur.into()
    }
}

pub struct Byte7x3 {
    pub cursor: Cursor,
}

impl From<Cursor> for Byte7x3 {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}

impl Byte7x3 {
    pub fn len(&self) -> usize {
        3
    }
}

impl Byte7x3 {
    pub fn get(&self, index: usize) -> Cursor {
        let cur = self.cursor.slice_by_offset(7 * index, 7).unwrap();
        cur.into()
    }
}

pub struct Byte9x3 {
    pub cursor: Cursor,
}

impl From<Cursor> for Byte9x3 {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}

impl Byte9x3 {
    pub fn len(&self) -> usize {
        3
    }
}

impl Byte9x3 {
    pub fn get(&self, index: usize) -> Cursor {
        let cur = self.cursor.slice_by_offset(9 * index, 9).unwrap();
        cur.into()
    }
}

pub struct StructA {
    pub cursor: Cursor,
}

impl From<Cursor> for StructA {
    fn from(cursor: Cursor) -> Self {
        StructA { cursor }
    }
}

impl StructA {
    pub fn f1(&self) -> u8 {
        let cur = self.cursor.slice_by_offset(0, 1).unwrap();
        cur.into()
    }
}

impl StructA {
    pub fn f2(&self) -> u8 {
        let cur = self.cursor.slice_by_offset(1, 1).unwrap();
        cur.into()
    }
}

impl StructA {
    pub fn f3(&self) -> Cursor {
        let cur = self.cursor.slice_by_offset(2, 2).unwrap();
        cur.into()
    }
}

impl StructA {
    pub fn f4(&self) -> Cursor {
        let cur = self.cursor.slice_by_offset(4, 2).unwrap();
        cur.into()
    }
}

pub struct StructB {
    pub cursor: Cursor,
}

impl From<Cursor> for StructB {
    fn from(cursor: Cursor) -> Self {
        StructB { cursor }
    }
}

impl StructB {
    pub fn f1(&self) -> u8 {
        let cur = self.cursor.slice_by_offset(0, 1).unwrap();
        cur.into()
    }
}

impl StructB {
    pub fn f2(&self) -> u8 {
        let cur = self.cursor.slice_by_offset(1, 1).unwrap();
        cur.into()
    }
}

impl StructB {
    pub fn f3(&self) -> Cursor {
        let cur = self.cursor.slice_by_offset(2, 2).unwrap();
        cur.into()
    }
}

impl StructB {
    pub fn f4(&self) -> Cursor {
        let cur = self.cursor.slice_by_offset(4, 3).unwrap();
        cur.into()
    }
}

pub struct StructC {
    pub cursor: Cursor,
}

impl From<Cursor> for StructC {
    fn from(cursor: Cursor) -> Self {
        StructC { cursor }
    }
}

impl StructC {
    pub fn f1(&self) -> u8 {
        let cur = self.cursor.slice_by_offset(0, 1).unwrap();
        cur.into()
    }
}

impl StructC {
    pub fn f2(&self) -> u8 {
        let cur = self.cursor.slice_by_offset(1, 1).unwrap();
        cur.into()
    }
}

impl StructC {
    pub fn f3(&self) -> Cursor {
        let cur = self.cursor.slice_by_offset(2, 2).unwrap();
        cur.into()
    }
}

impl StructC {
    pub fn f4(&self) -> Cursor {
        let cur = self.cursor.slice_by_offset(4, 4).unwrap();
        cur.into()
    }
}

pub struct StructD {
    pub cursor: Cursor,
}

impl From<Cursor> for StructD {
    fn from(cursor: Cursor) -> Self {
        StructD { cursor }
    }
}

impl StructD {
    pub fn f1(&self) -> u8 {
        let cur = self.cursor.slice_by_offset(0, 1).unwrap();
        cur.into()
    }
}

impl StructD {
    pub fn f2(&self) -> u8 {
        let cur = self.cursor.slice_by_offset(1, 1).unwrap();
        cur.into()
    }
}

impl StructD {
    pub fn f3(&self) -> Cursor {
        let cur = self.cursor.slice_by_offset(2, 2).unwrap();
        cur.into()
    }
}

impl StructD {
    pub fn f4(&self) -> Cursor {
        let cur = self.cursor.slice_by_offset(4, 5).unwrap();
        cur.into()
    }
}

pub struct StructE {
    pub cursor: Cursor,
}

impl From<Cursor> for StructE {
    fn from(cursor: Cursor) -> Self {
        StructE { cursor }
    }
}

impl StructE {
    pub fn f1(&self) -> u8 {
        let cur = self.cursor.slice_by_offset(0, 1).unwrap();
        cur.into()
    }
}

impl StructE {
    pub fn f2(&self) -> Cursor {
        let cur = self.cursor.slice_by_offset(1, 2).unwrap();
        cur.into()
    }
}

impl StructE {
    pub fn f3(&self) -> u8 {
        let cur = self.cursor.slice_by_offset(3, 1).unwrap();
        cur.into()
    }
}

impl StructE {
    pub fn f4(&self) -> Cursor {
        let cur = self.cursor.slice_by_offset(4, 2).unwrap();
        cur.into()
    }
}

pub struct StructF {
    pub cursor: Cursor,
}

impl From<Cursor> for StructF {
    fn from(cursor: Cursor) -> Self {
        StructF { cursor }
    }
}

impl StructF {
    pub fn f1(&self) -> u8 {
        let cur = self.cursor.slice_by_offset(0, 1).unwrap();
        cur.into()
    }
}

impl StructF {
    pub fn f2(&self) -> Cursor {
        let cur = self.cursor.slice_by_offset(1, 3).unwrap();
        cur.into()
    }
}

impl StructF {
    pub fn f3(&self) -> u8 {
        let cur = self.cursor.slice_by_offset(4, 1).unwrap();
        cur.into()
    }
}

pub struct StructG {
    pub cursor: Cursor,
}

impl From<Cursor> for StructG {
    fn from(cursor: Cursor) -> Self {
        StructG { cursor }
    }
}

impl StructG {
    pub fn f1(&self) -> Cursor {
        let cur = self.cursor.slice_by_offset(0, 3).unwrap();
        cur.into()
    }
}

impl StructG {
    pub fn f2(&self) -> u8 {
        let cur = self.cursor.slice_by_offset(3, 1).unwrap();
        cur.into()
    }
}

impl StructG {
    pub fn f3(&self) -> Cursor {
        let cur = self.cursor.slice_by_offset(4, 2).unwrap();
        cur.into()
    }
}

impl StructG {
    pub fn f4(&self) -> Word2 {
        let cur = self.cursor.slice_by_offset(6, 4).unwrap();
        cur.into()
    }
}

pub struct StructH {
    pub cursor: Cursor,
}

impl From<Cursor> for StructH {
    fn from(cursor: Cursor) -> Self {
        StructH { cursor }
    }
}

impl StructH {
    pub fn f1(&self) -> Cursor {
        let cur = self.cursor.slice_by_offset(0, 3).unwrap();
        cur.into()
    }
}

impl StructH {
    pub fn f2(&self) -> u8 {
        let cur = self.cursor.slice_by_offset(3, 1).unwrap();
        cur.into()
    }
}

impl StructH {
    pub fn f3(&self) -> Cursor {
        let cur = self.cursor.slice_by_offset(4, 2).unwrap();
        cur.into()
    }
}

impl StructH {
    pub fn f4(&self) -> Cursor {
        let cur = self.cursor.slice_by_offset(6, 4).unwrap();
        cur.into()
    }
}

pub struct StructI {
    pub cursor: Cursor,
}

impl From<Cursor> for StructI {
    fn from(cursor: Cursor) -> Self {
        StructI { cursor }
    }
}

impl StructI {
    pub fn f1(&self) -> Cursor {
        let cur = self.cursor.slice_by_offset(0, 3).unwrap();
        cur.into()
    }
}

impl StructI {
    pub fn f2(&self) -> u8 {
        let cur = self.cursor.slice_by_offset(3, 1).unwrap();
        cur.into()
    }
}

pub struct StructJ {
    pub cursor: Cursor,
}

impl From<Cursor> for StructJ {
    fn from(cursor: Cursor) -> Self {
        StructJ { cursor }
    }
}

impl StructJ {
    pub fn f1(&self) -> Cursor {
        let cur = self.cursor.slice_by_offset(0, 6).unwrap();
        cur.into()
    }
}

impl StructJ {
    pub fn f2(&self) -> u8 {
        let cur = self.cursor.slice_by_offset(6, 1).unwrap();
        cur.into()
    }
}

pub struct StructIx3 {
    pub cursor: Cursor,
}

impl From<Cursor> for StructIx3 {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}

impl StructIx3 {
    pub fn len(&self) -> usize {
        3
    }
}

impl StructIx3 {
    pub fn get(&self, index: usize) -> StructI {
        let cur = self.cursor.slice_by_offset(4 * index, 4).unwrap();
        cur.into()
    }
}

pub struct StructO {
    pub cursor: Cursor,
}

impl From<Cursor> for StructO {
    fn from(cursor: Cursor) -> Self {
        StructO { cursor }
    }
}

impl StructO {
    pub fn f1(&self) -> StructIx3 {
        let cur = self.cursor.slice_by_offset(0, 12).unwrap();
        cur.into()
    }
}

impl StructO {
    pub fn f2(&self) -> u8 {
        let cur = self.cursor.slice_by_offset(12, 1).unwrap();
        cur.into()
    }
}

pub struct StructP {
    pub cursor: Cursor,
}

impl From<Cursor> for StructP {
    fn from(cursor: Cursor) -> Self {
        StructP { cursor }
    }
}

impl StructP {
    pub fn f1(&self) -> StructJ {
        let cur = self.cursor.slice_by_offset(0, 7).unwrap();
        cur.into()
    }
}

impl StructP {
    pub fn f2(&self) -> u8 {
        let cur = self.cursor.slice_by_offset(7, 1).unwrap();
        cur.into()
    }
}

pub struct Bytes {
    pub cursor: Cursor,
}

impl From<Cursor> for Bytes {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}

impl Bytes {
    pub fn len(&self) -> usize {
        self.cursor.fixvec_length()
    }
}

impl Bytes {
    pub fn get(&self, index: usize) -> u8 {
        let cur = self.cursor.fixvec_slice_by_index(1, index).unwrap();
        cur.into()
    }
}

pub struct Words {
    pub cursor: Cursor,
}

impl From<Cursor> for Words {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}

impl Words {
    pub fn len(&self) -> usize {
        self.cursor.fixvec_length()
    }
}

impl Words {
    pub fn get(&self, index: usize) -> Cursor {
        let cur = self.cursor.fixvec_slice_by_index(2, index).unwrap();
        cur.into()
    }
}

pub struct Byte3Vec {
    pub cursor: Cursor,
}

impl From<Cursor> for Byte3Vec {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}

impl Byte3Vec {
    pub fn len(&self) -> usize {
        self.cursor.fixvec_length()
    }
}

impl Byte3Vec {
    pub fn get(&self, index: usize) -> Cursor {
        let cur = self.cursor.fixvec_slice_by_index(3, index).unwrap();
        cur.into()
    }
}

pub struct Byte7Vec {
    pub cursor: Cursor,
}

impl From<Cursor> for Byte7Vec {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}

impl Byte7Vec {
    pub fn len(&self) -> usize {
        self.cursor.fixvec_length()
    }
}

impl Byte7Vec {
    pub fn get(&self, index: usize) -> Cursor {
        let cur = self.cursor.fixvec_slice_by_index(7, index).unwrap();
        cur.into()
    }
}

pub struct StructIVec {
    pub cursor: Cursor,
}

impl From<Cursor> for StructIVec {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}

impl StructIVec {
    pub fn len(&self) -> usize {
        self.cursor.fixvec_length()
    }
}

impl StructIVec {
    pub fn get(&self, index: usize) -> StructI {
        let cur = self.cursor.fixvec_slice_by_index(4, index).unwrap();
        cur.into()
    }
}

pub struct StructJVec {
    pub cursor: Cursor,
}

impl From<Cursor> for StructJVec {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}

impl StructJVec {
    pub fn len(&self) -> usize {
        self.cursor.fixvec_length()
    }
}

impl StructJVec {
    pub fn get(&self, index: usize) -> StructJ {
        let cur = self.cursor.fixvec_slice_by_index(7, index).unwrap();
        cur.into()
    }
}

pub struct StructPVec {
    pub cursor: Cursor,
}

impl From<Cursor> for StructPVec {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}

impl StructPVec {
    pub fn len(&self) -> usize {
        self.cursor.fixvec_length()
    }
}

impl StructPVec {
    pub fn get(&self, index: usize) -> StructP {
        let cur = self.cursor.fixvec_slice_by_index(8, index).unwrap();
        cur.into()
    }
}

pub struct BytesVec {
    pub cursor: Cursor,
}

impl From<Cursor> for BytesVec {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}

impl BytesVec {
    pub fn len(&self) -> usize {
        self.cursor.dynvec_length()
    }
}

impl BytesVec {
    pub fn get(&self, index: usize) -> Cursor {
        let cur = self.cursor.dynvec_slice_by_index(index).unwrap();
        let cur2 = cur.convert_to_rawbytes().unwrap();
        cur2.try_into().unwrap()
    }
}

pub struct WordsVec {
    pub cursor: Cursor,
}

impl From<Cursor> for WordsVec {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}

impl WordsVec {
    pub fn len(&self) -> usize {
        self.cursor.dynvec_length()
    }
}

impl WordsVec {
    pub fn get(&self, index: usize) -> Words {
        let cur = self.cursor.dynvec_slice_by_index(index).unwrap();
        cur.into()
    }
}

pub struct Table0 {
    pub cursor: Cursor,
}

impl From<Cursor> for Table0 {
    fn from(cursor: Cursor) -> Self {
        Table0 { cursor }
    }
}

pub struct Table1 {
    pub cursor: Cursor,
}

impl From<Cursor> for Table1 {
    fn from(cursor: Cursor) -> Self {
        Table1 { cursor }
    }
}

impl Table1 {
    pub fn f1(&self) -> u8 {
        let cur = self.cursor.table_slice_by_index(0).unwrap();
        cur.into()
    }
}

pub struct Table2 {
    pub cursor: Cursor,
}

impl From<Cursor> for Table2 {
    fn from(cursor: Cursor) -> Self {
        Table2 { cursor }
    }
}

impl Table2 {
    pub fn f1(&self) -> u8 {
        let cur = self.cursor.table_slice_by_index(0).unwrap();
        cur.into()
    }
}

impl Table2 {
    pub fn f2(&self) -> Word2 {
        let cur = self.cursor.table_slice_by_index(1).unwrap();
        cur.into()
    }
}

pub struct Table3 {
    pub cursor: Cursor,
}

impl From<Cursor> for Table3 {
    fn from(cursor: Cursor) -> Self {
        Table3 { cursor }
    }
}

impl Table3 {
    pub fn f1(&self) -> u8 {
        let cur = self.cursor.table_slice_by_index(0).unwrap();
        cur.into()
    }
}

impl Table3 {
    pub fn f2(&self) -> Word2 {
        let cur = self.cursor.table_slice_by_index(1).unwrap();
        cur.into()
    }
}

impl Table3 {
    pub fn f3(&self) -> StructA {
        let cur = self.cursor.table_slice_by_index(2).unwrap();
        cur.into()
    }
}

pub struct Table4 {
    pub cursor: Cursor,
}

impl From<Cursor> for Table4 {
    fn from(cursor: Cursor) -> Self {
        Table4 { cursor }
    }
}

impl Table4 {
    pub fn f1(&self) -> u8 {
        let cur = self.cursor.table_slice_by_index(0).unwrap();
        cur.into()
    }
}

impl Table4 {
    pub fn f2(&self) -> Word2 {
        let cur = self.cursor.table_slice_by_index(1).unwrap();
        cur.into()
    }
}

impl Table4 {
    pub fn f3(&self) -> StructA {
        let cur = self.cursor.table_slice_by_index(2).unwrap();
        cur.into()
    }
}

impl Table4 {
    pub fn f4(&self) -> Cursor {
        let cur = self.cursor.table_slice_by_index(3).unwrap();
        let cur2 = cur.convert_to_rawbytes().unwrap();
        cur2.try_into().unwrap()
    }
}

pub struct Table5 {
    pub cursor: Cursor,
}

impl From<Cursor> for Table5 {
    fn from(cursor: Cursor) -> Self {
        Table5 { cursor }
    }
}

impl Table5 {
    pub fn f1(&self) -> u8 {
        let cur = self.cursor.table_slice_by_index(0).unwrap();
        cur.into()
    }
}

impl Table5 {
    pub fn f2(&self) -> Word2 {
        let cur = self.cursor.table_slice_by_index(1).unwrap();
        cur.into()
    }
}

impl Table5 {
    pub fn f3(&self) -> StructA {
        let cur = self.cursor.table_slice_by_index(2).unwrap();
        cur.into()
    }
}

impl Table5 {
    pub fn f4(&self) -> Cursor {
        let cur = self.cursor.table_slice_by_index(3).unwrap();
        let cur2 = cur.convert_to_rawbytes().unwrap();
        cur2.try_into().unwrap()
    }
}

impl Table5 {
    pub fn f5(&self) -> BytesVec {
        let cur = self.cursor.table_slice_by_index(4).unwrap();
        cur.into()
    }
}

pub struct Table6 {
    pub cursor: Cursor,
}

impl From<Cursor> for Table6 {
    fn from(cursor: Cursor) -> Self {
        Table6 { cursor }
    }
}

impl Table6 {
    pub fn f1(&self) -> u8 {
        let cur = self.cursor.table_slice_by_index(0).unwrap();
        cur.into()
    }
}

impl Table6 {
    pub fn f2(&self) -> Word2 {
        let cur = self.cursor.table_slice_by_index(1).unwrap();
        cur.into()
    }
}

impl Table6 {
    pub fn f3(&self) -> StructA {
        let cur = self.cursor.table_slice_by_index(2).unwrap();
        cur.into()
    }
}

impl Table6 {
    pub fn f4(&self) -> Cursor {
        let cur = self.cursor.table_slice_by_index(3).unwrap();
        let cur2 = cur.convert_to_rawbytes().unwrap();
        cur2.try_into().unwrap()
    }
}

impl Table6 {
    pub fn f5(&self) -> BytesVec {
        let cur = self.cursor.table_slice_by_index(4).unwrap();
        cur.into()
    }
}

impl Table6 {
    pub fn f6(&self) -> Table5 {
        let cur = self.cursor.table_slice_by_index(5).unwrap();
        cur.into()
    }
}
// warning: ByteOpt not implemented for Rust
pub struct ByteOpt {
    pub cursor: Cursor,
}
impl From<Cursor> for ByteOpt {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}
// warning: WordOpt not implemented for Rust
pub struct WordOpt {
    pub cursor: Cursor,
}
impl From<Cursor> for WordOpt {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}
// warning: StructAOpt not implemented for Rust
pub struct StructAOpt {
    pub cursor: Cursor,
}
impl From<Cursor> for StructAOpt {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}
// warning: StructPOpt not implemented for Rust
pub struct StructPOpt {
    pub cursor: Cursor,
}
impl From<Cursor> for StructPOpt {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}
// warning: BytesOpt not implemented for Rust
pub struct BytesOpt {
    pub cursor: Cursor,
}
impl From<Cursor> for BytesOpt {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}
// warning: WordsOpt not implemented for Rust
pub struct WordsOpt {
    pub cursor: Cursor,
}
impl From<Cursor> for WordsOpt {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}
// warning: BytesVecOpt not implemented for Rust
pub struct BytesVecOpt {
    pub cursor: Cursor,
}
impl From<Cursor> for BytesVecOpt {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}
// warning: WordsVecOpt not implemented for Rust
pub struct WordsVecOpt {
    pub cursor: Cursor,
}
impl From<Cursor> for WordsVecOpt {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}
// warning: Table0Opt not implemented for Rust
pub struct Table0Opt {
    pub cursor: Cursor,
}
impl From<Cursor> for Table0Opt {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}
// warning: Table6Opt not implemented for Rust
pub struct Table6Opt {
    pub cursor: Cursor,
}
impl From<Cursor> for Table6Opt {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}
// warning: Table6OptOpt not implemented for Rust
pub struct Table6OptOpt {
    pub cursor: Cursor,
}
impl From<Cursor> for Table6OptOpt {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}

pub struct ByteOptVec {
    pub cursor: Cursor,
}

impl From<Cursor> for ByteOptVec {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}

impl ByteOptVec {
    pub fn len(&self) -> usize {
        self.cursor.dynvec_length()
    }
}

impl ByteOptVec {
    pub fn get(&self, index: usize) -> Option<u8> {
        let cur = self.cursor.dynvec_slice_by_index(index).unwrap();
        if cur.option_is_none() {
            None
        } else {
            Some(cur.into())
        }
    }
}

pub struct WordOptVec {
    pub cursor: Cursor,
}

impl From<Cursor> for WordOptVec {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}

impl WordOptVec {
    pub fn len(&self) -> usize {
        self.cursor.dynvec_length()
    }
}

impl WordOptVec {
    pub fn get(&self, index: usize) -> Option<Cursor> {
        let cur = self.cursor.dynvec_slice_by_index(index).unwrap();
        if cur.option_is_none() {
            None
        } else {
            Some(cur.into())
        }
    }
}

pub struct WordsOptVec {
    pub cursor: Cursor,
}

impl From<Cursor> for WordsOptVec {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}

impl WordsOptVec {
    pub fn len(&self) -> usize {
        self.cursor.dynvec_length()
    }
}

impl WordsOptVec {
    pub fn get(&self, index: usize) -> Option<Words> {
        let cur = self.cursor.dynvec_slice_by_index(index).unwrap();
        if cur.option_is_none() {
            None
        } else {
            Some(cur.into())
        }
    }
}

pub struct BytesOptVec {
    pub cursor: Cursor,
}

impl From<Cursor> for BytesOptVec {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}

impl BytesOptVec {
    pub fn len(&self) -> usize {
        self.cursor.dynvec_length()
    }
}

impl BytesOptVec {
    pub fn get(&self, index: usize) -> Option<Cursor> {
        let cur = self.cursor.dynvec_slice_by_index(index).unwrap();
        if cur.option_is_none() {
            None
        } else {
            Some(cur.into())
        }
    }
}

pub struct UnionA {
    pub cursor: Cursor,
}

impl From<Cursor> for UnionA {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}

impl UnionA {
    pub fn item_id(&self) -> usize {
        let union = self.cursor.union_unpack();
        union.item_id
    }
}

impl UnionA {
    pub fn as_byte(&self) -> u8 {
        let union = self.cursor.union_unpack();
        let cur = union.cursor.clone();
        cur.into()
    }
}

impl UnionA {
    pub fn as_word(&self) -> Cursor {
        let union = self.cursor.union_unpack();
        let cur = union.cursor.clone();
        cur.into()
    }
}

impl UnionA {
    pub fn as_structa(&self) -> StructA {
        let union = self.cursor.union_unpack();
        let cur = union.cursor.clone();
        cur.into()
    }
}

impl UnionA {
    pub fn as_bytes(&self) -> Cursor {
        let union = self.cursor.union_unpack();
        let cur = union.cursor.clone();
        let cur2 = cur.convert_to_rawbytes().unwrap();
        cur2.try_into().unwrap()
    }
}

impl UnionA {
    pub fn as_words(&self) -> Words {
        let union = self.cursor.union_unpack();
        let cur = union.cursor.clone();
        cur.into()
    }
}

impl UnionA {
    pub fn as_table0(&self) -> Table0 {
        let union = self.cursor.union_unpack();
        let cur = union.cursor.clone();
        cur.into()
    }
}

impl UnionA {
    pub fn as_table6(&self) -> Table6 {
        let union = self.cursor.union_unpack();
        let cur = union.cursor.clone();
        cur.into()
    }
}

impl UnionA {
    pub fn as_table6opt(&self) -> Option<Table6> {
        let union = self.cursor.union_unpack();
        let cur = union.cursor.clone();
        if cur.option_is_none() {
            None
        } else {
            Some(cur.into())
        }
    }
}

pub struct TableA {
    pub cursor: Cursor,
}

impl From<Cursor> for TableA {
    fn from(cursor: Cursor) -> Self {
        TableA { cursor }
    }
}

impl TableA {
    pub fn f1(&self) -> Word2 {
        let cur = self.cursor.table_slice_by_index(0).unwrap();
        cur.into()
    }
}

impl TableA {
    pub fn f2(&self) -> StructA {
        let cur = self.cursor.table_slice_by_index(1).unwrap();
        cur.into()
    }
}

impl TableA {
    pub fn f3(&self) -> Cursor {
        let cur = self.cursor.table_slice_by_index(2).unwrap();
        let cur2 = cur.convert_to_rawbytes().unwrap();
        cur2.try_into().unwrap()
    }
}

impl TableA {
    pub fn f4(&self) -> BytesVec {
        let cur = self.cursor.table_slice_by_index(3).unwrap();
        cur.into()
    }
}

impl TableA {
    pub fn f5(&self) -> Table1 {
        let cur = self.cursor.table_slice_by_index(4).unwrap();
        cur.into()
    }
}

impl TableA {
    pub fn f6(&self) -> Option<Cursor> {
        let cur = self.cursor.table_slice_by_index(5).unwrap();
        if cur.option_is_none() {
            None
        } else {
            Some(cur.into())
        }
    }
}

impl TableA {
    pub fn f7(&self) -> UnionA {
        let cur = self.cursor.table_slice_by_index(6).unwrap();
        cur.into()
    }
}

impl TableA {
    pub fn f8(&self) -> u8 {
        let cur = self.cursor.table_slice_by_index(7).unwrap();
        cur.into()
    }
}

pub struct AllInOne {
    pub cursor: Cursor,
}

impl From<Cursor> for AllInOne {
    fn from(cursor: Cursor) -> Self {
        AllInOne { cursor }
    }
}

impl AllInOne {
    pub fn f0(&self) -> u8 {
        let cur = self.cursor.table_slice_by_index(0).unwrap();
        cur.into()
    }
}

impl AllInOne {
    pub fn f1(&self) -> Cursor {
        let cur = self.cursor.table_slice_by_index(1).unwrap();
        cur.into()
    }
}

impl AllInOne {
    pub fn f2(&self) -> Cursor {
        let cur = self.cursor.table_slice_by_index(2).unwrap();
        cur.into()
    }
}

impl AllInOne {
    pub fn f3(&self) -> Cursor {
        let cur = self.cursor.table_slice_by_index(3).unwrap();
        cur.into()
    }
}

impl AllInOne {
    pub fn f4(&self) -> Cursor {
        let cur = self.cursor.table_slice_by_index(4).unwrap();
        cur.into()
    }
}

impl AllInOne {
    pub fn f5(&self) -> Cursor {
        let cur = self.cursor.table_slice_by_index(5).unwrap();
        cur.into()
    }
}

impl AllInOne {
    pub fn f6(&self) -> Cursor {
        let cur = self.cursor.table_slice_by_index(6).unwrap();
        cur.into()
    }
}

impl AllInOne {
    pub fn f7(&self) -> Cursor {
        let cur = self.cursor.table_slice_by_index(7).unwrap();
        cur.into()
    }
}

impl AllInOne {
    pub fn f8(&self) -> Cursor {
        let cur = self.cursor.table_slice_by_index(8).unwrap();
        cur.into()
    }
}

impl AllInOne {
    pub fn f9(&self) -> Cursor {
        let cur = self.cursor.table_slice_by_index(9).unwrap();
        cur.into()
    }
}

impl AllInOne {
    pub fn f10(&self) -> Cursor {
        let cur = self.cursor.table_slice_by_index(10).unwrap();
        cur.into()
    }
}

impl AllInOne {
    pub fn f11(&self) -> Cursor {
        let cur = self.cursor.table_slice_by_index(11).unwrap();
        cur.into()
    }
}

impl AllInOne {
    pub fn f12(&self) -> Cursor {
        let cur = self.cursor.table_slice_by_index(12).unwrap();
        cur.into()
    }
}

impl AllInOne {
    pub fn f13(&self) -> Cursor {
        let cur = self.cursor.table_slice_by_index(13).unwrap();
        cur.into()
    }
}

impl AllInOne {
    pub fn f14(&self) -> Cursor {
        let cur = self.cursor.table_slice_by_index(14).unwrap();
        cur.into()
    }
}

impl AllInOne {
    pub fn f15(&self) -> Cursor {
        let cur = self.cursor.table_slice_by_index(15).unwrap();
        cur.into()
    }
}

impl AllInOne {
    pub fn f16(&self) -> Cursor {
        let cur = self.cursor.table_slice_by_index(16).unwrap();
        cur.into()
    }
}

impl AllInOne {
    pub fn f17(&self) -> Word2 {
        let cur = self.cursor.table_slice_by_index(17).unwrap();
        cur.into()
    }
}

impl AllInOne {
    pub fn f18(&self) -> Word3 {
        let cur = self.cursor.table_slice_by_index(18).unwrap();
        cur.into()
    }
}

impl AllInOne {
    pub fn f19(&self) -> Word4 {
        let cur = self.cursor.table_slice_by_index(19).unwrap();
        cur.into()
    }
}

impl AllInOne {
    pub fn f20(&self) -> Word5 {
        let cur = self.cursor.table_slice_by_index(20).unwrap();
        cur.into()
    }
}

impl AllInOne {
    pub fn f21(&self) -> Word6 {
        let cur = self.cursor.table_slice_by_index(21).unwrap();
        cur.into()
    }
}

impl AllInOne {
    pub fn f22(&self) -> Word7 {
        let cur = self.cursor.table_slice_by_index(22).unwrap();
        cur.into()
    }
}

impl AllInOne {
    pub fn f23(&self) -> Word8 {
        let cur = self.cursor.table_slice_by_index(23).unwrap();
        cur.into()
    }
}

impl AllInOne {
    pub fn f24(&self) -> Byte3x3 {
        let cur = self.cursor.table_slice_by_index(24).unwrap();
        cur.into()
    }
}

impl AllInOne {
    pub fn f25(&self) -> Byte5x3 {
        let cur = self.cursor.table_slice_by_index(25).unwrap();
        cur.into()
    }
}

impl AllInOne {
    pub fn f26(&self) -> Byte7x3 {
        let cur = self.cursor.table_slice_by_index(26).unwrap();
        cur.into()
    }
}

impl AllInOne {
    pub fn f27(&self) -> Byte9x3 {
        let cur = self.cursor.table_slice_by_index(27).unwrap();
        cur.into()
    }
}

impl AllInOne {
    pub fn f28(&self) -> StructA {
        let cur = self.cursor.table_slice_by_index(28).unwrap();
        cur.into()
    }
}

impl AllInOne {
    pub fn f29(&self) -> StructB {
        let cur = self.cursor.table_slice_by_index(29).unwrap();
        cur.into()
    }
}

impl AllInOne {
    pub fn f30(&self) -> StructC {
        let cur = self.cursor.table_slice_by_index(30).unwrap();
        cur.into()
    }
}

impl AllInOne {
    pub fn f31(&self) -> StructD {
        let cur = self.cursor.table_slice_by_index(31).unwrap();
        cur.into()
    }
}

impl AllInOne {
    pub fn f32(&self) -> StructE {
        let cur = self.cursor.table_slice_by_index(32).unwrap();
        cur.into()
    }
}

impl AllInOne {
    pub fn f33(&self) -> StructF {
        let cur = self.cursor.table_slice_by_index(33).unwrap();
        cur.into()
    }
}

impl AllInOne {
    pub fn f34(&self) -> StructG {
        let cur = self.cursor.table_slice_by_index(34).unwrap();
        cur.into()
    }
}

impl AllInOne {
    pub fn f35(&self) -> StructH {
        let cur = self.cursor.table_slice_by_index(35).unwrap();
        cur.into()
    }
}

impl AllInOne {
    pub fn f36(&self) -> StructI {
        let cur = self.cursor.table_slice_by_index(36).unwrap();
        cur.into()
    }
}

impl AllInOne {
    pub fn f37(&self) -> StructJ {
        let cur = self.cursor.table_slice_by_index(37).unwrap();
        cur.into()
    }
}

impl AllInOne {
    pub fn f38(&self) -> StructIx3 {
        let cur = self.cursor.table_slice_by_index(38).unwrap();
        cur.into()
    }
}

impl AllInOne {
    pub fn f39(&self) -> StructO {
        let cur = self.cursor.table_slice_by_index(39).unwrap();
        cur.into()
    }
}

impl AllInOne {
    pub fn f40(&self) -> StructP {
        let cur = self.cursor.table_slice_by_index(40).unwrap();
        cur.into()
    }
}

impl AllInOne {
    pub fn f41(&self) -> Cursor {
        let cur = self.cursor.table_slice_by_index(41).unwrap();
        let cur2 = cur.convert_to_rawbytes().unwrap();
        cur2.try_into().unwrap()
    }
}

impl AllInOne {
    pub fn f42(&self) -> Words {
        let cur = self.cursor.table_slice_by_index(42).unwrap();
        cur.into()
    }
}

impl AllInOne {
    pub fn f43(&self) -> Byte3Vec {
        let cur = self.cursor.table_slice_by_index(43).unwrap();
        cur.into()
    }
}

impl AllInOne {
    pub fn f44(&self) -> Byte7Vec {
        let cur = self.cursor.table_slice_by_index(44).unwrap();
        cur.into()
    }
}

impl AllInOne {
    pub fn f45(&self) -> StructIVec {
        let cur = self.cursor.table_slice_by_index(45).unwrap();
        cur.into()
    }
}

impl AllInOne {
    pub fn f46(&self) -> StructJVec {
        let cur = self.cursor.table_slice_by_index(46).unwrap();
        cur.into()
    }
}

impl AllInOne {
    pub fn f47(&self) -> StructPVec {
        let cur = self.cursor.table_slice_by_index(47).unwrap();
        cur.into()
    }
}

impl AllInOne {
    pub fn f48(&self) -> BytesVec {
        let cur = self.cursor.table_slice_by_index(48).unwrap();
        cur.into()
    }
}

impl AllInOne {
    pub fn f49(&self) -> WordsVec {
        let cur = self.cursor.table_slice_by_index(49).unwrap();
        cur.into()
    }
}

impl AllInOne {
    pub fn f50(&self) -> Table0 {
        let cur = self.cursor.table_slice_by_index(50).unwrap();
        cur.into()
    }
}

impl AllInOne {
    pub fn f51(&self) -> Table1 {
        let cur = self.cursor.table_slice_by_index(51).unwrap();
        cur.into()
    }
}

impl AllInOne {
    pub fn f52(&self) -> Table2 {
        let cur = self.cursor.table_slice_by_index(52).unwrap();
        cur.into()
    }
}

impl AllInOne {
    pub fn f53(&self) -> Table3 {
        let cur = self.cursor.table_slice_by_index(53).unwrap();
        cur.into()
    }
}

impl AllInOne {
    pub fn f54(&self) -> Table4 {
        let cur = self.cursor.table_slice_by_index(54).unwrap();
        cur.into()
    }
}

impl AllInOne {
    pub fn f55(&self) -> Table5 {
        let cur = self.cursor.table_slice_by_index(55).unwrap();
        cur.into()
    }
}

impl AllInOne {
    pub fn f56(&self) -> Table6 {
        let cur = self.cursor.table_slice_by_index(56).unwrap();
        cur.into()
    }
}

impl AllInOne {
    pub fn f57(&self) -> Option<u8> {
        let cur = self.cursor.table_slice_by_index(57).unwrap();
        if cur.option_is_none() {
            None
        } else {
            Some(cur.into())
        }
    }
}

impl AllInOne {
    pub fn f58(&self) -> Option<Cursor> {
        let cur = self.cursor.table_slice_by_index(58).unwrap();
        if cur.option_is_none() {
            None
        } else {
            Some(cur.into())
        }
    }
}

impl AllInOne {
    pub fn f59(&self) -> Option<StructA> {
        let cur = self.cursor.table_slice_by_index(59).unwrap();
        if cur.option_is_none() {
            None
        } else {
            Some(cur.into())
        }
    }
}

impl AllInOne {
    pub fn f60(&self) -> Option<StructP> {
        let cur = self.cursor.table_slice_by_index(60).unwrap();
        if cur.option_is_none() {
            None
        } else {
            Some(cur.into())
        }
    }
}

impl AllInOne {
    pub fn f61(&self) -> Option<Cursor> {
        let cur = self.cursor.table_slice_by_index(61).unwrap();
        if cur.option_is_none() {
            None
        } else {
            Some(cur.into())
        }
    }
}

impl AllInOne {
    pub fn f62(&self) -> Option<Words> {
        let cur = self.cursor.table_slice_by_index(62).unwrap();
        if cur.option_is_none() {
            None
        } else {
            Some(cur.into())
        }
    }
}

impl AllInOne {
    pub fn f63(&self) -> Option<BytesVec> {
        let cur = self.cursor.table_slice_by_index(63).unwrap();
        if cur.option_is_none() {
            None
        } else {
            Some(cur.into())
        }
    }
}

impl AllInOne {
    pub fn f64(&self) -> Option<WordsVec> {
        let cur = self.cursor.table_slice_by_index(64).unwrap();
        if cur.option_is_none() {
            None
        } else {
            Some(cur.into())
        }
    }
}

impl AllInOne {
    pub fn f65(&self) -> Option<Table0> {
        let cur = self.cursor.table_slice_by_index(65).unwrap();
        if cur.option_is_none() {
            None
        } else {
            Some(cur.into())
        }
    }
}

impl AllInOne {
    pub fn f66(&self) -> Option<Table6> {
        let cur = self.cursor.table_slice_by_index(66).unwrap();
        if cur.option_is_none() {
            None
        } else {
            Some(cur.into())
        }
    }
}

impl AllInOne {
    pub fn f67(&self) -> Option<Option<Table6>> {
        let cur = self.cursor.table_slice_by_index(67).unwrap();
        if cur.option_is_none() {
            None
        } else {
            Some(Some(cur.into()))
        }
    }
}

impl AllInOne {
    pub fn f68(&self) -> ByteOptVec {
        let cur = self.cursor.table_slice_by_index(68).unwrap();
        cur.into()
    }
}

impl AllInOne {
    pub fn f69(&self) -> WordOptVec {
        let cur = self.cursor.table_slice_by_index(69).unwrap();
        cur.into()
    }
}

impl AllInOne {
    pub fn f70(&self) -> WordsOptVec {
        let cur = self.cursor.table_slice_by_index(70).unwrap();
        cur.into()
    }
}

impl AllInOne {
    pub fn f71(&self) -> BytesOptVec {
        let cur = self.cursor.table_slice_by_index(71).unwrap();
        cur.into()
    }
}

impl AllInOne {
    pub fn f72(&self) -> UnionA {
        let cur = self.cursor.table_slice_by_index(72).unwrap();
        cur.into()
    }
}

impl AllInOne {
    pub fn f73(&self) -> TableA {
        let cur = self.cursor.table_slice_by_index(73).unwrap();
        cur.into()
    }
}
