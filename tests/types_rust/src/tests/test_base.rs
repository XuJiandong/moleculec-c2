#![allow(unused_imports)]
// #![allow(dead_code)]

use crate::types_api;
use crate::types_api2;

use core::convert::TryInto;
use lazy_static::lazy_static;
use molecule::prelude::{Builder, Entity, Reader};
use molecule::{bytes::Bytes, prelude::Byte};
use rand::{random, rngs::ThreadRng, thread_rng, Rng, RngCore};

use super::*;

#[derive(Default)]
struct TestDataAll {
    f0: TypesArray<u8, 1>,
    f1: TypesArray<u8, 2>,
    f2: TypesArray<u8, 3>,
    f3: TypesArray<u8, 4>,
    f4: TypesArray<u8, 5>,
    f5: TypesArray<u8, 6>,
    f6: TypesArray<u8, 7>,
    f7: TypesArray<u8, 8>,
    f8: TypesArray<u8, 9>,
    f9: TypesArray<u8, 10>,
    f10: TypesArray<u8, 11>,
    f11: TypesArray<u8, 12>,
    f12: TypesArray<u8, 13>,
    f13: TypesArray<u8, 14>,
    f14: TypesArray<u8, 15>,
    f15: TypesArray<u8, 16>,

    f16: TypesArrayWord,
    f17: TypesArray<TypesArrayWord, 2>,
    f18: TypesArray<TypesArrayWord, 3>,
    f19: TypesArray<TypesArrayWord, 4>,
    f20: TypesArray<TypesArrayWord, 5>,
    f21: TypesArray<TypesArrayWord, 6>,
    f22: TypesArray<TypesArrayWord, 7>,
    f23: TypesArray<TypesArrayWord, 8>,

    f24: TypesArray<TypesArray<u8, 3>, 3>,
    f25: TypesArray<TypesArray<u8, 5>, 3>,
    f26: TypesArray<TypesArray<u8, 7>, 3>,
    f27: TypesArray<TypesArray<u8, 9>, 3>,

    f28: TypesStructA,
    f29: TypesStructB,
    f30: TypesStructC,
    f31: TypesStructD,
    f32: TypesStructE,
    f33: TypesStructF,
    f34: TypesStructG,
    f35: TypesStructH,
    f36: TypesStructI,
    f37: TypesStructJ,
    f38: TypesStructIx3,
    f39: TypesStructO,
    f40: TypesStructP,

    f41: TypesVec<u8>,
    f42: TypesVec<TypesArrayWord>,
    f43: TypesVec<TypesArray<u8, 3>>,
    f44: TypesVec<TypesArray<u8, 7>>,
    f45: TypesVec<TypesStructI>,
    f46: TypesVec<TypesStructJ>,
    f47: TypesVec<TypesStructP>,
    f48: TypesVec<TypesVec<u8>>,
    f49: TypesVec<TypesVec<TypesArrayWord>>,

    f50: TypesTable0,
    f51: TypesTable1,
    f52: TypesTable2,
    f53: TypesTable3,
    f54: TypesTable4,
    f55: TypesTable5,
    f56: TypesTable6,

    f57: TypesOption<u8>,
    f58: TypesOption<TypesArrayWord>,
    f59: TypesOption<TypesStructA>,
    f60: TypesOption<TypesStructP>,
    f61: TypesOption<TypesVec<u8>>,
    f62: TypesOption<TypesVec<TypesArrayWord>>,
    f63: TypesOption<TypesVec<TypesVec<u8>>>,
    f64: TypesOption<TypesVec<TypesVec<TypesArrayWord>>>,
    f65: TypesOption<TypesTable0>,
    f66: TypesOption<TypesTable6>,
    f67: TypesOption<TypesOption<TypesTable6>>,

    f68: TypesVec<TypesOption<u8>>,
    f69: TypesVec<TypesOption<TypesArrayWord>>,
    f70: TypesVec<TypesOption<TypesVec<TypesArrayWord>>>,
    f71: TypesVec<TypesOption<TypesVec<u8>>>,

    f72: TypesUnionA,
    f73: TypesTableA,
}

