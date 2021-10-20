
#![allow(dead_code)]
#![allow(unused_imports)]
extern crate alloc;
use alloc::vec::Vec;
use molecule2::Cursor;

pub struct Uint32 {
    cursor: Cursor,
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
    pub fn get(&self, index: usize) -> u8 {
        let cursor = self.cursor.slice_by_offset(1 * index, 1).unwrap();
        cursor.into()
    }
}

pub struct Uint64 {
    cursor: Cursor,
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
    pub fn get(&self, index: usize) -> u8 {
        let cursor = self.cursor.slice_by_offset(1 * index, 1).unwrap();
        cursor.into()
    }
}

pub struct Uint128 {
    cursor: Cursor,
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
    pub fn get(&self, index: usize) -> u8 {
        let cursor = self.cursor.slice_by_offset(1 * index, 1).unwrap();
        cursor.into()
    }
}

pub struct Byte32 {
    cursor: Cursor,
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
    pub fn get(&self, index: usize) -> u8 {
        let cursor = self.cursor.slice_by_offset(1 * index, 1).unwrap();
        cursor.into()
    }
}

pub struct Uint256 {
    cursor: Cursor,
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
    pub fn get(&self, index: usize) -> u8 {
        let cursor = self.cursor.slice_by_offset(1 * index, 1).unwrap();
        cursor.into()
    }
}

pub struct Bytes {
    cursor: Cursor,
}

impl From<Cursor> for Bytes {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}

impl Bytes {
    pub fn len(&self) -> usize {
        self.cursor.fixvec_length()
    }
}
impl Bytes {
    pub fn get(&self, index: usize) -> u8 {
        let cursor = self.cursor.fixvec_slice_by_index(1, index).unwrap();
        cursor.into()
    }
}
// warning: BytesOpt not implemented for Rust
pub struct BytesOpt {
    cursor: Cursor,
}
impl From<Cursor> for BytesOpt {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}

pub struct BytesVec {
    cursor: Cursor,
}

impl From<Cursor> for BytesVec {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}

impl BytesVec {
    pub fn len(&self) -> usize {
        self.cursor.dynvec_length()
    }
}
impl BytesVec {
    pub fn get(&self, index: usize) -> Vec<u8> {
        let cur = self.cursor.dynvec_slice_by_index(index).unwrap();
        let cur2 = cur.convert_to_rawbytes().unwrap();
        cur2.into()
    }
}

pub struct Byte32Vec {
    cursor: Cursor,
}

impl From<Cursor> for Byte32Vec {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}

impl Byte32Vec {
    pub fn len(&self) -> usize {
        self.cursor.fixvec_length()
    }
}
impl Byte32Vec {
    pub fn get(&self, index: usize) -> Vec<u8> {
        let cursor = self.cursor.fixvec_slice_by_index(32, index).unwrap();
        cursor.into()
    }
}
// warning: ScriptOpt not implemented for Rust
pub struct ScriptOpt {
    cursor: Cursor,
}
impl From<Cursor> for ScriptOpt {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}

pub struct ProposalShortId {
    cursor: Cursor,
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
    pub fn get(&self, index: usize) -> u8 {
        let cursor = self.cursor.slice_by_offset(1 * index, 1).unwrap();
        cursor.into()
    }
}

pub struct UncleBlockVec {
    cursor: Cursor,
}

impl From<Cursor> for UncleBlockVec {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}

impl UncleBlockVec {
    pub fn len(&self) -> usize {
        self.cursor.dynvec_length()
    }
}
impl UncleBlockVec {
    pub fn get(&self, index: usize) -> UncleBlock {
        let cursor = self.cursor.dynvec_slice_by_index(index).unwrap();
        UncleBlock { cursor }
    }
}

pub struct TransactionVec {
    cursor: Cursor,
}

impl From<Cursor> for TransactionVec {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}

impl TransactionVec {
    pub fn len(&self) -> usize {
        self.cursor.dynvec_length()
    }
}
impl TransactionVec {
    pub fn get(&self, index: usize) -> Transaction {
        let cursor = self.cursor.dynvec_slice_by_index(index).unwrap();
        Transaction { cursor }
    }
}

