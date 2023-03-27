use criterion::{criterion_group, criterion_main, Criterion};

extern crate cbitmap;
use cbitmap::bitmap::*;

macro_rules! simpleb {
    ($t:expr) => {
        |b| b.iter($t)
    };
}

pub fn bench_create(c: &mut Criterion) {
    c.bench_function("new 1B", simpleb!(|| Bitmap::<1>::new()));
    c.bench_function("new 16B", simpleb!(|| Bitmap::<128>::new()));

    c.bench_function("from 1B", simpleb!(|| Bitmap::<1>::from(4u8)));
    c.bench_function("from 16B", simpleb!(|| Bitmap::<128>::from(1237894213u128)));

    c.bench_function("newmap 128b", simpleb!(|| newmap!(;128)));
    c.bench_function("newmap 100b", simpleb!(|| newmap!(;100)));
    c.bench_function("newmap 128b init 5", simpleb!(|| newmap!(1|2|4|8|16; 128)));

    c.bench_function("he_lang 128b init 5", simpleb!(|| he_lang!(0|14|65|90|111; 128)));

    c.bench_function("box 4KB", simpleb!(|| Box::new(Bitmap::<4096>::new())));
    c.bench_function("box 2MB", simpleb!(|| Box::new(Bitmap::<{2*1024*1024*8}>::new())));
}

criterion_group!(benches, bench_create);
criterion_main!(benches);
