
#![allow(dead_code)]
#![allow(unused_imports)]
extern crate alloc;
use alloc::vec::Vec;
use core::convert::TryInto;
use molecule2::Cursor;
use molecule2::Error;

pub struct Uint32 {
    pub cursor: Cursor,
}
impl From<Cursor> for Uint32 {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}
impl Uint32 {
    pub fn len(&self) -> usize {
        4
    }
}
impl Uint32 {
    pub fn get(&self, index: usize) -> Result<u8, Error> {
        let cur = self.cursor.slice_by_offset(1 * index, 1)?;
        cur.try_into()
    }
}
pub struct Uint64 {
    pub cursor: Cursor,
}
impl From<Cursor> for Uint64 {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}
impl Uint64 {
    pub fn len(&self) -> usize {
        8
    }
}
impl Uint64 {
    pub fn get(&self, index: usize) -> Result<u8, Error> {
        let cur = self.cursor.slice_by_offset(1 * index, 1)?;
        cur.try_into()
    }
}
pub struct Uint128 {
    pub cursor: Cursor,
}
impl From<Cursor> for Uint128 {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}
impl Uint128 {
    pub fn len(&self) -> usize {
        16
    }
}
impl Uint128 {
    pub fn get(&self, index: usize) -> Result<u8, Error> {
        let cur = self.cursor.slice_by_offset(1 * index, 1)?;
        cur.try_into()
    }
}
pub struct Byte32 {
    pub cursor: Cursor,
}
impl From<Cursor> for Byte32 {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}
impl Byte32 {
    pub fn len(&self) -> usize {
        32
    }
}
impl Byte32 {
    pub fn get(&self, index: usize) -> Result<u8, Error> {
        let cur = self.cursor.slice_by_offset(1 * index, 1)?;
        cur.try_into()
    }
}
pub struct Uint256 {
    pub cursor: Cursor,
}
impl From<Cursor> for Uint256 {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}
impl Uint256 {
    pub fn len(&self) -> usize {
        32
    }
}
impl Uint256 {
    pub fn get(&self, index: usize) -> Result<u8, Error> {
        let cur = self.cursor.slice_by_offset(1 * index, 1)?;
        cur.try_into()
    }
}
pub struct Bytes {
    pub cursor: Cursor,
}
impl From<Cursor> for Bytes {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}
impl Bytes {
    pub fn len(&self) -> Result<usize, Error> {
        self.cursor.fixvec_length()
    }
}
impl Bytes {
    pub fn get(&self, index: usize) -> Result<u8, Error> {
        let cur = self.cursor.fixvec_slice_by_index(1, index)?;
        cur.try_into()
    }
}
pub struct BytesOpt {
    pub cursor: Cursor,
}
impl From<Cursor> for BytesOpt {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}
pub struct BytesVec {
    pub cursor: Cursor,
}
impl From<Cursor> for BytesVec {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}
impl BytesVec {
    pub fn len(&self) -> Result<usize, Error> {
        self.cursor.dynvec_length()
    }
}
impl BytesVec {
    pub fn get(&self, index: usize) -> Result<Cursor, Error> {
        let cur = self.cursor.dynvec_slice_by_index(index)?;
        cur.convert_to_rawbytes()
    }
}
pub struct Byte32Vec {
    pub cursor: Cursor,
}
impl From<Cursor> for Byte32Vec {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}
impl Byte32Vec {
    pub fn len(&self) -> Result<usize, Error> {
        self.cursor.fixvec_length()
    }
}
impl Byte32Vec {
    pub fn get(&self, index: usize) -> Result<Cursor, Error> {
        let cur = self.cursor.fixvec_slice_by_index(32, index)?;
        Ok(cur)
    }
}
pub struct ScriptOpt {
    pub cursor: Cursor,
}
impl From<Cursor> for ScriptOpt {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}
pub struct ProposalShortId {
    pub cursor: Cursor,
}
impl From<Cursor> for ProposalShortId {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}
impl ProposalShortId {
    pub fn len(&self) -> usize {
        10
    }
}
impl ProposalShortId {
    pub fn get(&self, index: usize) -> Result<u8, Error> {
        let cur = self.cursor.slice_by_offset(1 * index, 1)?;
        cur.try_into()
    }
}
pub struct UncleBlockVec {
    pub cursor: Cursor,
}
impl From<Cursor> for UncleBlockVec {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}
impl UncleBlockVec {
    pub fn len(&self) -> Result<usize, Error> {
        self.cursor.dynvec_length()
    }
}
impl UncleBlockVec {
    pub fn get(&self, index: usize) -> Result<UncleBlock, Error> {
        let cur = self.cursor.dynvec_slice_by_index(index)?;
        Ok(cur.into())
    }
}
pub struct TransactionVec {
    pub cursor: Cursor,
}
impl From<Cursor> for TransactionVec {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}
impl TransactionVec {
    pub fn len(&self) -> Result<usize, Error> {
        self.cursor.dynvec_length()
    }
}
impl TransactionVec {
    pub fn get(&self, index: usize) -> Result<Transaction, Error> {
        let cur = self.cursor.dynvec_slice_by_index(index)?;
        Ok(cur.into())
    }
}
pub struct ProposalShortIdVec {
    pub cursor: Cursor,
}
impl From<Cursor> for ProposalShortIdVec {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}
impl ProposalShortIdVec {
    pub fn len(&self) -> Result<usize, Error> {
        self.cursor.fixvec_length()
    }
}
impl ProposalShortIdVec {
    pub fn get(&self, index: usize) -> Result<Cursor, Error> {
        let cur = self.cursor.fixvec_slice_by_index(10, index)?;
        Ok(cur)
    }
}
pub struct CellDepVec {
    pub cursor: Cursor,
}
impl From<Cursor> for CellDepVec {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}
impl CellDepVec {
    pub fn len(&self) -> Result<usize, Error> {
        self.cursor.fixvec_length()
    }
}
impl CellDepVec {
    pub fn get(&self, index: usize) -> Result<CellDep, Error> {
        let cur = self.cursor.fixvec_slice_by_index(37, index)?;
        Ok(cur.into())
    }
}
pub struct CellInputVec {
    pub cursor: Cursor,
}
impl From<Cursor> for CellInputVec {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}
impl CellInputVec {
    pub fn len(&self) -> Result<usize, Error> {
        self.cursor.fixvec_length()
    }
}
impl CellInputVec {
    pub fn get(&self, index: usize) -> Result<CellInput, Error> {
        let cur = self.cursor.fixvec_slice_by_index(44, index)?;
        Ok(cur.into())
    }
}
pub struct CellOutputVec {
    pub cursor: Cursor,
}
impl From<Cursor> for CellOutputVec {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}
impl CellOutputVec {
    pub fn len(&self) -> Result<usize, Error> {
        self.cursor.dynvec_length()
    }
}
impl CellOutputVec {
    pub fn get(&self, index: usize) -> Result<CellOutput, Error> {
        let cur = self.cursor.dynvec_slice_by_index(index)?;
        Ok(cur.into())
    }
}
pub struct Script {
    pub cursor: Cursor,
}
impl From<Cursor> for Script {
    fn from(cursor: Cursor) -> Self {
        Script { cursor }
    }
}
impl Script {
    pub fn code_hash(&self) -> Result<Cursor, Error> {
        let cur = self.cursor.table_slice_by_index(0)?;
        Ok(cur)
    }
}

