#![allow(dead_code)]
#![allow(unused_imports)]
extern crate alloc;
use alloc::vec::Vec;
use molecule2::Cursor;

use super::blockchain::*;
pub struct ScriptVec {
    cursor: Cursor,
}

impl From<Cursor> for ScriptVec {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}

impl ScriptVec {
    pub fn len(&self) -> usize {
        self.cursor.dynvec_length()
    }
}

impl ScriptVec {
    pub fn get(&self, index: usize) -> Script {
        let cur = self.cursor.dynvec_slice_by_index(index).unwrap();
        cur.into()
    }
}
// warning: ScriptVecOpt not implemented for Rust
pub struct ScriptVecOpt {
    cursor: Cursor,
}
impl From<Cursor> for ScriptVecOpt {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}
