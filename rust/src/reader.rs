#![allow(dead_code)]
#![allow(unused_variables)]

use std::cell::RefCell;
use std::cmp::min;
use std::rc::Rc;

#[derive(Debug)]
enum Error {
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
}

trait Reader {
    fn read(&self, buf: &mut [u8], offset: u32) -> Result<u32, Error>;
}

const MAX_CACHE_SIZE: u32 = 2048;
const MIN_CACHE_SIZE: u32 = 64;
const NUM_T_SIZE: u32 = 4;

struct DataSourceType {
    reader: Box<dyn Reader>,

    total_size: u32,
    start_point: u32,
    cache_size: u32,
    max_cache_size: u32,
    cache: Vec<u8>,
}

#[derive(Clone)]
struct CursorType {
    offset: u32,
    size: u32,
    data_source: Rc<RefCell<DataSourceType>>,
}

fn source_memory(buff: &mut [u8], offset: u32) -> Result<u32, Error> {
    Ok(0)
}

// fn make_cursor_from_memory(memory: &[u8]) -> CursorType {
// }

fn read_at(cur: &CursorType, buff: &mut [u8]) -> Result<u32, Error> {
    let read_len = min(cur.size, buff.len() as u32);
    let reader = &cur.data_source.borrow().reader;

    let mut ds = cur.data_source.borrow_mut();
    if read_len > ds.max_cache_size {
        return ds.reader.read(buff, cur.offset);
    }
    if cur.offset < ds.start_point ||
        (cur.offset + read_len) > (ds.start_point + ds.cache_size) {
        let size = reader.read(&mut ds.cache[..], cur.offset).unwrap();
        if size < read_len {
            panic!("read_at");
        }
        ds.cache_size = size;
        ds.start_point = cur.offset;

        if ds.cache_size > ds.max_cache_size {
            panic!("read_at");
        }
    }
    if cur.offset < ds.start_point ||
        (cur.offset - ds.start_point) > ds.max_cache_size {
        panic!("read_at");
    }
    let read_point = cur.offset - ds.start_point;
    if read_point + read_len > ds.cache_size {
        panic!("read_at")
    }
    buff.copy_from_slice(&ds.cache[read_point as usize..(read_point + read_len) as usize]);
    Ok(read_len)
}

// mol2_seg_t : &[u8]

struct UnionTypez {
    item_id: u32,
    cursor: CursorType,
}

// mol2_cursor_res_t, use Result<CursorType, Error>

fn add_offset(cur: &CursorType, offset: u32) -> CursorType {
    let offset = cur.offset.checked_add(offset).unwrap();
    let mut res = cur.clone();
    res.offset = offset;
    res
}

fn sub_size(cur: &CursorType, shrinked_size: u32) -> CursorType {
    let size = cur.size.checked_sub(shrinked_size).unwrap();
    let mut res = cur.clone();
    res.size = size;
    res
}

fn validate(cur: &CursorType) {
    if let Some(size) = cur.offset.checked_add(cur.size) {
        if size > cur.data_source.borrow().total_size {
            panic!("validate: size > cur.source.total_size")
        }
    } else {
        panic!("validate")
    }
}

fn unpack_number(cur: &CursorType) -> u32 {
    let mut src = [0u8; 4];
    let size = read_at(cur, &mut src[..]).unwrap();
    if size != 4 {
        panic!("unpack_number");
    } else {
        let res = u32::from_le_bytes(src);
        res
    }
}

fn verify_fixed_size(cur: CursorType, total_size: u32) -> Result<(), Error> {
    if cur.size == total_size {
        Ok(())
    } else {
        Err(Error::TotalSize)
    }
}


fn fixvec_verify(cur: &CursorType, item_size: u32) -> Result<(), Error> {
    if cur.size < NUM_T_SIZE {
        panic!("fixvec_verify")
    }
    let item_count = unpack_number(cur);
    if item_count == 0 {
        if cur.size == NUM_T_SIZE {
            return Ok(());
        } else {
            return Err(Error::Header);
        }
    }

    let total_size = calculate_offset(item_size, item_count, NUM_T_SIZE);
    if cur.size == total_size {
        Ok(())
    } else {
        Err(Error::TotalSize)
    }
}

fn calculate_offset(item_size: u32, item_count: u32, offset: u32) -> u32 {
    let res = item_size.checked_mul(item_count).unwrap();
    res.checked_add(offset).unwrap()
}

fn option_is_none(cur: CursorType) -> bool {
    cur.size == 0
}

// fn union_unpack(cur: CursorType) -> UnionType {
// }

fn fixvec_length(cur: CursorType) -> u32 {
    unpack_number(&cur)
}

fn dynvec_length(cur: &CursorType) -> Result<u32, Error> {
    if cur.size == NUM_T_SIZE {
        Ok(0)
    } else {
        let mut cur2 = cur.clone();
        cur2 = add_offset(&cur2, NUM_T_SIZE);
        cur2 = sub_size(&cur2, NUM_T_SIZE);
        validate(&cur2);
        Ok(get_item_count(&cur2))
    }
}

fn get_item_count(cur: &CursorType) -> u32 {
    let count = unpack_number(cur) / 4;
    if count == 0 {
        panic!("get_item_count");
    } else {
        count - 1
    }
}


fn table_actual_field_count(cur: &CursorType) -> Result<u32, Error> {
    dynvec_length(cur)
}