impl TestDataAll {
    pub fn to_bytes(&self) -> Vec<u8> {
        use crate::types_api::*;

        let builder = types_api::AllInOneBuilder::default()
            .f0(self.f0.to_mol())
            .f1(self.f1.to_mol())
            .f2(self.f2.to_mol())
            .f3(self.f3.to_mol())
            .f4(self.f4.to_mol())
            .f5(self.f5.to_mol())
            .f6(self.f6.to_mol())
            .f7(self.f7.to_mol())
            .f8(self.f8.to_mol())
            .f9(self.f9.to_mol())
            .f10(self.f10.to_mol())
            .f11(self.f11.to_mol())
            .f12(self.f12.to_mol())
            .f13(self.f13.to_mol())
            .f14(self.f14.to_mol())
            .f15(self.f15.to_mol())
            .f16(
                types_api::Word::new_builder()
                    .set(self.f16.d.map(|f| f.into()))
                    .build(),
            )
            .f17(self.f17.to_mol())
            .f18(self.f18.to_mol())
            .f19(self.f19.to_mol())
            .f20(self.f20.to_mol())
            .f21(self.f21.to_mol())
            .f22(self.f22.to_mol())
            .f23(self.f23.to_mol())
            .f24(self.f24.to_mol())
            .f25(self.f25.to_mol())
            .f26(self.f26.to_mol())
            .f27(self.f27.to_mol())
            .f28(self.f28.to_mol())
            .f29(self.f29.to_mol())
            .f30(self.f30.to_mol())
            .f31(self.f31.to_mol())
            .f32(self.f32.to_mol())
            .f33(self.f33.to_mol())
            .f34(self.f34.to_mol())
            .f35(self.f35.to_mol())
            .f36(self.f36.to_mol())
            .f37(self.f37.to_mol())
            .f38(self.f38.to_mol())
            .f39(self.f39.to_mol())
            .f40(self.f40.to_mol())
            .f41(self.f41.to_mol())
            .f42(self.f42.to_mol())
            .f43(self.f43.to_mol())
            .f44(self.f44.to_mol())
            .f45(self.f45.to_mol())
            .f46(self.f46.to_mol())
            .f47(self.f47.to_mol())
            .f48(self.f48.to_mol())
            .f49(self.f49.to_mol())
            .f50(self.f50.to_mol())
            .f51(self.f51.to_mol())
            .f52(self.f52.to_mol())
            .f53(self.f53.to_mol())
            .f54(self.f54.to_mol())
            .f55(self.f55.to_mol())
            .f56(self.f56.to_mol())
            .f57(self.f57.to_mol())
            .f58(self.f58.to_mol())
            .f59(self.f59.to_mol())
            .f60(self.f60.to_mol())
            .f61(self.f61.to_mol())
            .f62(self.f62.to_mol())
            .f63(self.f63.to_mol())
            .f64(self.f64.to_mol())
            .f65(self.f65.to_mol())
            .f66(self.f66.to_mol())
            .f67(self.f67.to_mol())
            .f68(self.f68.to_mol())
            .f69(self.f69.to_mol())
            .f70(self.f70.to_mol())
            .f71(self.f71.to_mol())
            .f72(self.f72.to_mol())
            .f73(self.f73.to_mol())
            .build();

        builder.as_reader().as_slice().to_vec()
    }
}