impl Script {
    pub fn hash_type(&self) -> Result<u8, Error> {
        let cur = self.cursor.table_slice_by_index(1)?;
        cur.try_into()
    }
}

impl Script {
    pub fn args(&self) -> Result<Cursor, Error> {
        let cur = self.cursor.table_slice_by_index(2)?;
        cur.convert_to_rawbytes()
    }
}
pub struct OutPoint {
    pub cursor: Cursor,
}
impl From<Cursor> for OutPoint {
    fn from(cursor: Cursor) -> Self {
        OutPoint { cursor }
    }
}
impl OutPoint {
    pub fn tx_hash(&self) -> Result<Cursor, Error> {
        let cur = self.cursor.slice_by_offset(0, 32)?;
        Ok(cur)
    }
}

impl OutPoint {
    pub fn index(&self) -> Result<u32, Error> {
        let cur = self.cursor.slice_by_offset(32, 4)?;
        cur.try_into()
    }
}
pub struct CellInput {
    pub cursor: Cursor,
}
impl From<Cursor> for CellInput {
    fn from(cursor: Cursor) -> Self {
        CellInput { cursor }
    }
}
impl CellInput {
    pub fn since(&self) -> Result<u64, Error> {
        let cur = self.cursor.slice_by_offset(0, 8)?;
        cur.try_into()
    }
}

impl CellInput {
    pub fn previous_output(&self) -> Result<OutPoint, Error> {
        let cur = self.cursor.slice_by_offset(8, 36)?;
        Ok(cur.into())
    }
}
pub struct CellOutput {
    pub cursor: Cursor,
}
impl From<Cursor> for CellOutput {
    fn from(cursor: Cursor) -> Self {
        CellOutput { cursor }
    }
}
impl CellOutput {
    pub fn capacity(&self) -> Result<u64, Error> {
        let cur = self.cursor.table_slice_by_index(0)?;
        cur.try_into()
    }
}

