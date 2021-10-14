#![allow(dead_code)]
#![allow(unused_imports)]

extern crate alloc;

use alloc::vec::Vec;
use molecule2;
use molecule2::{make_cursor_from_memory, CursorType};

use ckb_types::core;
use ckb_types::molecule::bytes;
use ckb_types::packed;
use ckb_types::prelude::{Builder, Entity, Pack};

// The following is manually written code which can finally be generated by tool.
// It will be a reference to check the generated code is correct or not.
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
        let cursor = self.cursor.table_slice_by_index(0).unwrap();
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
    pub fn get_type_(&self) -> Option<Script> {
        let cursor = self.cursor.table_slice_by_index(2).unwrap();
        if cursor.option_is_none() {
            None
        } else {
            Some(cursor.into())
        }
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
        let cur = self.cursor.table_slice_by_index(1).unwrap();
        // primitive types
        cur.into()
    }
    pub fn get_args(&self) -> Vec<u8> {
        let cur = self.cursor.table_slice_by_index(2).unwrap();
        // need extra step for fixvec
        let cur2 = cur.convert_to_rawbytes().unwrap();
        cur2.into()
    }
}

// testing part
// all fields use fixed values to simplify testing
const CODE_HASH: [u8; 32] = [1u8; 32];
const HASH_TYPE: u8 = 1;
const ARGS: [u8; 5000] = [1; 5000];
const CAPACITY: u64 = 1;
const TX_HASH: [u8; 32] = [1u8; 32];
const INDEX: u32 = 1;
const DEP_TYPE: u8 = 1;

fn create_script() -> packed::Script {
    let code_hash = CODE_HASH.clone();
    let hash_type = HASH_TYPE;
    let args = ARGS.clone();

    let builder = packed::Script::new_builder();
    builder
        .code_hash(code_hash.clone().pack())
        .hash_type(hash_type.into())
        .args(bytes::Bytes::copy_from_slice(&args[..]).pack())
        .build()
}

impl From<packed::Script> for Script {
    fn from(script: packed::Script) -> Script {
        let memory = Vec::from(script.as_slice());
        let cursor = make_cursor_from_memory(memory);
        cursor.into()
    }
}

fn verify_script(script: Script) {
    let code_hash = script.get_code_hash();
    let hash_type = script.get_hash_type();
    let args = script.get_args();

    assert_eq!(HASH_TYPE, hash_type);
    assert_eq!(&CODE_HASH, &code_hash[..]);
    assert_eq!(&ARGS, &args[..])
}

fn create_cell_output() -> packed::CellOutput {
    let capacity = 1u64;
    let builder = packed::CellOutput::new_builder();
    builder
        .capacity(capacity.pack())
        .lock(create_script())
        .type_(Some(create_script()).pack())
        .build()
}

impl From<packed::CellOutput> for CellOutput {
    fn from(cell_output: packed::CellOutput) -> CellOutput {
        let memory = Vec::from(cell_output.as_slice());
        let cursor = make_cursor_from_memory(memory);
        cursor.into()
    }
}

fn verify_cell_output(cell_output: CellOutput) {
    let capacity = cell_output.get_capacity();
    assert_eq!(capacity, CAPACITY);

    let lock_script = cell_output.get_lock();
    assert_eq!(lock_script.get_code_hash().as_slice(), &CODE_HASH);
    assert_eq!(lock_script.get_args().as_slice(), &ARGS);
    assert_eq!(lock_script.get_hash_type(), HASH_TYPE);

    let type_script = cell_output.get_type_().unwrap();
    assert_eq!(type_script.get_code_hash().as_slice(), &CODE_HASH);
    assert_eq!(type_script.get_args().as_slice(), &ARGS);
    assert_eq!(type_script.get_hash_type(), HASH_TYPE);
}

fn create_cell_output_vec() -> packed::CellOutputVec {
    let builder = packed::CellOutputVec::new_builder();
    builder
        .push(create_cell_output())
        .push(create_cell_output())
        .build()
}

impl From<packed::CellOutputVec> for CellOutputVec {
    fn from(cell_output_vec: packed::CellOutputVec) -> Self {
        let memory = Vec::from(cell_output_vec.as_slice());
        let cursor = make_cursor_from_memory(memory);
        cursor.into()
    }
}

fn verify_cell_output_vec(cell_output_vec: CellOutputVec) {
    let len = cell_output_vec.len();
    for i in 0..len {
        let cell_output = cell_output_vec.get(i);
        verify_cell_output(cell_output);
    }
}

fn create_cell_dep() -> packed::CellDep {
    let builder = packed::CellDep::new_builder();
    builder
        .out_point(create_out_point())
        .dep_type(DEP_TYPE.into())
        .build()
}

impl From<packed::CellDep> for CellDep {
    fn from(cell_dep: packed::CellDep) -> Self {
        let memory = Vec::from(cell_dep.as_slice());
        let cursor = make_cursor_from_memory(memory);
        cursor.into()
    }
}

fn verify_cell_dep(cell_dep: CellDep) {
    let dep_type = cell_dep.get_dep_type();
    let out_point = cell_dep.get_out_point();
    assert_eq!(dep_type, DEP_TYPE);
    verify_out_point(out_point);
}

fn create_out_point() -> packed::OutPoint {
    let builder = packed::OutPoint::new_builder();
    builder.tx_hash(TX_HASH.pack()).index(INDEX.pack()).build()
}

impl From<packed::OutPoint> for OutPoint {
    fn from(out_point: packed::OutPoint) -> Self {
        let memory = Vec::from(out_point.as_slice());
        let cursor = make_cursor_from_memory(memory);
        cursor.into()
    }
}

fn verify_out_point(out_point: OutPoint) {
    let tx_hash = out_point.get_tx_hash();
    let index = out_point.get_index();

    assert_eq!(tx_hash.as_slice(), &TX_HASH);
    assert_eq!(index, INDEX);
}

// here is an example of union, it can either be script or u64
pub struct SampleUnion {
    cursor: CursorType,
}

impl From<CursorType> for SampleUnion {
    fn from(cursor: CursorType) -> Self {
        Self { cursor }
    }
}

impl SampleUnion {
    pub fn get_item_id(&self) -> usize {
        let union = self.cursor.union_unpack();
        union.item_id
    }

    pub fn as_script(&self) -> Script {
        let union = self.cursor.union_unpack();
        union.cursor.clone().into()
    }
    pub fn as_capacity(&self) -> u64 {
        let union = self.cursor.union_unpack();
        union.cursor.clone().into()
    }
}

#[test]
pub fn test_script() {
    let script = create_script();
    let new_script = script.into();
    verify_script(new_script);
}

#[test]
pub fn test_cell_output() {
    let cell_output = create_cell_output();
    let new_cell_output = cell_output.into();
    verify_cell_output(new_cell_output);
}

#[test]
pub fn test_cell_output_vec() {
    let cell_output_vec = create_cell_output_vec();
    let new_cell_output_vec = cell_output_vec.into();
    verify_cell_output_vec(new_cell_output_vec);
}

#[test]
pub fn test_cell_dep() {
    let cell_dep = create_cell_dep();
    let new_cell_dep = cell_dep.into();
    verify_cell_dep(new_cell_dep);
}
