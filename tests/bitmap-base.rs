extern crate cbitmap;
use cbitmap::bitmap::*;

#[test]
fn base_op() {
    let mut bitmap: Bitmap<2> = Default::default();

    assert_eq!(bitmap.get_bool(0), false);
    assert_eq!(bitmap.get_01(4), 0);

    bitmap.set(7);
    bitmap.set(8);

    assert_eq!(bitmap.get_bool(7), true);
    assert_eq!(bitmap.get_01(8), 1);

    bitmap.reset(7);

    assert_eq!(bitmap.get_bool(7), false);

    bitmap.flip(8);

    assert_eq!(bitmap.get_bool(8), false);

    bitmap.flip(9);
    assert_eq!(bitmap.get_bool(9), true);

    bitmap.set_all();
    assert_eq!(&bitmap.range_to_string(0, 16).unwrap(), "11111111 11111111");

    bitmap.reset_all();
    assert_eq!(&bitmap.range_to_string(0, 16).unwrap(), "00000000 00000000");

    // 00010000 00100010
    bitmap.set(1).set(5).set(12);
    assert_eq!(bitmap.range_to_string(0, 0), None);
    assert_eq!(&bitmap.range_to_string(5, 6).unwrap(), "1");
    assert_eq!(&bitmap.range_to_string(7, 8).unwrap(), "0");
    assert_eq!(&bitmap.range_to_string(7, 9).unwrap(), "0 0");

    bitmap.flip_all();
    assert_eq!(&bitmap.range_to_string(0, 16).unwrap(), "11101111 11011101");
    
}

#[test]
fn from_into() {
    let map0 = Bitmap::<0>::from([]);
    assert_eq!(Into::<[u8; 0]>::into(map0), []);
    let map1 = Bitmap::<1>::from(0b00001010);
    assert_eq!(Into::<[u8; 1]>::into(map1), [0b00001010u8; 1]);
    let map2 = Bitmap::<2>::from(0b00001111_00001010);
    assert_eq!(Into::<[u8; 2]>::into(map2), [0b00001010u8, 0b00001111]);
    let u = 0u8;
    let mut map3: Bitmap<4> = [u, u, u, u].into();
    map3.flip_all();
    assert_eq!(Into::<[u8; 4]>::into(map3), [255; 4]);
}

#[test]
fn ref_refmut() {
    let mut bitmap = Bitmap::<2>::new();
    {
        let mut bitmut = bitmap.at_mut(7);

        assert_eq!(*bitmut, false);
        bitmut.set();
        assert_eq!(*bitmut, true);
        assert_eq!(Into::<bool>::into(bitmut), true);
    }

    assert_eq!(bitmap.get_bool(7), true);

    {
        let bit = bitmap.at(7);

        assert_eq!(*bit, true);
        assert_eq!(Into::<bool>::into(bit), true);
    }
    {
        let mut bitmut = bitmap.at_mut(7);

        assert_eq!(*bitmut, true);
        bitmut.reset();
        assert_eq!(*bitmut, false);
        assert_eq!(Into::<bool>::into(bitmut), false);
    }
    {
        let bit = bitmap.at(7);

        assert_eq!(*bit, false);
        assert_eq!(Into::<bool>::into(bit), false);
    }
    {
        let mut bitmut = bitmap.at_mut(7);
        bitmut.flip();
        assert_eq!(*bitmut, true);
    }

    assert_eq!(bitmap.get_bool(7), true);

    {
        let mut bitmut = bitmap.at_mut(4);
        bitmut.reset().flip();
    }

    assert_eq!(bitmap.get_bool(4), true);
}

#[test]
fn and_or() {
    let mut bitmap = Bitmap::<16>::new();

    // ...10100010
    bitmap |= 0b_10100010_u8;

    assert_eq!(bitmap.get_01(0), 0);
    assert_eq!(bitmap.get_01(1), 1);
    assert_eq!(bitmap.get_01(2), 0);
    assert_eq!(bitmap.get_01(3), 0);
    assert_eq!(bitmap.get_01(4), 0);
    assert_eq!(bitmap.get_01(5), 1);
    assert_eq!(bitmap.get_01(6), 0);
    assert_eq!(bitmap.get_01(7), 1);

    bitmap |= (0b_11000000 << 8) as u16;

    assert_eq!(bitmap.get_01(14), 1);
    assert_eq!(bitmap.get_01(15), 1);

    bitmap |= (0b_00110000_00000011 << 16) as u32;

    assert_eq!(bitmap.get_01(16), 1);
    assert_eq!(bitmap.get_01(17), 1);
    assert_eq!(bitmap.get_01(28), 1);
    assert_eq!(bitmap.get_01(29), 1);

    bitmap |= 1u64 << 32;

    assert_eq!(bitmap.get_01(32), 1);

    bitmap |= 1u128 << 100;

    assert_eq!(bitmap.get_01(100), 1);

    assert_eq!(&bitmap & (1u128 << 100), 1u128 << 100);
    assert_eq!(&bitmap & (1u128 << 99), 0);
    assert_eq!(
        &bitmap & (0b_10001000_11000010_u16),
        0b_10000000_10000010_u16
    );

    bitmap |= [0b_10101010_u8; 15];

    assert_eq!(&bitmap & [0b_10001000_u8; 10], [0b_10001000_u8; 10]);

    bitmap.set(127);
    bitmap &= [0b_10000000_u8; 15];

    assert_eq!(&bitmap & [0b_11111111_u8; 15], [0b_10000000_u8; 15]);
    assert_eq!(bitmap.get_bool(127), false);

    bitmap.set_all();

    bitmap &= u128::MAX;
    assert_eq!(bitmap.get_bool(127), true);
    bitmap &= u64::MAX;
    assert_eq!(bitmap.get_bool(127), false);
    assert_eq!(bitmap.get_bool(63), true);
    bitmap &= u32::MAX;
    assert_eq!(bitmap.get_bool(63), false);
    assert_eq!(bitmap.get_bool(31), true);
    bitmap &= u16::MAX;
    assert_eq!(bitmap.get_bool(31), false);
    assert_eq!(bitmap.get_bool(15), true);
    bitmap &= u8::MAX;
    assert_eq!(bitmap.get_bool(15), false);
    assert_eq!(bitmap.get_bool(7), true);
}

#[test]
#[should_panic]
fn at_out_of_range() {
    let _ = Bitmap::<1>::new().at(8);
}

#[test]
#[should_panic]
fn at_mut_out_of_range() {
    let _ = (&mut Bitmap::<1>::new()).at_mut(8);
}

#[test]
#[should_panic]
fn set_mut_out_of_range() {
    (&mut Bitmap::<1>::new()).set(8);
}

#[test]
#[should_panic]
fn reset_mut_out_of_range() {
    (&mut Bitmap::<1>::new()).reset(8);
}

#[test]
#[should_panic]
fn flip_mut_out_of_range() {
    let _ = (&mut Bitmap::<1>::new()).flip(8);
}
