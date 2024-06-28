#![feature(test)]

extern crate test;
use test::Bencher;
use simple_rust_example;

#[bench]
fn bench_add(b: &mut Bencher) {
    b.iter(|| simple_rust_example::add(2, 2));
}