impl CellOutput {
    pub fn lock(&self) -> Result<Script, Error> {
        let cur = self.cursor.table_slice_by_index(1)?;
        Ok(cur.into())
    }
}

impl CellOutput {
    pub fn type_(&self) -> Result<Option<Script>, Error> {
        let cur = self.cursor.table_slice_by_index(2)?;
        if cur.option_is_none() {
            Ok(None)
        } else {
            Ok(Some(cur.into()))
        }
    }
}
pub struct CellDep {
    pub cursor: Cursor,
}
impl From<Cursor> for CellDep {
    fn from(cursor: Cursor) -> Self {
        CellDep { cursor }
    }
}
impl CellDep {
    pub fn out_point(&self) -> Result<OutPoint, Error> {
        let cur = self.cursor.slice_by_offset(0, 36)?;
        Ok(cur.into())
    }
}

impl CellDep {
    pub fn dep_type(&self) -> Result<u8, Error> {
        let cur = self.cursor.slice_by_offset(36, 1)?;
        cur.try_into()
    }
}
pub struct RawTransaction {
    pub cursor: Cursor,
}
impl From<Cursor> for RawTransaction {
    fn from(cursor: Cursor) -> Self {
        RawTransaction { cursor }
    }
}
impl RawTransaction {
    pub fn version(&self) -> Result<u32, Error> {
        let cur = self.cursor.table_slice_by_index(0)?;
        cur.try_into()
    }
}

impl RawTransaction {
    pub fn cell_deps(&self) -> Result<CellDepVec, Error> {
        let cur = self.cursor.table_slice_by_index(1)?;
        Ok(cur.into())
    }
}

impl RawTransaction {
    pub fn header_deps(&self) -> Result<Byte32Vec, Error> {
        let cur = self.cursor.table_slice_by_index(2)?;
        Ok(cur.into())
    }
}

impl RawTransaction {
    pub fn inputs(&self) -> Result<CellInputVec, Error> {
        let cur = self.cursor.table_slice_by_index(3)?;
        Ok(cur.into())
    }
}

impl RawTransaction {
    pub fn outputs(&self) -> Result<CellOutputVec, Error> {
        let cur = self.cursor.table_slice_by_index(4)?;
        Ok(cur.into())
    }
}

impl RawTransaction {
    pub fn outputs_data(&self) -> Result<BytesVec, Error> {
        let cur = self.cursor.table_slice_by_index(5)?;
        Ok(cur.into())
    }
}
pub struct Transaction {
    pub cursor: Cursor,
}
impl From<Cursor> for Transaction {
    fn from(cursor: Cursor) -> Self {
        Transaction { cursor }
    }
}
impl Transaction {
    pub fn raw(&self) -> Result<RawTransaction, Error> {
        let cur = self.cursor.table_slice_by_index(0)?;
        Ok(cur.into())
    }
}

impl Transaction {
    pub fn witnesses(&self) -> Result<BytesVec, Error> {
        let cur = self.cursor.table_slice_by_index(1)?;
        Ok(cur.into())
    }
}
pub struct RawHeader {
    pub cursor: Cursor,
}
impl From<Cursor> for RawHeader {
    fn from(cursor: Cursor) -> Self {
        RawHeader { cursor }
    }
}
impl RawHeader {
    pub fn version(&self) -> Result<u32, Error> {
        let cur = self.cursor.slice_by_offset(0, 4)?;
        cur.try_into()
    }
}

impl RawHeader {
    pub fn compact_target(&self) -> Result<u32, Error> {
        let cur = self.cursor.slice_by_offset(4, 4)?;
        cur.try_into()
    }
}

impl RawHeader {
    pub fn timestamp(&self) -> Result<u64, Error> {
        let cur = self.cursor.slice_by_offset(8, 8)?;
        cur.try_into()
    }
}

impl RawHeader {
    pub fn number(&self) -> Result<u64, Error> {
        let cur = self.cursor.slice_by_offset(16, 8)?;
        cur.try_into()
    }
}

impl RawHeader {
    pub fn epoch(&self) -> Result<u64, Error> {
        let cur = self.cursor.slice_by_offset(24, 8)?;
        cur.try_into()
    }
}

impl RawHeader {
    pub fn parent_hash(&self) -> Result<Cursor, Error> {
        let cur = self.cursor.slice_by_offset(32, 32)?;
        Ok(cur)
    }
}

impl RawHeader {
    pub fn transactions_root(&self) -> Result<Cursor, Error> {
        let cur = self.cursor.slice_by_offset(64, 32)?;
        Ok(cur)
    }
}