// #[test]
pub fn test_base() {
    use crate::tests::types_api2_mol2::Mol2Vec;
    use molecule2::Cursor;
    use types_api2::*;

    let test_data = TestDataAll::default();
    let buffer = test_data.to_bytes();
    // println!("dbg: buf : \n{:02x?}", buffer);

    let cursor = molecule2::Cursor::new(buffer.len(), Box::new(buffer));
    let all_in_one = AllInOne { cursor };

    test_data.f0.check(&all_in_one.f0()).expect("f0");
    test_data.f1.check(&all_in_one.f1().into()).expect("f1");
    test_data.f2.check(&all_in_one.f2().into()).expect("f2");
    test_data.f3.check(&all_in_one.f3().into()).expect("f3");
    test_data.f4.check(&all_in_one.f4().into()).expect("f4");
    test_data.f5.check(&all_in_one.f5().into()).expect("f5");
    test_data.f6.check(&all_in_one.f6().into()).expect("f6");
    test_data.f7.check(&all_in_one.f7().into()).expect("f7");
    test_data.f8.check(&all_in_one.f8().into()).expect("f8");
    test_data.f9.check(&all_in_one.f9().into()).expect("f9");
    test_data.f10.check(&all_in_one.f10().into()).expect("f10");
    test_data.f11.check(&all_in_one.f11().into()).expect("f11");
    test_data.f12.check(&all_in_one.f12().into()).expect("f12");
    test_data.f13.check(&all_in_one.f13().into()).expect("f13");
    test_data.f14.check(&all_in_one.f14().into()).expect("f14");
    test_data.f15.check(&all_in_one.f15().into()).expect("f15");

    test_data.f16.check2(&all_in_one.f16().into()).expect("f16");
    test_data.f17.check(&all_in_one.f17().into()).expect("f17");
    test_data.f18.check(&all_in_one.f18().into()).expect("f18");
    test_data.f19.check(&all_in_one.f19().into()).expect("f19");
    test_data.f20.check(&all_in_one.f20().into()).expect("f20");
    test_data.f21.check(&all_in_one.f21().into()).expect("f21");
    test_data.f22.check(&all_in_one.f22().into()).expect("f22");
    test_data.f23.check(&all_in_one.f23().into()).expect("f23");

    test_data.f24.check(&all_in_one.f24().into()).expect("f24");
    test_data.f25.check(&all_in_one.f25().into()).expect("f25");
    test_data.f26.check(&all_in_one.f26().into()).expect("f26");
    test_data.f27.check(&all_in_one.f27().into()).expect("f27");

    test_data.f28.check(&all_in_one.f28().into()).expect("f28");
    test_data.f29.check(&all_in_one.f29().into()).expect("f29");
    test_data.f30.check(&all_in_one.f30().into()).expect("f30");
    test_data.f31.check(&all_in_one.f31().into()).expect("f31");
    test_data.f32.check(&all_in_one.f32().into()).expect("f32");
    test_data.f33.check(&all_in_one.f33().into()).expect("f33");
    test_data.f34.check(&all_in_one.f34().into()).expect("f34");
    test_data.f35.check(&all_in_one.f35().into()).expect("f35");
    test_data.f36.check(&all_in_one.f36().into()).expect("f36");
    test_data.f37.check(&all_in_one.f37().into()).expect("f37");
    test_data.f38.check(&all_in_one.f38().into()).expect("f38");
    test_data.f39.check(&all_in_one.f39().into()).expect("f39");
    test_data.f40.check(&all_in_one.f40().into()).expect("f40");

    test_data.f41.check(&all_in_one.f41().into()).expect("f41");
    test_data.f42.check(&all_in_one.f42().into()).expect("f42");
    test_data.f43.check(&all_in_one.f43().into()).expect("f43");
    test_data.f44.check(&all_in_one.f44().into()).expect("f44");
    test_data.f45.check(&all_in_one.f45().into()).expect("f45");
    test_data.f46.check(&all_in_one.f46().into()).expect("f46");
    test_data.f47.check(&all_in_one.f47().into()).expect("f47");
    test_data.f48.check(&all_in_one.f48().into()).expect("f48");
    test_data.f49.check(&all_in_one.f49().into()).expect("f49");

    test_data.f50.check(&all_in_one.f50().into()).expect("f50");
    test_data.f51.check(&all_in_one.f51().into()).expect("f51");
    test_data.f52.check(&all_in_one.f52().into()).expect("f52");
    test_data.f53.check(&all_in_one.f53().into()).expect("f53");
    test_data.f54.check(&all_in_one.f54().into()).expect("f54");
    test_data.f55.check(&all_in_one.f55().into()).expect("f55");
    test_data.f56.check(&all_in_one.f56().into()).expect("f56");

    test_data.f57.check(&all_in_one.f57().into()).expect("f57");
    test_data
        .f58
        .check(&all_in_one.f58().map(|f| f.into()))
        .expect("f58");
    test_data.f59.check(&all_in_one.f59()).expect("f59");
    test_data.f60.check(&all_in_one.f60()).expect("f60");
    test_data.f61.check(&all_in_one.f61()).expect("f61");
    test_data.f62.check(&all_in_one.f62()).expect("f62");
    test_data.f63.check(&all_in_one.f63()).expect("f63");
    test_data.f64.check(&all_in_one.f64()).expect("f64");
    test_data.f65.check(&all_in_one.f65()).expect("f65");
    test_data.f66.check(&all_in_one.f66()).expect("f66");
    // test_data.f67.check(&all_in_one.f67()).expect("f67"); // TODO Some(None) == None?
    test_data.f68.check(&all_in_one.f68()).expect("f68");
    test_data.f69.check(&all_in_one.f69()).expect("f69");
    test_data.f70.check(&all_in_one.f70()).expect("f70");
    // test_data.f71.check(&all_in_one.f71()).expect("f71"); // TODO Bytes get len
    test_data.f72.check(&all_in_one.f72()).expect("f72");
    test_data.f73.check(&all_in_one.f73()).expect("f73");
}
