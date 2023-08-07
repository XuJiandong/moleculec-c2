
#![allow(dead_code)]
#![allow(unused_imports)]
extern crate alloc;
use alloc::vec::Vec;
use core::convert::TryInto;
use molecule2::Cursor;
use molecule2::Error;

use super::blockchain::*;
pub struct ScriptVec {
    pub cursor: Cursor,
}
impl From<Cursor> for ScriptVec {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}
impl ScriptVec {
    pub fn len(&self) -> Result<usize, Error> {
        self.cursor.dynvec_length()
    }
}
impl ScriptVec {
    pub fn get(&self, index: usize) -> Result<Script, Error> {
        let cur = self.cursor.dynvec_slice_by_index(index)?;
        Ok(cur.into())
    }
}
pub struct ScriptVecOpt {
    pub cursor: Cursor,
}
impl From<Cursor> for ScriptVecOpt {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}
