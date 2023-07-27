use crate::types_api2::*;

pub trait Mol2Vec<T> {
    fn len(&self) -> usize;
    fn get(&self, index: usize) -> T;
}
impl Mol2Vec<u8> for Vec<u8> {
    fn len(&self) -> usize {
        self.len()
    }
    fn get(&self, index: usize) -> u8 {
        self[index]
    }
}
impl Mol2Vec<u8> for Byte2 {
    fn len(&self) -> usize {
        self.len()
    }
    fn get(&self, index: usize) -> u8 {
        self.get(index)
    }
}
impl Mol2Vec<u8> for Byte3 {
    fn len(&self) -> usize {
        self.len()
    }
    fn get(&self, index: usize) -> u8 {
        self.get(index)
    }
}
impl Mol2Vec<u8> for Byte4 {
    fn len(&self) -> usize {
        self.len()
    }
    fn get(&self, index: usize) -> u8 {
        self.get(index)
    }
}
impl Mol2Vec<u8> for Byte5 {
    fn len(&self) -> usize {
        self.len()
    }
    fn get(&self, index: usize) -> u8 {
        self.get(index)
    }
}
impl Mol2Vec<u8> for Byte6 {
    fn len(&self) -> usize {
        self.len()
    }
    fn get(&self, index: usize) -> u8 {
        self.get(index)
    }
}
impl Mol2Vec<u8> for Byte7 {
    fn len(&self) -> usize {
        self.len()
    }
    fn get(&self, index: usize) -> u8 {
        self.get(index)
    }
}
impl Mol2Vec<u8> for Byte8 {
    fn len(&self) -> usize {
        self.len()
    }
    fn get(&self, index: usize) -> u8 {
        self.get(index)
    }
}
impl Mol2Vec<u8> for Byte9 {
    fn len(&self) -> usize {
        self.len()
    }
    fn get(&self, index: usize) -> u8 {
        self.get(index)
    }
}
impl Mol2Vec<u8> for Byte10 {
    fn len(&self) -> usize {
        self.len()
    }
    fn get(&self, index: usize) -> u8 {
        self.get(index)
    }
}
impl Mol2Vec<u8> for Byte11 {
    fn len(&self) -> usize {
        self.len()
    }
    fn get(&self, index: usize) -> u8 {
        self.get(index)
    }
}
impl Mol2Vec<u8> for Byte12 {
    fn len(&self) -> usize {
        self.len()
    }
    fn get(&self, index: usize) -> u8 {
        self.get(index)
    }
}
impl Mol2Vec<u8> for Byte13 {
    fn len(&self) -> usize {
        self.len()
    }
    fn get(&self, index: usize) -> u8 {
        self.get(index)
    }
}
impl Mol2Vec<u8> for Byte14 {
    fn len(&self) -> usize {
        self.len()
    }
    fn get(&self, index: usize) -> u8 {
        self.get(index)
    }
}
impl Mol2Vec<u8> for Byte15 {
    fn len(&self) -> usize {
        self.len()
    }
    fn get(&self, index: usize) -> u8 {
        self.get(index)
    }
}
impl Mol2Vec<u8> for Byte16 {
    fn len(&self) -> usize {
        self.len()
    }
    fn get(&self, index: usize) -> u8 {
        self.get(index)
    }
}
impl Mol2Vec<u8> for Word {
    fn len(&self) -> usize {
        self.len()
    }
    fn get(&self, index: usize) -> u8 {
        self.get(index)
    }
}
impl Mol2Vec<StructI> for StructIx3 {
    fn len(&self) -> usize {
        self.len()
    }
    fn get(&self, index: usize) -> StructI {
        self.get(index)
    }
}
impl Mol2Vec<u8> for Bytes {
    fn len(&self) -> usize {
        self.len()
    }
    fn get(&self, index: usize) -> u8 {
        self.get(index)
    }
}
impl Mol2Vec<StructI> for StructIVec {
    fn len(&self) -> usize {
        self.len()
    }
    fn get(&self, index: usize) -> StructI {
        self.get(index)
    }
}
impl Mol2Vec<StructJ> for StructJVec {
    fn len(&self) -> usize {
        self.len()
    }
    fn get(&self, index: usize) -> StructJ {
        self.get(index)
    }
}
impl Mol2Vec<StructP> for StructPVec {
    fn len(&self) -> usize {
        self.len()
    }
    fn get(&self, index: usize) -> StructP {
        self.get(index)
    }
}
impl Mol2Vec<Word> for Word2 {
    fn len(&self) -> usize {
        self.len()
    }
    fn get(&self, index: usize) -> Word {
        Word::from(self.get(index))
    }
}
impl Mol2Vec<Word> for Word3 {
    fn len(&self) -> usize {
        self.len()
    }
    fn get(&self, index: usize) -> Word {
        Word::from(self.get(index))
    }
}
impl Mol2Vec<Word> for Word4 {
    fn len(&self) -> usize {
        self.len()
    }
    fn get(&self, index: usize) -> Word {
        Word::from(self.get(index))
    }
}
impl Mol2Vec<Word> for Word5 {
    fn len(&self) -> usize {
        self.len()
    }
    fn get(&self, index: usize) -> Word {
        Word::from(self.get(index))
    }
}
impl Mol2Vec<Word> for Word6 {
    fn len(&self) -> usize {
        self.len()
    }
    fn get(&self, index: usize) -> Word {
        Word::from(self.get(index))
    }
}
impl Mol2Vec<Word> for Word7 {
    fn len(&self) -> usize {
        self.len()
    }
    fn get(&self, index: usize) -> Word {
        Word::from(self.get(index))
    }
}
impl Mol2Vec<Word> for Word8 {
    fn len(&self) -> usize {
        self.len()
    }
    fn get(&self, index: usize) -> Word {
        Word::from(self.get(index))
    }
}
impl Mol2Vec<Byte3> for Byte3x3 {
    fn len(&self) -> usize {
        self.len()
    }
    fn get(&self, index: usize) -> Byte3 {
        Byte3::from(self.get(index))
    }
}
impl Mol2Vec<Byte5> for Byte5x3 {
    fn len(&self) -> usize {
        self.len()
    }
    fn get(&self, index: usize) -> Byte5 {
        Byte5::from(self.get(index))
    }
}
impl Mol2Vec<Byte7> for Byte7x3 {
    fn len(&self) -> usize {
        self.len()
    }
    fn get(&self, index: usize) -> Byte7 {
        Byte7::from(self.get(index))
    }
}
impl Mol2Vec<Byte9> for Byte9x3 {
    fn len(&self) -> usize {
        self.len()
    }
    fn get(&self, index: usize) -> Byte9 {
        Byte9::from(self.get(index))
    }
}
impl Mol2Vec<Word> for Words {
    fn len(&self) -> usize {
        self.len()
    }
    fn get(&self, index: usize) -> Word {
        Word::from(self.get(index))
    }
}
impl Mol2Vec<Byte3> for Byte3Vec {
    fn len(&self) -> usize {
        self.len()
    }
    fn get(&self, index: usize) -> Byte3 {
        Byte3::from(self.get(index))
    }
}
impl Mol2Vec<Byte7> for Byte7Vec {
    fn len(&self) -> usize {
        self.len()
    }
    fn get(&self, index: usize) -> Byte7 {
        Byte7::from(self.get(index))
    }
}
impl Mol2Vec<Bytes> for BytesVec {
    fn len(&self) -> usize {
        self.len()
    }
    fn get(&self, index: usize) -> Bytes {
        Bytes::from(self.get(index))
    }
}
impl Mol2Vec<Words> for WordsVec {
    fn len(&self) -> usize {
        self.len()
    }
    fn get(&self, index: usize) -> Words {
        Words::from(self.get(index))
    }
}
