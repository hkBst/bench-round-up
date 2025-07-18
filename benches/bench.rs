#![feature(test)]

extern crate test;

use bench_round_up::*;

const LEN: usize = 1000_000;

#[bench]
fn bench_round_up_iter(b: &mut test::Bencher) {
    b.iter(|| {
        assert_eq!(round_up_iter(test::black_box(&mut [b'9'; LEN])), Some(b'0'));
    });
}

#[bench]
fn bench_round_up_for(b: &mut test::Bencher) {
    b.iter(|| {
        assert_eq!(round_up_for(test::black_box(&mut [b'9'; LEN])), Some(b'0'));
    });
}