pub struct ProposalShortIdVec {
    cursor: Cursor,
}

impl From<Cursor> for ProposalShortIdVec {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}

impl ProposalShortIdVec {
    pub fn len(&self) -> usize {
        self.cursor.fixvec_length()
    }
}
impl ProposalShortIdVec {
    pub fn get(&self, index: usize) -> Vec<u8> {
        let cursor = self.cursor.fixvec_slice_by_index(10, index).unwrap();
        cursor.into()
    }
}

pub struct CellDepVec {
    cursor: Cursor,
}

impl From<Cursor> for CellDepVec {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}

impl CellDepVec {
    pub fn len(&self) -> usize {
        self.cursor.fixvec_length()
    }
}
impl CellDepVec {
    pub fn get(&self, index: usize) -> CellDep {
        let cursor = self.cursor.fixvec_slice_by_index(37, index).unwrap();
        CellDep { cursor }
    }
}

pub struct CellInputVec {
    cursor: Cursor,
}

impl From<Cursor> for CellInputVec {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}

impl CellInputVec {
    pub fn len(&self) -> usize {
        self.cursor.fixvec_length()
    }
}
impl CellInputVec {
    pub fn get(&self, index: usize) -> CellInput {
        let cursor = self.cursor.fixvec_slice_by_index(44, index).unwrap();
        CellInput { cursor }
    }
}

pub struct CellOutputVec {
    cursor: Cursor,
}

impl From<Cursor> for CellOutputVec {
    fn from(cursor: Cursor) -> Self {
        Self { cursor }
    }
}

impl CellOutputVec {
    pub fn len(&self) -> usize {
        self.cursor.dynvec_length()
    }
}
impl CellOutputVec {
    pub fn get(&self, index: usize) -> CellOutput {
        let cursor = self.cursor.dynvec_slice_by_index(index).unwrap();
        CellOutput { cursor }
    }
}

pub struct Script {
    cursor: Cursor,
}

impl From<Cursor> for Script {
    fn from(cursor: Cursor) -> Self {
        Script { cursor }
    }
}
impl Script {
    pub fn code_hash(&self) -> Vec<u8> {
        let cur = self.cursor.table_slice_by_index(0).unwrap();
        cur.into()
    }
}
impl Script {
    pub fn hash_type(&self) -> u8 {
        let cur = self.cursor.table_slice_by_index(1).unwrap();
        cur.into()
    }
}
impl Script {
    pub fn args(&self) -> Vec<u8> {
        let cur = self.cursor.table_slice_by_index(2).unwrap();
        let cur2 = cur.convert_to_rawbytes().unwrap();
        cur2.into()
    }
}
pub struct OutPoint {
    cursor: Cursor,
}

impl From<Cursor> for OutPoint {
    fn from(cursor: Cursor) -> Self {
        OutPoint { cursor }
    }
}
impl OutPoint {
    pub fn tx_hash(&self) -> Vec<u8> {
        let cur = self.cursor.slice_by_offset(0, 32).unwrap();
        cur.into()
    }
}
impl OutPoint {
    pub fn index(&self) -> u32 {
        let cur = self.cursor.slice_by_offset(32, 4).unwrap();
        cur.into()
    }
}
pub struct CellInput {
    cursor: Cursor,
}

impl From<Cursor> for CellInput {
    fn from(cursor: Cursor) -> Self {
        CellInput { cursor }
    }
}
impl CellInput {
    pub fn since(&self) -> u64 {
        let cur = self.cursor.slice_by_offset(0, 8).unwrap();
        cur.into()
    }
}
impl CellInput {
    pub fn previous_output(&self) -> OutPoint {
        let cur = self.cursor.slice_by_offset(8, 36).unwrap();
        cur.into()
    }
}

pub struct CellOutput {
    cursor: Cursor,
}

impl From<Cursor> for CellOutput {
    fn from(cursor: Cursor) -> Self {
        CellOutput { cursor }
    }
}
impl CellOutput {
    pub fn capacity(&self) -> u64 {
        let cur = self.cursor.table_slice_by_index(0).unwrap();
        cur.into()
    }
}
impl CellOutput {
    pub fn lock(&self) -> Script {
        let cur = self.cursor.table_slice_by_index(1).unwrap();
        cur.into()
    }
}

