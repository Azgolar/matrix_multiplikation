use criterion::{Criterion, criterion_group, criterion_main, BenchmarkId};
use std::{hint::black_box, time::Duration};
use core_affinity::get_core_ids;

use multiplikation::matrix::zufallsmatrix_2d;
use multiplikation::algorithmen::single;
use multiplikation::algorithmen::manuell_sicher;

/*
    Einstellungen für alle Benchmarks
    globale Variablen müssen const oder static sein
*/
const ANZAHL: usize = 10;       // Anzahl der Durchläufe
const ZEIT: u64 = 60;           // maximale Zeit in Sekunden je Durchlauf
const MATRIZEN: &[usize] = &[4, 8, 11, 16, 25, 32, 64, 94, 128, 256, 357, 512, 787, 1024, 1667];    // Matrixgrößen

pub fn run_single(einstellungen: &mut Criterion) {
    let mut gruppe: criterion::BenchmarkGroup<'_, criterion::measurement::WallTime> = einstellungen.benchmark_group("single thread");
    
    // Benchmark Einstellungen
    gruppe.sample_size(ANZAHL);
    gruppe.measurement_time(Duration::from_secs(ZEIT));

    // Kern für cpu pinning
    let kerne: Vec<core_affinity::CoreId> = get_core_ids().unwrap();
    let kern: &core_affinity::CoreId = &kerne[0];

    for &n in MATRIZEN {
        gruppe.bench_with_input(BenchmarkId::from_parameter("single"), &n, |messen, &n| {
            // Matrizen initialisieren
            let a: Vec<Vec<f64>> = zufallsmatrix_2d(n);
            let b: Vec<Vec<f64>> = zufallsmatrix_2d(n);
            let mut c: Vec<Vec<f64>> = vec![vec![0.0; n]; n];

            // Benchmark ausführen
            messen.iter(|| {
                single::ausführen(&a, &b, &mut c, n, kern);
                black_box(&c);
            });
        });
    }

    // Benchmark abschließen und Statistiken erstellen
    gruppe.finish();
}

pub fn run_manuell(einstellungen: &mut Criterion) {
}

pub fn run_unroll(einstellungen: &mut Criterion) {
}

pub fn run_tiling(einstellungen: &mut Criterion) {
}

pub fn run_simd(einstellungen: &mut Criterion) {
}

pub fn run_simd_tiling(einstellungen: &mut Criterion) {
}


pub fn run_rayon(einstellungen: &mut Criterion) {
}

pub fn run_crossbeam(einstellungen: &mut Criterion) {
}

// Einzelne Benchmarks definieren
criterion_group!(
    name = single;
    config = Criterion::default();
    targets = run_single
);

criterion_group!(
    name = manuell;
    config = Criterion::default();
    targets = run_manuell
);

criterion_group!(
    name = unroll;
    config = Criterion::default();
    targets = run_unroll
);

criterion_group!(
    name = tiling;
    config = Criterion::default();
    targets = run_tiling
);

criterion_group!(
    name = simd;
    config = Criterion::default();
    targets = run_simd
);

criterion_group!(
    name = simd_tiling;
    config = Criterion::default();
    targets = run_simd_tiling
);

criterion_group!(
    name = rayon;
    config = Criterion::default();
    targets = run_rayon
);

criterion_group!(
    name = crossbeam;
    config = Criterion::default();
    targets = run_crossbeam
);

criterion_main!(single);