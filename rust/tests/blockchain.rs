extern crate alloc;

use alloc::vec::Vec;
use molecule2::CursorType;

pub struct TransactionVec {
    cursor: CursorType,
}

impl TransactionVec {
    pub fn get(&self, index: usize) -> Transaction {
        let cursor = self.cursor.dynvec_slice_by_index(index).unwrap();
        Transaction { cursor }
    }
    pub fn len(&self) -> usize {
        self.cursor.dynvec_length()
    }
}

pub struct Transaction {
    cursor: CursorType,
}

impl From<CursorType> for Transaction {
    fn from(cursor: CursorType) -> Self {
        Transaction { cursor }
    }
}

impl Transaction {
    pub fn get_raw(&self) -> RawTransaction {
        let cursor = self.cursor.table_slice_by_index(0).unwrap();
        cursor.into()
    }
    pub fn get_witness(&self) -> WitnessArgs {
        let cursor = self.cursor.table_slice_by_index(1).unwrap();
        cursor.into()
    }
}

pub struct RawTransaction {
    cursor: CursorType,
}

impl From<CursorType> for RawTransaction {
    fn from(cursor: CursorType) -> Self {
        RawTransaction { cursor }
    }
}

impl RawTransaction {
    pub fn get_version(&self) -> u32 {
        let cursor = self.cursor.table_slice_by_index(1).unwrap();
        cursor.into()
    }
    pub fn get_cell_deps(&self) -> CellDepVec {
        let cursor = self.cursor.table_slice_by_index(1).unwrap();
        cursor.into()
    }

    pub fn get_inputs(&self) -> CellInputVec {
        let cursor = self.cursor.table_slice_by_index(3).unwrap();
        cursor.into()
    }
    pub fn get_outputs(&self) -> CellOutputVec {
        let cursor = self.cursor.table_slice_by_index(4).unwrap();
        cursor.into()
    }
    pub fn get_outputs_data(&self) -> BytesVec {
        let cursor = self.cursor.table_slice_by_index(5).unwrap();
        cursor.into()
    }
}

pub struct CellDepVec {
    cursor: CursorType,
}

impl From<CursorType> for CellDepVec {
    fn from(cursor: CursorType) -> Self {
        Self { cursor }
    }
}

impl CellDepVec {
    pub fn get(&self, index: usize) -> CellDep {
        let cursor = self.cursor.fixvec_slice_by_index(37, index).unwrap();
        cursor.into()
    }
    pub fn len(&self) -> usize {
        self.cursor.fixvec_length()
    }
}
pub struct CellDep {
    cursor: CursorType,
}

impl From<CursorType> for CellDep {
    fn from(cursor: CursorType) -> Self {
        Self { cursor }
    }
}

impl CellDep {
    pub fn get_out_point(&self) -> OutPoint {
        let cursor = self.cursor.slice_by_offset(0, 36).unwrap();
        cursor.into()
    }

    pub fn get_dep_type(&self) -> u8 {
        let cursor = self.cursor.slice_by_offset(36, 1).unwrap();
        cursor.into()
    }
}

pub struct CellOutputVec {
    cursor: CursorType,
}

impl From<CursorType> for CellOutputVec {
    fn from(cursor: CursorType) -> Self {
        Self { cursor }
    }
}

impl CellOutputVec {
    pub fn get(&self, index: usize) -> CellOutput {
        let cursor = self.cursor.dynvec_slice_by_index(index).unwrap();
        cursor.into()
    }
    pub fn len(&self) -> usize {
        self.cursor.dynvec_length()
    }
}

pub struct CellOutput {
    cursor: CursorType,
}

impl From<CursorType> for CellOutput {
    fn from(cursor: CursorType) -> Self {
        Self { cursor }
    }
}

impl CellOutput {
    pub fn get_capacity(&self) -> u64 {
        let cursor = self.cursor.table_slice_by_index(0).unwrap();
        cursor.into()
    }
    pub fn get_lock(&self) -> Script {
        let cursor = self.cursor.table_slice_by_index(1).unwrap();
        cursor.into()
    }
    pub fn get_type_(&self) -> Script {
        let cursor = self.cursor.table_slice_by_index(2).unwrap();
        cursor.into()
    }
}