impl CellOutput {
    pub fn type_(&self) -> Option<Script> {
        let cur = self.cursor.table_slice_by_index(2).unwrap();
        if cur.option_is_none() {
            None
        } else {
            Some(cur.into())
        }
    }
}

pub struct CellDep {
    cursor: Cursor,
}

impl From<Cursor> for CellDep {
    fn from(cursor: Cursor) -> Self {
        CellDep { cursor }
    }
}
impl CellDep {
    pub fn out_point(&self) -> OutPoint {
        let cur = self.cursor.slice_by_offset(0, 36).unwrap();
        cur.into()
    }
}

impl CellDep {
    pub fn dep_type(&self) -> u8 {
        let cur = self.cursor.slice_by_offset(36, 1).unwrap();
        cur.into()
    }
}
pub struct RawTransaction {
    cursor: Cursor,
}

impl From<Cursor> for RawTransaction {
    fn from(cursor: Cursor) -> Self {
        RawTransaction { cursor }
    }
}
impl RawTransaction {
    pub fn version(&self) -> u32 {
        let cur = self.cursor.table_slice_by_index(0).unwrap();
        cur.into()
    }
}
impl RawTransaction {
    pub fn cell_deps(&self) -> CellDepVec {
        let cur = self.cursor.table_slice_by_index(1).unwrap();
        cur.into()
    }
}

impl RawTransaction {
    pub fn header_deps(&self) -> Byte32Vec {
        let cur = self.cursor.table_slice_by_index(2).unwrap();
        cur.into()
    }
}

impl RawTransaction {
    pub fn inputs(&self) -> CellInputVec {
        let cur = self.cursor.table_slice_by_index(3).unwrap();
        cur.into()
    }
}

impl RawTransaction {
    pub fn outputs(&self) -> CellOutputVec {
        let cur = self.cursor.table_slice_by_index(4).unwrap();
        cur.into()
    }
}

impl RawTransaction {
    pub fn outputs_data(&self) -> BytesVec {
        let cur = self.cursor.table_slice_by_index(5).unwrap();
        cur.into()
    }
}

pub struct Transaction {
    cursor: Cursor,
}

impl From<Cursor> for Transaction {
    fn from(cursor: Cursor) -> Self {
        Transaction { cursor }
    }
}
impl Transaction {
    pub fn raw(&self) -> RawTransaction {
        let cur = self.cursor.table_slice_by_index(0).unwrap();
        cur.into()
    }
}

impl Transaction {
    pub fn witnesses(&self) -> BytesVec {
        let cur = self.cursor.table_slice_by_index(1).unwrap();
        cur.into()
    }
}

pub struct RawHeader {
    cursor: Cursor,
}

impl From<Cursor> for RawHeader {
    fn from(cursor: Cursor) -> Self {
        RawHeader { cursor }
    }
}
impl RawHeader {
    pub fn version(&self) -> u32 {
        let cur = self.cursor.slice_by_offset(0, 4).unwrap();
        cur.into()
    }
}
impl RawHeader {
    pub fn compact_target(&self) -> u32 {
        let cur = self.cursor.slice_by_offset(4, 4).unwrap();
        cur.into()
    }
}
impl RawHeader {
    pub fn timestamp(&self) -> u64 {
        let cur = self.cursor.slice_by_offset(8, 8).unwrap();
        cur.into()
    }
}
impl RawHeader {
    pub fn number(&self) -> u64 {
        let cur = self.cursor.slice_by_offset(16, 8).unwrap();
        cur.into()
    }
}
impl RawHeader {
    pub fn epoch(&self) -> u64 {
        let cur = self.cursor.slice_by_offset(24, 8).unwrap();
        cur.into()
    }
}
impl RawHeader {
    pub fn parent_hash(&self) -> Vec<u8> {
        let cur = self.cursor.slice_by_offset(32, 32).unwrap();
        cur.into()
    }
}
impl RawHeader {
    pub fn transactions_root(&self) -> Vec<u8> {
        let cur = self.cursor.slice_by_offset(64, 32).unwrap();
        cur.into()
    }
}
impl RawHeader {
    pub fn proposals_hash(&self) -> Vec<u8> {
        let cur = self.cursor.slice_by_offset(96, 32).unwrap();
        cur.into()
    }
}
impl RawHeader {
    pub fn uncles_hash(&self) -> Vec<u8> {
        let cur = self.cursor.slice_by_offset(128, 32).unwrap();
        cur.into()
    }
}
impl RawHeader {
    pub fn dao(&self) -> Vec<u8> {
        let cur = self.cursor.slice_by_offset(160, 32).unwrap();
        cur.into()
    }
}
pub struct Header {
    cursor: Cursor,
}

