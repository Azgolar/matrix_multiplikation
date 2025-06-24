use criterion::{Criterion, criterion_group, criterion_main};

pub fn single(einstellungen: &mut Criterion) {
}

pub fn manuell(einstellungen: &mut Criterion) {
}

pub fn unroll(einstellungen: &mut Criterion) {
}

pub fn tiling(einstellungen: &mut Criterion) {
}

pub fn simd(einstellungen: &mut Criterion) {
}

pub fn rayon(einstellungen: &mut Criterion) {
}

pub fn crossbeam(einstellungen: &mut Criterion) {
}

criterion_group!(benchmark, single, manuell, unroll, tiling, simd, rayon, crossbeam);

criterion_main!(benchmark);