pub struct BytesVec {
    cursor: CursorType,
}

impl From<CursorType> for BytesVec {
    fn from(cursor: CursorType) -> Self {
        Self { cursor }
    }
}

impl BytesVec {
    pub fn get(&self, index: usize) -> Vec<u8> {
        let cursor = self.cursor.dynvec_slice_by_index(index).unwrap();
        let cur2 = cursor.convert_to_rawbytes().unwrap();
        cur2.into()
    }

    pub fn len(&self) -> usize {
        self.cursor.dynvec_length()
    }
}
pub struct CellInputVec {
    cursor: CursorType,
}

impl From<CursorType> for CellInputVec {
    fn from(cursor: CursorType) -> Self {
        Self { cursor }
    }
}

impl CellInputVec {
    pub fn get(&self, index: usize) -> CellInput {
        let cursor = self.cursor.fixvec_slice_by_index(44, index).unwrap();
        cursor.into()
    }

    pub fn len(&self) -> usize {
        self.cursor.fixvec_length()
    }
}

pub struct CellInput {
    cursor: CursorType,
}

impl From<CursorType> for CellInput {
    fn from(cursor: CursorType) -> Self {
        Self { cursor }
    }
}

impl CellInput {
    pub fn get_since(&self) -> u64 {
        let cursor = self.cursor.slice_by_offset(0, 8).unwrap();
        cursor.into()
    }
    pub fn get_previous_output(&self) -> OutPoint {
        let cursor = self.cursor.slice_by_offset(8, 36).unwrap();
        cursor.into()
    }
}

pub struct OutPoint {
    cursor: CursorType,
}

impl From<CursorType> for OutPoint {
    fn from(cursor: CursorType) -> Self {
        Self { cursor }
    }
}

impl OutPoint {
    pub fn get_tx_hash(&self) -> Vec<u8> {
        let cursor = self.cursor.slice_by_offset(0, 32).unwrap();
        cursor.into()
    }

    pub fn get_index(&self) -> u32 {
        let cursor = self.cursor.slice_by_offset(32, 4).unwrap();
        cursor.into()
    }
}

pub struct WitnessArgs {
    cursor: CursorType,
}

impl From<CursorType> for WitnessArgs {
    fn from(cursor: CursorType) -> Self {
        cursor.into()
    }
}

impl WitnessArgs {
    pub fn get_lock(&self) -> BytesOpt {
        let cursor = self.cursor.table_slice_by_index(0).unwrap();
        cursor.into()
    }
}

pub struct BytesOpt {
    cursor: CursorType,
}

impl From<CursorType> for BytesOpt {
    fn from(cursor: CursorType) -> Self {
        BytesOpt { cursor }
    }
}

impl BytesOpt {
    pub fn is_some(&self) -> bool {
        self.cursor.option_is_none()
    }
    pub fn is_none(&self) -> bool {
        !self.cursor.option_is_none()
    }

    pub fn unwrap(&self) -> Vec<u8> {
        let cursor = self.cursor.convert_to_rawbytes().unwrap();
        cursor.into()
    }
}

pub struct Bytes {
    cursor: CursorType,
}

impl From<CursorType> for Bytes {
    fn from(cursor: CursorType) -> Self {
        Bytes { cursor }
    }
}

pub struct Script {
    cursor: CursorType,
}

impl From<CursorType> for Script {
    fn from(cursor: CursorType) -> Self {
        Script { cursor }
    }
}

impl Script {
    pub fn get_code_hash(&self) -> Vec<u8> {
        let cur = self.cursor.table_slice_by_index(0).unwrap();
        // array
        // convert_to_array can be omitted
        cur.into()
    }
    pub fn get_hash_type(&self) -> u8 {
        let cur = self.cursor.table_slice_by_index(0).unwrap();
        // primitive types
        cur.into()
    }
    pub fn get_args(&self) -> Vec<u8> {
        let cur = self.cursor.table_slice_by_index(0).unwrap();
        // need extra step for fixvec
        let cur2 = cur.convert_to_rawbytes().unwrap();
        cur2.into()
    }
}

#[test]
pub fn test_script() {}
