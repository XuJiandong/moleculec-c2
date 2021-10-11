extern crate alloc;

use alloc::rc::Rc;
use alloc::vec::Vec;
use core::cell::RefCell;
use core::cmp::min;

#[derive(Debug)]
pub enum Error {
    Ok,
    Common,
    TotalSize,
    Header,
    Offset,
    UnknownItem,
    OutOfBound,
    FieldCount,
    Data,
    Overflow,
    Read,
}

pub trait Reader {
    fn read(&self, buf: &mut [u8], offset: usize) -> Result<usize, Error>;
}

pub const MAX_CACHE_SIZE: usize = 2048;
pub const MIN_CACHE_SIZE: usize = 64;
pub const NUM_T_SIZE: usize = 4;

pub struct DataSourceType {
    reader: Box<dyn Reader>,

    total_size: usize,
    start_point: usize,
    cache_size: usize,
    max_cache_size: usize,
    cache: Vec<u8>,
}

#[derive(Clone)]
pub struct CursorType {
    offset: usize,
    size: usize,
    data_source: Rc<RefCell<DataSourceType>>,
}

#[allow(dead_code)]
pub struct UnionType {
    item_id: usize,
    cursor: CursorType,
}

// it's an example about how to build a data source from memory
impl Reader for Vec<u8> {
    fn read(&self, buf: &mut [u8], offset: usize) -> Result<usize, Error> {
        let offset2 = offset as usize;
        let slice = self.as_slice();

        let slice2 = &slice[offset2..offset2 + buf.len()];
        buf.copy_from_slice(slice2);
        Ok(buf.len() as usize)
    }
}

pub fn make_cursor_from_memory(mem: Vec<u8>) -> CursorType {
    let cache_size = MAX_CACHE_SIZE;
    let mut cache = Vec::<u8>::new();
    let total_size = mem.len();

    cache.resize(cache_size, 0);
    let reader = Box::new(mem);

    let data_source = DataSourceType {
        reader,
        total_size,
        start_point: 0,
        cache_size,
        max_cache_size: MAX_CACHE_SIZE,
        cache,
    };
    CursorType {
        offset: 0,
        size: total_size,
        data_source: Rc::new(RefCell::new(data_source)),
    }
}
// end of example

pub fn read_at(cur: &CursorType, buff: &mut [u8]) -> Result<usize, Error> {
    let read_len = min(cur.size, buff.len() as usize);
    let mut ds = &mut *cur.data_source.borrow_mut();
    if read_len > ds.max_cache_size {
        return ds.reader.read(buff, cur.offset);
    }
    if cur.offset < ds.start_point || (cur.offset + read_len) > (ds.start_point + ds.cache_size) {
        let reader = &ds.reader;
        let size = reader.read(&mut ds.cache[..], cur.offset).unwrap();
        if size < read_len {
            panic!("read_at `if size < read_len`");
        }
        ds.cache_size = size;
        ds.start_point = cur.offset;

        if ds.cache_size > ds.max_cache_size {
            panic!("read_at `if ds.cache_size > ds.max_cache_size`");
        }
    }
    if cur.offset < ds.start_point || (cur.offset - ds.start_point) > ds.max_cache_size {
        panic!("read_at `if cur.offset < ds.start_point || ...`");
    }
    let read_point = cur.offset - ds.start_point;
    if read_point + read_len > ds.cache_size {
        panic!("read_at `if read_point + read_len > ds.cache_size`")
    }
    buff.copy_from_slice(&ds.cache[read_point as usize..(read_point + read_len) as usize]);
    Ok(read_len)
}

// mol2_seg_t : &[u8]
// mol2_cursor_res_t, use Result<CursorType, Error>

impl CursorType {
    pub fn add_offset(&mut self, offset: usize) {
        self.offset = self.offset.checked_add(offset).unwrap();
    }

    pub fn sub_size(&mut self, shrink_size: usize) {
        self.size = self.size.checked_sub(shrink_size).unwrap();
    }

