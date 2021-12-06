#![allow(dead_code)]

mod blockchain;
mod import;
mod sample;
mod types;

extern crate alloc;

use alloc::vec::Vec;
use blockchain::*;
use molecule2::Cursor;
// test molecule "import"
#[allow(unused_imports)]
use import::ScriptVec;
#[allow(unused_imports)]
use import::ScriptVecOpt;

use ckb_types::molecule::bytes;
use ckb_types::packed;
use ckb_types::prelude::{Builder, Entity, Pack};

fn main() {
    println!("Hello, world!");
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
        .args(bytes::Bytes::copy_from_slice(&args).pack())
        .build()
}

impl From<packed::Script> for Script {
    fn from(script: packed::Script) -> Script {
        let memory = Vec::from(script.as_slice());
        let cursor: Cursor = memory.into();
        cursor.into()
    }
}

fn verify_script(script: Script) {
    let code_hash = script.code_hash();
    let hash_type = script.hash_type();
    let args = script.args();

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
        let cursor: Cursor = memory.into();
        cursor.into()
    }
}

fn verify_cell_output(cell_output: CellOutput) {
    let capacity = cell_output.capacity();
    assert_eq!(capacity, CAPACITY);

    let lock_script = cell_output.lock();
    assert_eq!(lock_script.code_hash().as_slice(), &CODE_HASH);
    assert_eq!(lock_script.args().as_slice(), &ARGS);
    assert_eq!(lock_script.hash_type(), HASH_TYPE);

    let type_script = cell_output.type_().unwrap();
    assert_eq!(type_script.code_hash().as_slice(), &CODE_HASH);
    assert_eq!(type_script.args().as_slice(), &ARGS);
    assert_eq!(type_script.hash_type(), HASH_TYPE);
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
        let cursor: Cursor = memory.into();
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
        let cursor: Cursor = memory.into();
        cursor.into()
    }
}

fn verify_cell_dep(cell_dep: CellDep) {
    let dep_type = cell_dep.dep_type();
    let out_point = cell_dep.out_point();
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
        let cursor: Cursor = memory.into();
        cursor.into()
    }
}

fn verify_out_point(out_point: OutPoint) {
    let tx_hash = out_point.tx_hash();
    let index = out_point.index();

    assert_eq!(tx_hash.as_slice(), &TX_HASH);
    assert_eq!(index, INDEX);
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