fn table_has_extra_fields(cur: &CursorType, field_count: u32) -> bool {
    let count = table_actual_field_count(cur).unwrap();
    count > field_count
}

fn slice_by_offset(cur: &CursorType, offset: u32, size: u32) -> Result<CursorType, Error> {
    let mut cur2 = cur.clone();
    cur2 = add_offset(&cur2, offset);
    cur2.size = size;
    validate(&cur2);
    Ok(cur2)
}

fn fixvec_slice_by_index(cur: &CursorType, item_size: u32, item_index: u32) -> Result<CursorType, Error> {
    let mut cur2 = cur.clone();
    let item_count = unpack_number(cur);
    if item_index >= item_count {
        Err(Error::OutOfBound)
    } else {
        let offset = calculate_offset(item_size, item_index, NUM_T_SIZE);
        cur2 = add_offset(&cur2, offset);
        cur2.size = item_size;
        validate(&cur2);
        Ok(cur2)
    }
}

fn dynvec_slice_by_index(cur: &CursorType, item_index: u32) -> Result<CursorType, Error> {
    let mut res = cur.clone();
    let mut temp = cur.clone();
    let total_size = unpack_number(cur);
    temp = add_offset(&temp, NUM_T_SIZE);
    let item_count = get_item_count(&temp);
    if item_index >= item_count {
        panic!("dynvec_slice_by_index");
    }
    temp.offset = cur.offset;
    let temp_offset = calculate_offset(NUM_T_SIZE, item_index + 1, 0);
    temp = add_offset(&temp, temp_offset);
    let item_start = unpack_number(&temp);
    if (item_index + 1) == item_count {
        res.offset = cur.offset;
        res = add_offset(&res, item_start);
        res.size = total_size;
        res = sub_size(&cur, item_start)
    } else {
        temp.offset = cur.offset;
        let calc_offset = calculate_offset(NUM_T_SIZE, item_index + 2, 0);
        temp = add_offset(&temp, calc_offset);

        let item_end = unpack_number(&temp);
        res.offset = cur.offset;
        res = add_offset(&res, item_start);
        res.size = item_end;
        res = sub_size(&res, item_start);
    }
    validate(&res);
    Ok(res)
}

fn table_slice_by_index(cur: &CursorType, field_index: u32) -> Result<CursorType, Error> {
    dynvec_slice_by_index(cur, field_index)
}

fn fixvec_slice_raw_bytes(cur: &CursorType) -> Result<CursorType, Error> {
    let mut res = add_offset(&cur, NUM_T_SIZE);
    res.size = unpack_number(cur);
    validate(cur);
    Ok(res)
}

fn convert_to_u64(cur: &CursorType) -> Result<u64, Error> {
    let mut buf = [0u8; 8];
    let size = read_at(cur, &mut buf[..]).unwrap();
    if size as usize != buf.len() {
        panic!("convert_to_u64");
    }
    Ok(u64::from_le_bytes(buf))
}

fn convert_to_i64(cur: &CursorType) -> Result<i64, Error> {
    let mut buf = [0u8; 8];
    let size = read_at(cur, &mut buf[..]).unwrap();
    if size as usize != buf.len() {
        panic!("convert_to_i64");
    }
    Ok(i64::from_le_bytes(buf))
}

fn convert_to_u32(cur: &CursorType) -> Result<u32, Error> {
    let mut buf = [0u8; 4];
    let size = read_at(cur, &mut buf[..]).unwrap();
    if size as usize != buf.len() {
        panic!("convert_to_u32");
    }
    Ok(u32::from_le_bytes(buf))
}


fn convert_to_i32(cur: &CursorType) -> Result<i32, Error> {
    let mut buf = [0u8; 4];
    let size = read_at(cur, &mut buf[..]).unwrap();
    if size as usize != buf.len() {
        panic!("convert_to_i32");
    }
    Ok(i32::from_le_bytes(buf))
}

fn convert_to_u16(cur: &CursorType) -> Result<u16, Error> {
    let mut buf = [0u8; 2];
    let size = read_at(cur, &mut buf[..]).unwrap();
    if size as usize != buf.len() {
        panic!("convert_to_u16");
    }
    Ok(u16::from_le_bytes(buf))
}


fn convert_to_i16(cur: &CursorType) -> Result<i16, Error> {
    let mut buf = [0u8; 2];
    let size = read_at(cur, &mut buf[..]).unwrap();
    if size as usize != buf.len() {
        panic!("convert_to_i16");
    }
    Ok(i16::from_le_bytes(buf))
}

fn convert_to_u8(cur: &CursorType) -> Result<u8, Error> {
    let mut buf = [0u8; 1];
    let size = read_at(cur, &mut buf[..]).unwrap();
    if size as usize != buf.len() {
        panic!("convert_to_u8");
    }
    Ok(u8::from_le_bytes(buf))
}

fn convert_to_i8(cur: &CursorType) -> Result<i8, Error> {
    let mut buf = [0u8; 1];
    let size = read_at(cur, &mut buf[..]).unwrap();
    if size as usize != buf.len() {
        panic!("convert_to_i8");
    }
    Ok(i8::from_le_bytes(buf))
}

fn convert_to_array(cur: &CursorType) -> Result<CursorType, Error> {
    Ok(cur.clone())
}

fn convert_to_rawbytes(cur: &CursorType) -> Result<CursorType, Error> {
    fixvec_slice_raw_bytes(cur)
}