    pub fn validate(&self) {
        if let Some(size) = self.offset.checked_add(self.size) {
            if size > self.data_source.borrow().total_size {
                panic!("validate: size > cur.source.total_size")
            }
        } else {
            panic!("validate")
        }
    }

    pub fn unpack_number(&self) -> usize {
        let mut src = [0u8; 4];
        let size = read_at(self, &mut src[..]).unwrap();
        if size != 4 {
            panic!("unpack_number");
        } else {
            let res = u32::from_le_bytes(src);
            res as usize
        }
    }

    pub fn verify_fixed_size(&self, total_size: usize) -> Result<(), Error> {
        if self.size == total_size {
            Ok(())
        } else {
            Err(Error::TotalSize)
        }
    }

    pub fn fixvec_verify(&self, item_size: usize) -> Result<(), Error> {
        if self.size < NUM_T_SIZE {
            panic!("fixvec_verify")
        }
        let item_count = self.unpack_number();
        if item_count == 0 {
            if self.size == NUM_T_SIZE {
                return Ok(());
            } else {
                return Err(Error::Header);
            }
        }

        let total_size = calculate_offset(item_size, item_count, NUM_T_SIZE);
        if self.size == total_size {
            Ok(())
        } else {
            Err(Error::TotalSize)
        }
    }

    pub fn option_is_none(&self) -> bool {
        self.size == 0
    }
    pub fn fixvec_length(&self) -> usize {
        self.unpack_number()
    }

    pub fn dynvec_length(&self) -> Result<usize, Error> {
        if self.size == NUM_T_SIZE {
            Ok(0)
        } else {
            let mut cur2 = self.clone();
            cur2.add_offset(NUM_T_SIZE);
            cur2.sub_size(NUM_T_SIZE);
            cur2.validate();
            Ok(cur2.get_item_count())
        }
    }

    pub fn get_item_count(&self) -> usize {
        let count = self.unpack_number() / 4;
        if count == 0 {
            panic!("get_item_count");
        } else {
            count - 1
        }
    }

    pub fn table_actual_field_count(&self) -> Result<usize, Error> {
        self.dynvec_length()
    }

    pub fn table_has_extra_fields(&self, field_count: usize) -> bool {
        let count = self.table_actual_field_count().unwrap();
        count > field_count
    }

    pub fn slice_by_offset(&self, offset: usize, size: usize) -> Result<CursorType, Error> {
        let mut cur2 = self.clone();
        cur2.add_offset(offset);
        cur2.size = size;
        cur2.validate();
        Ok(cur2)
    }

    pub fn fixvec_slice_by_index(
        &self,
        item_size: usize,
        item_index: usize,
    ) -> Result<CursorType, Error> {
        let mut cur2 = self.clone();
        let item_count = self.unpack_number();
        if item_index >= item_count {
            Err(Error::OutOfBound)
        } else {
            let offset = calculate_offset(item_size, item_index, NUM_T_SIZE);
            cur2.add_offset(offset);
            cur2.size = item_size;
            cur2.validate();
            Ok(cur2)
        }
    }

    pub fn dynvec_slice_by_index(&self, item_index: usize) -> Result<CursorType, Error> {
        let mut res = self.clone();
        let mut temp = self.clone();
        let total_size = self.unpack_number();
        temp.add_offset(NUM_T_SIZE);
        let item_count = temp.get_item_count();
        if item_index >= item_count {
            panic!("dynvec_slice_by_index");
        }
        temp.offset = self.offset;
        let temp_offset = calculate_offset(NUM_T_SIZE, item_index + 1, 0);
        temp.add_offset(temp_offset);
        let item_start = temp.unpack_number();
        if (item_index + 1) == item_count {
            res.offset = self.offset;
            res.add_offset(item_start);
            res.size = total_size;
            res.sub_size(item_start)
        } else {
            temp.offset = self.offset;
            let calc_offset = calculate_offset(NUM_T_SIZE, item_index + 2, 0);
            temp.add_offset(calc_offset);

            let item_end = temp.unpack_number();
            res.offset = self.offset;
            res.add_offset(item_start);
            res.size = item_end;
            res.sub_size(item_start);
        }
        res.validate();
        Ok(res)
    }

