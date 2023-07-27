pub mod types_all_data;
pub mod types_api2_mol2;
pub mod types_array;
pub mod types_option;
pub mod types_struct;
pub mod types_table;
pub mod types_vec;

use crate::{types_api, types_api2};
use molecule::prelude::{Builder, Entity};
use rand::{rngs::ThreadRng, thread_rng, Rng};
use std::fmt::Debug;

pub use types_api2_mol2::*;
pub use types_array::*;
pub use types_option::*;
pub use types_struct::*;
pub use types_table::*;
pub use types_vec::*;

pub trait BaseTypes {
    fn new_rng(rng: &mut ThreadRng) -> Self;
}

impl BaseTypes for u8 {
    fn new_rng(rng: &mut ThreadRng) -> Self {
        rng.gen()
    }
}

#[derive(Debug)]
pub enum TypesCheckErr {
    Lenght(String),
    Data(String),
    Opt(String),
}
pub type ResCheckErr = Result<(), TypesCheckErr>;

impl TypesCheckErr {
    pub fn check_lenght(l1: usize, l2: usize) -> Result<(), Self> {
        if l1 == l2 {
            Ok(())
        } else {
            Err(Self::Lenght(format!("different length : {} {}", l1, l2)))
        }
    }

    pub fn check_1_data<T: Eq + Debug>(d1: &T, d2: &T) -> Result<(), Self> {
        if d1 == d2 {
            Ok(())
        } else {
            Err(Self::Data(format!(
                "different data: {:02x?}  {:02x?}",
                d1, d2
            )))
        }
    }

    pub fn check_data<T1: Eq + Debug, T: Mol2Vec<T1>>(d1: &T, d2: &[T1]) -> ResCheckErr {
        TypesCheckErr::check_lenght(d1.len(), d2.len())?;

        for i in 0..d1.len() {
            TypesCheckErr::check_1_data(&d1.get(i), &d2[i])?;
        }

        Ok(())
    }

    pub fn check_option<T1, T2>(d1: &Option<T1>, d2: &Option<T2>) -> ResCheckErr {
        if d1.is_some() != d2.is_some() {
            return Err(TypesCheckErr::Opt(format!(
                "different option: {:?}  {:?}",
                d1.is_some(),
                d2.is_some()
            )));
        }
        Ok(())
    }
}

pub enum TypesUnionA {
    Byte(TypesArray<u8, 1>),
    Word(TypesArrayWord),
    StructA(TypesStructA),
    Bytes(TypesVec<u8>),
    Words(TypesVec<TypesArrayWord>),
    Table0(TypesTable0),
    Table6(TypesTable6),
    Table6Opt(TypesOption<TypesTable6>),
}
impl BaseTypes for TypesUnionA {
    fn new_rng(rng: &mut ThreadRng) -> Self {
        let v = rng.gen_range(0..8);
        match v {
            0 => Self::Byte(TypesArray::new_rng(rng)),
            1 => Self::Word(TypesArrayWord::new_rng(rng)),
            2 => Self::StructA(TypesStructA::new_rng(rng)),
            3 => Self::Bytes(TypesVec::new_rng(rng)),
            4 => Self::Words(TypesVec::new_rng(rng)),
            5 => Self::Table0(TypesTable0::new_rng(rng)),
            6 => Self::Table6(TypesTable6::new_rng(rng)),
            7 => Self::Table6Opt(TypesOption::new_rng(rng)),

            _ => panic!("unknow error"),
        }
    }
}
impl Default for TypesUnionA {
    fn default() -> Self {
        Self::new_rng(&mut thread_rng())
    }
}
impl TypesUnionA {
    pub fn to_mol(&self) -> types_api::UnionA {
        let t = match self {
            Self::Byte(v) => types_api::UnionAUnion::Byte(v.to_mol()),
            Self::Word(v) => types_api::UnionAUnion::Word(v.to_mol2()),
            Self::StructA(v) => types_api::UnionAUnion::StructA(v.to_mol()),
            Self::Bytes(v) => types_api::UnionAUnion::Bytes(v.to_mol()),
            Self::Words(v) => types_api::UnionAUnion::Words(v.to_mol()),
            Self::Table0(v) => types_api::UnionAUnion::Table0(v.to_mol()),
            Self::Table6(v) => types_api::UnionAUnion::Table6(v.to_mol()),
            Self::Table6Opt(v) => types_api::UnionAUnion::Table6Opt(v.to_mol()),
        };
        types_api::UnionA::new_builder().set(t).build()
    }

    pub fn check(&self, d: &types_api2::UnionA) -> ResCheckErr {
        // let item_id = d.item_id();

        match self {
            Self::Byte(v) => v.check(&d.as_byte()),
            Self::Word(v) => v.check2(&d.as_word().into()),
            Self::StructA(v) => v.check(&d.as_structa()),
            Self::Bytes(v) => v.check(&d.as_bytes().into()),
            Self::Words(v) => v.check(&d.as_words().into()),
            Self::Table0(v) => v.check(&d.as_table0()),
            Self::Table6(v) => v.check(&d.as_table6()),
            Self::Table6Opt(v) => v.check(&d.as_table6opt()),
        }
    }
}
