use crate::types_api;
use crate::types_api2::*;

pub trait Mol2Vec<T> {
    fn mol_len(&self) -> usize;
    fn mol_get(&self, index: usize) -> T;
}
impl Mol2Vec<u8> for Vec<u8> {
    fn mol_len(&self) -> usize {
        self.len()
    }
    fn mol_get(&self, index: usize) -> u8 {
        self[index]
    }
}
impl Mol2Vec<u8> for Byte2 {
    fn mol_len(&self) -> usize {
        self.len()
    }
    fn mol_get(&self, index: usize) -> u8 {
        self.get(index)
    }
}
impl Mol2Vec<u8> for Byte3 {
    fn mol_len(&self) -> usize {
        self.len()
    }
    fn mol_get(&self, index: usize) -> u8 {
        self.get(index)
    }
}
impl Mol2Vec<u8> for Byte4 {
    fn mol_len(&self) -> usize {
        self.len()
    }
    fn mol_get(&self, index: usize) -> u8 {
        self.get(index)
    }
}
impl Mol2Vec<u8> for Byte5 {
    fn mol_len(&self) -> usize {
        self.len()
    }
    fn mol_get(&self, index: usize) -> u8 {
        self.get(index)
    }
}
impl Mol2Vec<u8> for Byte6 {
    fn mol_len(&self) -> usize {
        self.len()
    }
    fn mol_get(&self, index: usize) -> u8 {
        self.get(index)
    }
}
impl Mol2Vec<u8> for Byte7 {
    fn mol_len(&self) -> usize {
        self.len()
    }
    fn mol_get(&self, index: usize) -> u8 {
        self.get(index)
    }
}
impl Mol2Vec<u8> for Byte8 {
    fn mol_len(&self) -> usize {
        self.len()
    }
    fn mol_get(&self, index: usize) -> u8 {
        self.get(index)
    }
}
impl Mol2Vec<u8> for Byte9 {
    fn mol_len(&self) -> usize {
        self.len()
    }
    fn mol_get(&self, index: usize) -> u8 {
        self.get(index)
    }
}
impl Mol2Vec<u8> for Byte10 {
    fn mol_len(&self) -> usize {
        self.len()
    }
    fn mol_get(&self, index: usize) -> u8 {
        self.get(index)
    }
}
impl Mol2Vec<u8> for Byte11 {
    fn mol_len(&self) -> usize {
        self.len()
    }
    fn mol_get(&self, index: usize) -> u8 {
        self.get(index)
    }
}
impl Mol2Vec<u8> for Byte12 {
    fn mol_len(&self) -> usize {
        self.len()
    }
    fn mol_get(&self, index: usize) -> u8 {
        self.get(index)
    }
}
impl Mol2Vec<u8> for Byte13 {
    fn mol_len(&self) -> usize {
        self.len()
    }
    fn mol_get(&self, index: usize) -> u8 {
        self.get(index)
    }
}
impl Mol2Vec<u8> for Byte14 {
    fn mol_len(&self) -> usize {
        self.len()
    }
    fn mol_get(&self, index: usize) -> u8 {
        self.get(index)
    }
}
impl Mol2Vec<u8> for Byte15 {
    fn mol_len(&self) -> usize {
        self.len()
    }
    fn mol_get(&self, index: usize) -> u8 {
        self.get(index)
    }
}
impl Mol2Vec<u8> for Byte16 {
    fn mol_len(&self) -> usize {
        self.len()
    }
    fn mol_get(&self, index: usize) -> u8 {
        self.get(index)
    }
}
impl Mol2Vec<u8> for Word {
    fn mol_len(&self) -> usize {
        self.len()
    }
    fn mol_get(&self, index: usize) -> u8 {
        self.get(index)
    }
}
impl Mol2Vec<StructI> for StructIx3 {
    fn mol_len(&self) -> usize {
        self.len()
    }
    fn mol_get(&self, index: usize) -> StructI {
        self.get(index)
    }
}
impl Mol2Vec<u8> for Bytes {
    fn mol_len(&self) -> usize {
        self.len()
    }
    fn mol_get(&self, index: usize) -> u8 {
        self.get(index)
    }
}
impl Mol2Vec<StructI> for StructIVec {
    fn mol_len(&self) -> usize {
        self.len()
    }
    fn mol_get(&self, index: usize) -> StructI {
        self.get(index)
    }
}
impl Mol2Vec<StructJ> for StructJVec {
    fn mol_len(&self) -> usize {
        self.len()
    }
    fn mol_get(&self, index: usize) -> StructJ {
        self.get(index)
    }
}
impl Mol2Vec<StructP> for StructPVec {
    fn mol_len(&self) -> usize {
        self.len()
    }
    fn mol_get(&self, index: usize) -> StructP {
        self.get(index)
    }
}
impl Mol2Vec<Word> for Word2 {
    fn mol_len(&self) -> usize {
        self.len()
    }
    fn mol_get(&self, index: usize) -> Word {
        Word::from(self.get(index))
    }
}
impl Mol2Vec<Word> for Word3 {
    fn mol_len(&self) -> usize {
        self.len()
    }
    fn mol_get(&self, index: usize) -> Word {
        Word::from(self.get(index))
    }
}
impl Mol2Vec<Word> for Word4 {
    fn mol_len(&self) -> usize {
        self.len()
    }
    fn mol_get(&self, index: usize) -> Word {
        Word::from(self.get(index))
    }
}
impl Mol2Vec<Word> for Word5 {
    fn mol_len(&self) -> usize {
        self.len()
    }
    fn mol_get(&self, index: usize) -> Word {
        Word::from(self.get(index))
    }
}
impl Mol2Vec<Word> for Word6 {
    fn mol_len(&self) -> usize {
        self.len()
    }
    fn mol_get(&self, index: usize) -> Word {
        Word::from(self.get(index))
    }
}
impl Mol2Vec<Word> for Word7 {
    fn mol_len(&self) -> usize {
        self.len()
    }
    fn mol_get(&self, index: usize) -> Word {
        Word::from(self.get(index))
    }
}
impl Mol2Vec<Word> for Word8 {
    fn mol_len(&self) -> usize {
        self.len()
    }
    fn mol_get(&self, index: usize) -> Word {
        Word::from(self.get(index))
    }
}
impl Mol2Vec<Byte3> for Byte3x3 {
    fn mol_len(&self) -> usize {
        self.len()
    }
    fn mol_get(&self, index: usize) -> Byte3 {
        Byte3::from(self.get(index))
    }
}
impl Mol2Vec<Byte5> for Byte5x3 {
    fn mol_len(&self) -> usize {
        self.len()
    }
    fn mol_get(&self, index: usize) -> Byte5 {
        Byte5::from(self.get(index))
    }
}
impl Mol2Vec<Byte7> for Byte7x3 {
    fn mol_len(&self) -> usize {
        self.len()
    }
    fn mol_get(&self, index: usize) -> Byte7 {
        Byte7::from(self.get(index))
    }
}
impl Mol2Vec<Byte9> for Byte9x3 {
    fn mol_len(&self) -> usize {
        self.len()
    }
    fn mol_get(&self, index: usize) -> Byte9 {
        Byte9::from(self.get(index))
    }
}
impl Mol2Vec<Word> for Words {
    fn mol_len(&self) -> usize {
        self.len()
    }
    fn mol_get(&self, index: usize) -> Word {
        Word::from(self.get(index))
    }
}
impl Mol2Vec<Byte3> for Byte3Vec {
    fn mol_len(&self) -> usize {
        self.len()
    }
    fn mol_get(&self, index: usize) -> Byte3 {
        Byte3::from(self.get(index))
    }
}
impl Mol2Vec<Byte7> for Byte7Vec {
    fn mol_len(&self) -> usize {
        self.len()
    }
    fn mol_get(&self, index: usize) -> Byte7 {
        Byte7::from(self.get(index))
    }
}
impl Mol2Vec<Vec<u8>> for BytesVec {
    fn mol_len(&self) -> usize {
        self.len()
    }
    fn mol_get(&self, index: usize) -> Vec<u8> {
        self.get(index).try_into().unwrap()
    }
}
impl Mol2Vec<Words> for WordsVec {
    fn mol_len(&self) -> usize {
        self.len()
    }
    fn mol_get(&self, index: usize) -> Words {
        Words::from(self.get(index))
    }
}