    pub fn table_slice_by_index(&self, field_index: usize) -> Result<CursorType, Error> {
        self.dynvec_slice_by_index(field_index)
    }

    pub fn fixvec_slice_raw_bytes(&self) -> Result<CursorType, Error> {
        let mut res = self.clone();
        res.add_offset(NUM_T_SIZE);
        res.size = self.unpack_number();
        res.validate();
        Ok(res)
    }

    pub fn convert_to_array(&self) -> Result<CursorType, Error> {
        Ok(self.clone())
    }

    pub fn convert_to_rawbytes(&self) -> Result<CursorType, Error> {
        self.fixvec_slice_raw_bytes()
    }

    pub fn union_unpack(&self) -> UnionType {
        let item_id = self.unpack_number();
        let mut cursor = self.clone();
        cursor.add_offset(NUM_T_SIZE);
        cursor.sub_size(NUM_T_SIZE);
        cursor.validate();
        UnionType { item_id, cursor }
    }
}

fn calculate_offset(item_size: usize, item_count: usize, offset: usize) -> usize {
    let res = item_size.checked_mul(item_count).unwrap();
    res.checked_add(offset).unwrap()
}

impl From<CursorType> for u64 {
    fn from(cur: CursorType) -> Self {
        let mut buf = [0u8; 8];
        let size = read_at(&cur, &mut buf[..]).unwrap();
        if size as usize != buf.len() {
            panic!("convert_to_u64");
        }
        u64::from_le_bytes(buf)
    }
}

impl From<CursorType> for i64 {
    fn from(cur: CursorType) -> Self {
        let mut buf = [0u8; 8];
        let size = read_at(&cur, &mut buf[..]).unwrap();
        if size as usize != buf.len() {
            panic!("convert_to_i64");
        }
        i64::from_le_bytes(buf)
    }
}

impl From<CursorType> for u32 {
    fn from(cur: CursorType) -> Self {
        let mut buf = [0u8; 4];
        let size = read_at(&cur, &mut buf[..]).unwrap();
        if size as usize != buf.len() {
            panic!("convert_to_usize");
        }
        u32::from_le_bytes(buf)
    }
}

impl From<CursorType> for i32 {
    fn from(cur: CursorType) -> Self {
        let mut buf = [0u8; 4];
        let size = read_at(&cur, &mut buf[..]).unwrap();
        if size as usize != buf.len() {
            panic!("convert_to_i32");
        }
        i32::from_le_bytes(buf)
    }
}

impl From<CursorType> for u16 {
    fn from(cur: CursorType) -> Self {
        let mut buf = [0u8; 2];
        let size = read_at(&cur, &mut buf[..]).unwrap();
        if size as usize != buf.len() {
            panic!("convert_to_u16");
        }
        u16::from_le_bytes(buf)
    }
}

impl From<CursorType> for i16 {
    fn from(cur: CursorType) -> Self {
        let mut buf = [0u8; 2];
        let size = read_at(&cur, &mut buf[..]).unwrap();
        if size as usize != buf.len() {
            panic!("convert_to_i16");
        }
        i16::from_le_bytes(buf)
    }
}

impl From<CursorType> for u8 {
    fn from(cur: CursorType) -> Self {
        let mut buf = [0u8; 1];
        let size = read_at(&cur, &mut buf[..]).unwrap();
        if size as usize != buf.len() {
            panic!("convert_to_u8");
        }
        u8::from_le_bytes(buf)
    }
}

impl From<CursorType> for i8 {
    fn from(cur: CursorType) -> Self {
        let mut buf = [0u8; 1];
        let size = read_at(&cur, &mut buf[..]).unwrap();
        if size as usize != buf.len() {
            panic!("convert_to_i8");
        }
        i8::from_le_bytes(buf)
    }
}
