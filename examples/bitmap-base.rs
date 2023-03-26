extern crate cbitmap;
use cbitmap::bitmap::*;

fn main() {
    let mut bitmap: Bitmap<3> = [0; 3].into();
    println!("Init bitmap: \n{:#?}\n", &bitmap);

    bitmap.set(3);
    println!("Set at 3: \n{:#?}", bitmap);
    println!("Format: {}\n", bitmap);
    println!("Get bool at 2: {}", bitmap.get_bool(2));
    bitmap.set(20);
    println!("Set 20. Get 0/1 at 20: {}\n", &bitmap.get_01(20));
    println!("[7..21]: {}", bitmap.range_to_string(7, 21).unwrap());
    println!("[0..12]: {}", bitmap.range_to_string(0, 12).unwrap());
    println!("[0..24]: {}", bitmap.range_to_string(0, 24).unwrap());

    let bit1 = bitmap.at(1);
    println!("\nIndexing 1, deref: {}", *bit1);

    {
        let mut bitm = bitmap.at_mut(5);
        println!("Mutable indexing 5, deref: {}", *bitm);
        bitm.set();
        println!("Use set, deref: {}", *bitm);
    }

    println!("After drop, test 5: {}\n", &bitmap.get_bool(5));

    bitmap.reset_all();

    print!("Reset all, ");
    println!("[0..24]: {}", bitmap.range_to_string(0, 24).unwrap());

    bitmap |= 0b101001000u16;
    print!("\n|= 1_01001000, ");
    println!("[0..16]: {}", bitmap.range_to_string(0, 16).unwrap());
    println!("bitmap & 01000000 = {:08b}", &bitmap & 0b01000000u8);

    bitmap.set_all();
    print!("\nSet all, ");
    println!("[0..24]: {}", bitmap.range_to_string(0, 24).unwrap());
}