impl Mol2Vec<u8> for types_api::Byte2 {
    fn mol_len(&self) -> usize {
        self.raw_data().len()
    }
    fn mol_get(&self, index: usize) -> u8 {
        self.raw_data().get(index).unwrap().clone()
    }
}
impl Mol2Vec<u8> for types_api::Byte3 {
    fn mol_len(&self) -> usize {
        self.raw_data().len()
    }
    fn mol_get(&self, index: usize) -> u8 {
        self.raw_data().get(index).unwrap().clone()
    }
}
impl Mol2Vec<u8> for types_api::Byte4 {
    fn mol_len(&self) -> usize {
        self.raw_data().len()
    }
    fn mol_get(&self, index: usize) -> u8 {
        self.raw_data().get(index).unwrap().clone()
    }
}
impl Mol2Vec<u8> for types_api::Byte5 {
    fn mol_len(&self) -> usize {
        self.raw_data().len()
    }
    fn mol_get(&self, index: usize) -> u8 {
        self.raw_data().get(index).unwrap().clone()
    }
}
impl Mol2Vec<u8> for types_api::Byte6 {
    fn mol_len(&self) -> usize {
        self.raw_data().len()
    }
    fn mol_get(&self, index: usize) -> u8 {
        self.raw_data().get(index).unwrap().clone()
    }
}
impl Mol2Vec<u8> for types_api::Byte7 {
    fn mol_len(&self) -> usize {
        self.raw_data().len()
    }
    fn mol_get(&self, index: usize) -> u8 {
        self.raw_data().get(index).unwrap().clone()
    }
}
impl Mol2Vec<u8> for types_api::Byte8 {
    fn mol_len(&self) -> usize {
        self.raw_data().len()
    }
    fn mol_get(&self, index: usize) -> u8 {
        self.raw_data().get(index).unwrap().clone()
    }
}
impl Mol2Vec<u8> for types_api::Byte9 {
    fn mol_len(&self) -> usize {
        self.raw_data().len()
    }
    fn mol_get(&self, index: usize) -> u8 {
        self.raw_data().get(index).unwrap().clone()
    }
}
impl Mol2Vec<u8> for types_api::Byte10 {
    fn mol_len(&self) -> usize {
        self.raw_data().len()
    }
    fn mol_get(&self, index: usize) -> u8 {
        self.raw_data().get(index).unwrap().clone()
    }
}
impl Mol2Vec<u8> for types_api::Byte11 {
    fn mol_len(&self) -> usize {
        self.raw_data().len()
    }
    fn mol_get(&self, index: usize) -> u8 {
        self.raw_data().get(index).unwrap().clone()
    }
}
impl Mol2Vec<u8> for types_api::Byte12 {
    fn mol_len(&self) -> usize {
        self.raw_data().len()
    }
    fn mol_get(&self, index: usize) -> u8 {
        self.raw_data().get(index).unwrap().clone()
    }
}
impl Mol2Vec<u8> for types_api::Byte13 {
    fn mol_len(&self) -> usize {
        self.raw_data().len()
    }
    fn mol_get(&self, index: usize) -> u8 {
        self.raw_data().get(index).unwrap().clone()
    }
}
impl Mol2Vec<u8> for types_api::Byte14 {
    fn mol_len(&self) -> usize {
        self.raw_data().len()
    }
    fn mol_get(&self, index: usize) -> u8 {
        self.raw_data().get(index).unwrap().clone()
    }
}
impl Mol2Vec<u8> for types_api::Byte15 {
    fn mol_len(&self) -> usize {
        self.raw_data().len()
    }
    fn mol_get(&self, index: usize) -> u8 {
        self.raw_data().get(index).unwrap().clone()
    }
}
impl Mol2Vec<u8> for types_api::Byte16 {
    fn mol_len(&self) -> usize {
        self.raw_data().len()
    }
    fn mol_get(&self, index: usize) -> u8 {
        self.raw_data().get(index).unwrap().clone()
    }
}
impl Mol2Vec<u8> for types_api::Word {
    fn mol_len(&self) -> usize {
        self.raw_data().len()
    }
    fn mol_get(&self, index: usize) -> u8 {
        self.raw_data().get(index).unwrap().clone()
    }
}
impl Mol2Vec<types_api::Word> for types_api::Word2 {
    fn mol_len(&self) -> usize {
        2
    }
    fn mol_get(&self, index: usize) -> types_api::Word {
        match index {
            0 => self.nth0(),
            1 => self.nth1(),
            _ => panic!("out of bound"),
        }
    }
}
impl Mol2Vec<types_api::Word> for types_api::Word3 {
    fn mol_len(&self) -> usize {
        3
    }
    fn mol_get(&self, index: usize) -> types_api::Word {
        match index {
            0 => self.nth0(),
            1 => self.nth1(),
            2 => self.nth2(),
            _ => panic!("out of bound"),
        }
    }
}
impl Mol2Vec<types_api::Word> for types_api::Word4 {
    fn mol_len(&self) -> usize {
        4
    }
    fn mol_get(&self, index: usize) -> types_api::Word {
        match index {
            0 => self.nth0(),
            1 => self.nth1(),
            2 => self.nth2(),
            3 => self.nth3(),
            _ => panic!("out of bound"),
        }
    }
}
impl Mol2Vec<types_api::Word> for types_api::Word5 {
    fn mol_len(&self) -> usize {
        5
    }
    fn mol_get(&self, index: usize) -> types_api::Word {
        match index {
            0 => self.nth0(),
            1 => self.nth1(),
            2 => self.nth2(),
            3 => self.nth3(),
            4 => self.nth4(),
            _ => panic!("out of bound"),
        }
    }
}
impl Mol2Vec<types_api::Word> for types_api::Word6 {
    fn mol_len(&self) -> usize {
        6
    }
    fn mol_get(&self, index: usize) -> types_api::Word {
        match index {
            0 => self.nth0(),
            1 => self.nth1(),
            2 => self.nth2(),
            3 => self.nth3(),
            4 => self.nth4(),
            5 => self.nth5(),
            _ => panic!("out of bound"),
        }
    }
}
impl Mol2Vec<types_api::Word> for types_api::Word7 {
    fn mol_len(&self) -> usize {
        7
    }
    fn mol_get(&self, index: usize) -> types_api::Word {
        match index {
            0 => self.nth0(),
            1 => self.nth1(),
            2 => self.nth2(),
            3 => self.nth3(),
            4 => self.nth4(),
            5 => self.nth5(),
            6 => self.nth6(),
            _ => panic!("out of bound"),
        }
    }
}
impl Mol2Vec<types_api::Word> for types_api::Word8 {
    fn mol_len(&self) -> usize {
        8
    }
    fn mol_get(&self, index: usize) -> types_api::Word {
        match index {
            0 => self.nth0(),
            1 => self.nth1(),
            2 => self.nth2(),
            3 => self.nth3(),
            4 => self.nth4(),
            5 => self.nth5(),
            6 => self.nth6(),
            7 => self.nth7(),
            _ => panic!("out of bound"),
        }
    }
}
impl Mol2Vec<types_api::Byte3> for types_api::Byte3x3 {
    fn mol_len(&self) -> usize {
        3
    }
    fn mol_get(&self, index: usize) -> types_api::Byte3 {
        match index {
            0 => self.nth0(),
            1 => self.nth1(),
            2 => self.nth2(),
            _ => panic!("out of bound"),
        }
    }
}
impl Mol2Vec<types_api::Byte5> for types_api::Byte5x3 {
    fn mol_len(&self) -> usize {
        3
    }
    fn mol_get(&self, index: usize) -> types_api::Byte5 {
        match index {
            0 => self.nth0(),
            1 => self.nth1(),
            2 => self.nth2(),
            _ => panic!("out of bound"),
        }
    }
}
impl Mol2Vec<types_api::Byte7> for types_api::Byte7x3 {
    fn mol_len(&self) -> usize {
        3
    }
    fn mol_get(&self, index: usize) -> types_api::Byte7 {
        match index {
            0 => self.nth0(),
            1 => self.nth1(),
            2 => self.nth2(),
            _ => panic!("out of bound"),
        }
    }
}
impl Mol2Vec<types_api::Byte9> for types_api::Byte9x3 {
    fn mol_len(&self) -> usize {
        3
    }
    fn mol_get(&self, index: usize) -> types_api::Byte9 {
        match index {
            0 => self.nth0(),
            1 => self.nth1(),
            2 => self.nth2(),
            _ => panic!("out of bound"),
        }
    }
}
impl Mol2Vec<types_api::StructI> for types_api::StructIx3 {
    fn mol_len(&self) -> usize {
        3
    }
    fn mol_get(&self, index: usize) -> types_api::StructI {
        match index {
            0 => self.nth0(),
            1 => self.nth1(),
            2 => self.nth2(),
            _ => panic!("out of bound"),
        }
    }
}
impl Mol2Vec<u8> for types_api::Bytes {
    fn mol_len(&self) -> usize {
        self.len()
    }
    fn mol_get(&self, index: usize) -> u8 {
        self.get(index).unwrap().into()
    }
}
impl Mol2Vec<types_api::Word> for types_api::Words {
    fn mol_len(&self) -> usize {
        self.len()
    }
    fn mol_get(&self, index: usize) -> types_api::Word {
        self.get(index).unwrap()
    }
}
impl Mol2Vec<types_api::Byte3> for types_api::Byte3Vec {
    fn mol_len(&self) -> usize {
        self.len()
    }
    fn mol_get(&self, index: usize) -> types_api::Byte3 {
        self.get(index).unwrap()
    }
}
impl Mol2Vec<types_api::Byte7> for types_api::Byte7Vec {
    fn mol_len(&self) -> usize {
        self.len()
    }
    fn mol_get(&self, index: usize) -> types_api::Byte7 {
        self.get(index).unwrap()
    }
}
impl Mol2Vec<types_api::StructI> for types_api::StructIVec {
    fn mol_len(&self) -> usize {
        self.len()
    }
    fn mol_get(&self, index: usize) -> types_api::StructI {
        self.get(index).unwrap()
    }
}
impl Mol2Vec<types_api::StructJ> for types_api::StructJVec {
    fn mol_len(&self) -> usize {
        self.len()
    }
    fn mol_get(&self, index: usize) -> types_api::StructJ {
        self.get(index).unwrap()
    }
}
impl Mol2Vec<types_api::StructP> for types_api::StructPVec {
    fn mol_len(&self) -> usize {
        self.len()
    }
    fn mol_get(&self, index: usize) -> types_api::StructP {
        self.get(index).unwrap()
    }
}
impl Mol2Vec<types_api::Bytes> for types_api::BytesVec {
    fn mol_len(&self) -> usize {
        self.len()
    }
    fn mol_get(&self, index: usize) -> types_api::Bytes {
        self.get(index).unwrap()
    }
}
impl Mol2Vec<types_api::Words> for types_api::WordsVec {
    fn mol_len(&self) -> usize {
        self.len()
    }
    fn mol_get(&self, index: usize) -> types_api::Words {
        self.get(index).unwrap()
    }
}
impl Mol2Vec<types_api::ByteOpt> for types_api::ByteOptVec {
    fn mol_len(&self) -> usize {
        self.len()
    }
    fn mol_get(&self, index: usize) -> types_api::ByteOpt {
        self.get(index).unwrap()
    }
}
impl Mol2Vec<types_api::WordOpt> for types_api::WordOptVec {
    fn mol_len(&self) -> usize {
        self.len()
    }
    fn mol_get(&self, index: usize) -> types_api::WordOpt {
        self.get(index).unwrap()
    }
}
impl Mol2Vec<types_api::WordsOpt> for types_api::WordsOptVec {
    fn mol_len(&self) -> usize {
        self.len()
    }
    fn mol_get(&self, index: usize) -> types_api::WordsOpt {
        self.get(index).unwrap()
    }
}
impl Mol2Vec<types_api::BytesOpt> for types_api::BytesOptVec {
    fn mol_len(&self) -> usize {
        self.len()
    }
    fn mol_get(&self, index: usize) -> types_api::BytesOpt {
        self.get(index).unwrap()
    }
}
