use criterion::{black_box, criterion_group, criterion_main, Criterion};

extern crate cbitmap;
use cbitmap::bitmap::*;

macro_rules! simpleb {
    ($t:expr) => {
        |b| b.iter($t)
    };
}

pub fn bench_create(c: &mut Criterion) {
    let mut g = c.benchmark_group("create new");
    g.bench_function("1B", simpleb!(|| Bitmap::<1>::new()));
    g.bench_function("16B", simpleb!(|| Bitmap::<128>::new()));
    g.finish();

    let mut g = c.benchmark_group("create from");
    g.bench_function("1B", simpleb!(|| Bitmap::<1>::from(4u8)));
    g.bench_function("16B", simpleb!(|| Bitmap::<128>::from(1237894213u128)));
    g.finish();

    let mut g = c.benchmark_group("create macro");
    g.bench_function("newmap 128b", simpleb!(|| newmap!(;128)));
    g.bench_function("newmap 122b", simpleb!(|| newmap!(;122)));
    g.bench_function("newmap 128b init 5", simpleb!(|| newmap!(1|2|4|8|16; 128)));
    g.bench_function(
        "he_lang 128b init 5",
        simpleb!(|| he_lang!(0|14|65|90|111; 128)),
    );
    g.finish();

    let mut g = c.benchmark_group("create box");
    g.bench_function("4KB", simpleb!(|| Box::new(Bitmap::<4096>::new())));
    g.bench_function(
        "512KB",
        simpleb!(|| Box::new(Bitmap::<{ 512 * 1024 * 8 }>::new())),
    );
    g.bench_function(
        "2MB",
        simpleb!(|| Box::new(Bitmap::<{ 2 * 1024 * 1024 * 8 }>::new())),
    );
    g.finish();
}

fn bench_set(c: &mut Criterion) {
    let mut map = newmap!(;128);
    let mut g = c.benchmark_group("set-flip");
    g.bench_function(
        "set 1",
        simpleb!(|| {
            let _ = &mut map.set(black_box(0));
        }),
    );
    g.bench_function(
        "set chain 2",
        simpleb!(|| {
            let _ = &mut map.set(black_box(0)).set(black_box(1));
        }),
    );
    g.bench_function(
        "set all 128",
        simpleb!(|| {
            let _ = &mut map.set_all();
        }),
    );
    g.bench_function(
        "flip 1",
        simpleb!(|| {
            let _ = &mut map.flip(black_box(0));
        }),
    );
    g.bench_function(
        "flip chain 2",
        simpleb!(|| {
            let _ = &mut map.flip(black_box(0)).flip(black_box(1));
        }),
    );
    g.bench_function(
        "flip all 128",
        simpleb!(|| {
            let _ = &mut map.flip_all();
        }),
    );
    g.finish();
}

fn bench_find(c: &mut Criterion) {
    let mut map = newmap!(;128);
    let mut g = c.benchmark_group("find first");
    map.set(0);
    g.bench_function(
        "at begin",
        simpleb!(|| {
            let _ = &mut map.find_first_one();
        }),
    );
    map.reset(0);
    map.set(64);
    g.bench_function(
        "at middle",
        simpleb!(|| {
            let _ = &mut map.find_first_one();
        }),
    );
    map.reset(64);
    g.bench_function(
        "all 0 (128)",
        simpleb!(|| {
            let _ = &mut map.find_first_one();
        }),
    );
    g.finish();
}

fn bench_bitref(c: &mut Criterion) {
    let mut map = newmap!(;128);
    let mut g = c.benchmark_group("bitref");
    g.bench_function(
        "at-deref",
        simpleb!(|| {
            let b = &map.at(black_box(64));
            let _ = *b;
        }),
    );
    g.bench_function(
        "at-mut-deref",
        simpleb!(|| {
            let b = &mut map.at_mut(black_box(64));
            let _ = *b;
        }),
    );
    g.bench_function(
        "at-mut-set",
        simpleb!(|| {
            let b = &mut map.at_mut(black_box(64));
            b.set();
        }),
    );
    g.finish();
}

fn bench_op(c: &mut Criterion) {
    let mut map = newmap!(;128);
    let mut g = c.benchmark_group("ops");
    g.bench_function(
        "and u128",
        simpleb!(|| {
            let _ = &map & black_box(12348u128);
        }),
    );
    g.bench_function(
        "and= u128",
        simpleb!(|| {
            let _ = map &= black_box(12348u128);
        }),
    );
    g.bench_function(
        "or= u128",
        simpleb!(|| {
            let _ = map |= black_box(12348u128);
        }),
    );
    g.finish();
}

fn bench_fill(c: &mut Criterion) {
    let mut map = newmap!(;128);
    let mut g = c.benchmark_group("fill");
    let arr1 = [255u8; 4];
    g.bench_function(
        "prefix 32",
        simpleb!(|| {
            let _ = map.fill_prefix(black_box(arr1));
        }),
    );
    let arr2 = [255u8; 8];
    g.bench_function(
        "prefix 64",
        simpleb!(|| {
            let _ = map.fill_prefix(black_box(arr2));
        }),
    );
    let arr3 = [255u8; 16];
    g.bench_function(
        "prefix 128",
        simpleb!(|| {
            let _ = map.fill_prefix(black_box(arr3));
        }),
    );
    g.finish();
}

criterion_group!(benches, 
  bench_create, 
  bench_set, 
  bench_find, 
  bench_bitref, 
  bench_op, 
  bench_fill
);
criterion_main!(benches);