impl From<Cursor> for Header {
    fn from(cursor: Cursor) -> Self {
        Header { cursor }
    }
}
impl Header {
    pub fn raw(&self) -> RawHeader {
        let cur = self.cursor.slice_by_offset(0, 192).unwrap();
        cur.into()
    }
}

impl Header {
    pub fn nonce(&self) -> Vec<u8> {
        let cur = self.cursor.slice_by_offset(192, 16).unwrap();
        cur.into()
    }
}
pub struct UncleBlock {
    cursor: Cursor,
}

impl From<Cursor> for UncleBlock {
    fn from(cursor: Cursor) -> Self {
        UncleBlock { cursor }
    }
}
impl UncleBlock {
    pub fn header(&self) -> Header {
        let cur = self.cursor.table_slice_by_index(0).unwrap();
        cur.into()
    }
}

impl UncleBlock {
    pub fn proposals(&self) -> ProposalShortIdVec {
        let cur = self.cursor.table_slice_by_index(1).unwrap();
        cur.into()
    }
}

pub struct Block {
    cursor: Cursor,
}

impl From<Cursor> for Block {
    fn from(cursor: Cursor) -> Self {
        Block { cursor }
    }
}
impl Block {
    pub fn header(&self) -> Header {
        let cur = self.cursor.table_slice_by_index(0).unwrap();
        cur.into()
    }
}

impl Block {
    pub fn uncles(&self) -> UncleBlockVec {
        let cur = self.cursor.table_slice_by_index(1).unwrap();
        cur.into()
    }
}

impl Block {
    pub fn transactions(&self) -> TransactionVec {
        let cur = self.cursor.table_slice_by_index(2).unwrap();
        cur.into()
    }
}

impl Block {
    pub fn proposals(&self) -> ProposalShortIdVec {
        let cur = self.cursor.table_slice_by_index(3).unwrap();
        cur.into()
    }
}

pub struct CellbaseWitness {
    cursor: Cursor,
}

impl From<Cursor> for CellbaseWitness {
    fn from(cursor: Cursor) -> Self {
        CellbaseWitness { cursor }
    }
}
impl CellbaseWitness {
    pub fn lock(&self) -> Script {
        let cur = self.cursor.table_slice_by_index(0).unwrap();
        cur.into()
    }
}

impl CellbaseWitness {
    pub fn message(&self) -> Vec<u8> {
        let cur = self.cursor.table_slice_by_index(1).unwrap();
        let cur2 = cur.convert_to_rawbytes().unwrap();
        cur2.into()
    }
}
pub struct WitnessArgs {
    cursor: Cursor,
}

impl From<Cursor> for WitnessArgs {
    fn from(cursor: Cursor) -> Self {
        WitnessArgs { cursor }
    }
}
impl WitnessArgs {
    pub fn lock(&self) -> Option<Vec<u8>> {
        let cur = self.cursor.table_slice_by_index(0).unwrap();
        if cur.option_is_none() {
            None
        } else {
            Some(cur.into())
        }
    }
}

impl WitnessArgs {
    pub fn input_type(&self) -> Option<Vec<u8>> {
        let cur = self.cursor.table_slice_by_index(1).unwrap();
        if cur.option_is_none() {
            None
        } else {
            Some(cur.into())
        }
    }
}

impl WitnessArgs {
    pub fn output_type(&self) -> Option<Vec<u8>> {
        let cur = self.cursor.table_slice_by_index(2).unwrap();
        if cur.option_is_none() {
            None
        } else {
            Some(cur.into())
        }
    }
}