impl RawHeader {
    pub fn proposals_hash(&self) -> Result<Cursor, Error> {
        let cur = self.cursor.slice_by_offset(96, 32)?;
        Ok(cur)
    }
}

impl RawHeader {
    pub fn uncles_hash(&self) -> Result<Cursor, Error> {
        let cur = self.cursor.slice_by_offset(128, 32)?;
        Ok(cur)
    }
}

impl RawHeader {
    pub fn dao(&self) -> Result<Cursor, Error> {
        let cur = self.cursor.slice_by_offset(160, 32)?;
        Ok(cur)
    }
}
pub struct Header {
    pub cursor: Cursor,
}
impl From<Cursor> for Header {
    fn from(cursor: Cursor) -> Self {
        Header { cursor }
    }
}
impl Header {
    pub fn raw(&self) -> Result<RawHeader, Error> {
        let cur = self.cursor.slice_by_offset(0, 192)?;
        Ok(cur.into())
    }
}

impl Header {
    pub fn nonce(&self) -> Result<Cursor, Error> {
        let cur = self.cursor.slice_by_offset(192, 16)?;
        Ok(cur)
    }
}
pub struct UncleBlock {
    pub cursor: Cursor,
}
impl From<Cursor> for UncleBlock {
    fn from(cursor: Cursor) -> Self {
        UncleBlock { cursor }
    }
}
impl UncleBlock {
    pub fn header(&self) -> Result<Header, Error> {
        let cur = self.cursor.table_slice_by_index(0)?;
        Ok(cur.into())
    }
}

impl UncleBlock {
    pub fn proposals(&self) -> Result<ProposalShortIdVec, Error> {
        let cur = self.cursor.table_slice_by_index(1)?;
        Ok(cur.into())
    }
}
pub struct Block {
    pub cursor: Cursor,
}
impl From<Cursor> for Block {
    fn from(cursor: Cursor) -> Self {
        Block { cursor }
    }
}
impl Block {
    pub fn header(&self) -> Result<Header, Error> {
        let cur = self.cursor.table_slice_by_index(0)?;
        Ok(cur.into())
    }
}

impl Block {
    pub fn uncles(&self) -> Result<UncleBlockVec, Error> {
        let cur = self.cursor.table_slice_by_index(1)?;
        Ok(cur.into())
    }
}

impl Block {
    pub fn transactions(&self) -> Result<TransactionVec, Error> {
        let cur = self.cursor.table_slice_by_index(2)?;
        Ok(cur.into())
    }
}

impl Block {
    pub fn proposals(&self) -> Result<ProposalShortIdVec, Error> {
        let cur = self.cursor.table_slice_by_index(3)?;
        Ok(cur.into())
    }
}
pub struct CellbaseWitness {
    pub cursor: Cursor,
}
impl From<Cursor> for CellbaseWitness {
    fn from(cursor: Cursor) -> Self {
        CellbaseWitness { cursor }
    }
}
impl CellbaseWitness {
    pub fn lock(&self) -> Result<Script, Error> {
        let cur = self.cursor.table_slice_by_index(0)?;
        Ok(cur.into())
    }
}

impl CellbaseWitness {
    pub fn message(&self) -> Result<Cursor, Error> {
        let cur = self.cursor.table_slice_by_index(1)?;
        cur.convert_to_rawbytes()
    }
}
pub struct WitnessArgs {
    pub cursor: Cursor,
}
impl From<Cursor> for WitnessArgs {
    fn from(cursor: Cursor) -> Self {
        WitnessArgs { cursor }
    }
}
impl WitnessArgs {
    pub fn lock(&self) -> Result<Option<Cursor>, Error> {
        let cur = self.cursor.table_slice_by_index(0)?;
        if cur.option_is_none() {
            Ok(None)
        } else {
            let cur = cur.convert_to_rawbytes()?;
            Ok(Some(cur.into()))
        }
    }
}

impl WitnessArgs {
    pub fn input_type(&self) -> Result<Option<Cursor>, Error> {
        let cur = self.cursor.table_slice_by_index(1)?;
        if cur.option_is_none() {
            Ok(None)
        } else {
            let cur = cur.convert_to_rawbytes()?;
            Ok(Some(cur.into()))
        }
    }
}

impl WitnessArgs {
    pub fn output_type(&self) -> Result<Option<Cursor>, Error> {
        let cur = self.cursor.table_slice_by_index(2)?;
        if cur.option_is_none() {
            Ok(None)
        } else {
            let cur = cur.convert_to_rawbytes()?;
            Ok(Some(cur.into()))
        }
    }
}
