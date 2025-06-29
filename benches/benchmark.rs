use criterion::{Criterion, criterion_group, criterion_main, BenchmarkId};
use std::{hint::black_box, time::Duration, process};
use core_affinity::{get_core_ids, set_for_current};
use rayon::ThreadPoolBuilder;

use multiplikation::matrix::zufallsmatrix_2d;
use multiplikation::algorithmen::single;
use multiplikation::algorithmen::manuell_sicher;
use multiplikation::algorithmen::manuell_unsicher;
use multiplikation::algorithmen::unroll;
use multiplikation::algorithmen::tiling;
use multiplikation::algorithmen::simd;
use multiplikation::algorithmen::simd_tiling;
use multiplikation::algorithmen::rayon as mein_rayon;
use multiplikation::algorithmen::crossbeam;



/*
    Einstellungen für alle Benchmarks
    globale Variablen müssen const oder static sein
*/
const ANZAHL: usize = 10;       // Anzahl der Durchläufe
const ZEIT: u64 = 60;           // maximale Zeit in Sekunden je Durchlauf
const MATRIZEN: &[usize] = &[4, 8, 11, 16, 25, 32, 64, 94, 128, 256, 357, 512, 787, 1024, 1667];    // Matrixgrößen

/*
    Single Thread
*/
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

/*
    Threads ohne unsafe
*/
pub fn run_manuell_sicher(einstellungen: &mut Criterion) {
    let mut gruppe: criterion::BenchmarkGroup<'_, criterion::measurement::WallTime> = einstellungen.benchmark_group("Threasd ohne unsafe");
    
    // Benchmark Einstellungen
    gruppe.sample_size(ANZAHL);
    gruppe.measurement_time(Duration::from_secs(ZEIT));

    // Kern für cpu pinning
    let kerne: Vec<core_affinity::CoreId> = get_core_ids().unwrap();

    for &n in MATRIZEN {
        for threads in 2..kerne.len() {
            gruppe.bench_with_input(BenchmarkId::from_parameter("Threads ohne unsafe"), &n, |messen, &n| {
                // Matrizen initialisieren
                let a: Vec<Vec<f64>> = zufallsmatrix_2d(n);
                let b: Vec<Vec<f64>> = zufallsmatrix_2d(n);
                let mut c: Vec<Vec<f64>> = vec![vec![0.0; n]; n];

                // Benchmark ausführen
                messen.iter(|| {
                    manuell_sicher::ausführen(&a, &b, &mut c, n, threads, &kerne);
                    black_box(&c);
                });
            });
        }
    }
    // Benchmark abschließen und Statistiken erstellen
    gruppe.finish();
}

/*
    Threads mit unsafe
*/
pub fn run_manuell_unsicher(einstellungen: &mut Criterion) {
    let mut gruppe: criterion::BenchmarkGroup<'_, criterion::measurement::WallTime> = einstellungen.benchmark_group("Threads mit unsafe");
    
    // Benchmark Einstellungen
    gruppe.sample_size(ANZAHL);
    gruppe.measurement_time(Duration::from_secs(ZEIT));

    // Kern für cpu pinning
    let kerne: Vec<core_affinity::CoreId> = get_core_ids().unwrap();

    for &n in MATRIZEN {
        for threads in 2..kerne.len() {
            gruppe.bench_with_input(BenchmarkId::from_parameter("Threads mit unsafe"), &n, |messen, &n| {
                // Matrizen initialisieren
                let a: Vec<Vec<f64>> = zufallsmatrix_2d(n);
                let b: Vec<Vec<f64>> = zufallsmatrix_2d(n);
                let mut c: Vec<Vec<f64>> = vec![vec![0.0; n]; n];

                // Benchmark ausführen
                messen.iter(|| {
                    manuell_unsicher::ausführen(&a, &b, &mut c, n, threads, &kerne);
                    black_box(&c);
                });
            });
        }
    }
    // Benchmark abschließen und Statistiken erstellen
    gruppe.finish();
}

/*
    Threads mit loop unrolling
*/
pub fn run_unroll(einstellungen: &mut Criterion) {
    let mut gruppe: criterion::BenchmarkGroup<'_, criterion::measurement::WallTime> = einstellungen.benchmark_group("loop unrolling");
    
    // Benchmark Einstellungen
    gruppe.sample_size(ANZAHL);
    gruppe.measurement_time(Duration::from_secs(ZEIT));

    // Kern für cpu pinning
    let kerne: Vec<core_affinity::CoreId> = get_core_ids().unwrap();

    for &n in MATRIZEN {
        for threads in 2..kerne.len() {
            gruppe.bench_with_input(BenchmarkId::from_parameter("loop unrolling"), &n, |messen, &n| {
                // Matrizen initialisieren
                let a: Vec<Vec<f64>> = zufallsmatrix_2d(n);
                let b: Vec<Vec<f64>> = zufallsmatrix_2d(n);
                let mut c: Vec<Vec<f64>> = vec![vec![0.0; n]; n];

                // Benchmark ausführen
                messen.iter(|| {
                    unroll::ausführen(&a, &b, &mut c, n, threads, &kerne);
                    black_box(&c);
                });
            });
        }
    }
    // Benchmark abschließen und Statistiken erstellen
    gruppe.finish();   
}

pub fn run_tiling(einstellungen: &mut Criterion) {
    let mut gruppe: criterion::BenchmarkGroup<'_, criterion::measurement::WallTime> = einstellungen.benchmark_group("block tiling");
    
    // Benchmark Einstellungen
    gruppe.sample_size(ANZAHL);
    gruppe.measurement_time(Duration::from_secs(ZEIT));

    // Kern für cpu pinning
    let kerne: Vec<core_affinity::CoreId> = get_core_ids().unwrap();

    for &n in MATRIZEN {
        for threads in 2..kerne.len() {
            gruppe.bench_with_input(BenchmarkId::from_parameter("block tiling"), &n, |messen, &n| {
                // Matrizen initialisieren
                let a: Vec<Vec<f64>> = zufallsmatrix_2d(n);
                let b: Vec<Vec<f64>> = zufallsmatrix_2d(n);
                let mut c: Vec<Vec<f64>> = vec![vec![0.0; n]; n];

                // Benchmark ausführen
                messen.iter(|| {
                    tiling::ausführen(&a, &b, &mut c, n, threads, &kerne);
                    black_box(&c);
                });
            });
        }
    }
    // Benchmark abschließen und Statistiken erstellen
    gruppe.finish();
}

pub fn run_simd(einstellungen: &mut Criterion) {
    let mut gruppe: criterion::BenchmarkGroup<'_, criterion::measurement::WallTime> = einstellungen.benchmark_group("simd");
    
    // Benchmark Einstellungen
    gruppe.sample_size(ANZAHL);
    gruppe.measurement_time(Duration::from_secs(ZEIT));

    // Kern für cpu pinning
    let kerne: Vec<core_affinity::CoreId> = get_core_ids().unwrap();

    for &n in MATRIZEN {
        for threads in 2..kerne.len() {
            gruppe.bench_with_input(BenchmarkId::from_parameter("simd"), &n, |messen, &n| {
                // Matrizen initialisieren
                let a: Vec<Vec<f64>> = zufallsmatrix_2d(n);
                let b: Vec<Vec<f64>> = zufallsmatrix_2d(n);
                let mut c: Vec<Vec<f64>> = vec![vec![0.0; n]; n];

                // Benchmark ausführen
                messen.iter(|| {
                    simd::ausführen(&a, &b, &mut c, n, threads, &kerne);
                    black_box(&c);
                });
            });
        }
    }
    // Benchmark abschließen und Statistiken erstellen
    gruppe.finish();
}

pub fn run_simd_tiling(einstellungen: &mut Criterion) {
    let mut gruppe: criterion::BenchmarkGroup<'_, criterion::measurement::WallTime> = einstellungen.benchmark_group("block tiling und simd");
    
    // Benchmark Einstellungen
    gruppe.sample_size(ANZAHL);
    gruppe.measurement_time(Duration::from_secs(ZEIT));

    // Kern für cpu pinning
    let kerne: Vec<core_affinity::CoreId> = get_core_ids().unwrap();

    for &n in MATRIZEN {
        for threads in 2..kerne.len() {
            gruppe.bench_with_input(BenchmarkId::from_parameter("block tiling und simd"), &n, |messen, &n| {
                // Matrizen initialisieren
                let a: Vec<Vec<f64>> = zufallsmatrix_2d(n);
                let b: Vec<Vec<f64>> = zufallsmatrix_2d(n);
                let mut c: Vec<Vec<f64>> = vec![vec![0.0; n]; n];

                // Benchmark ausführen
                messen.iter(|| {
                    simd_tiling::ausführen(&a, &b, &mut c, n, threads, &kerne);
                    black_box(&c);
                });
            });
        }
    }
    // Benchmark abschließen und Statistiken erstellen
    gruppe.finish();
}


pub fn run_rayon(einstellungen: &mut Criterion) {
    let mut gruppe: criterion::BenchmarkGroup<'_, criterion::measurement::WallTime> = einstellungen.benchmark_group("Rayon");
    
    // Benchmark Einstellungen
    gruppe.sample_size(ANZAHL);
    gruppe.measurement_time(Duration::from_secs(ZEIT));

    // Kern für cpu pinning
    let kerne: Vec<core_affinity::CoreId> = get_core_ids().unwrap();

    for &n in MATRIZEN {
        for threads in 2..kerne.len() {

            // Kopie für jeden Thread
            let kerne_kopie: Vec<core_affinity::CoreId> = kerne.clone();

            // Threadpool erstellen
            let pool = ThreadPoolBuilder::new().num_threads(threads)
                .start_handler(move |id| { set_for_current(kerne_kopie[id]); })
                    .build().unwrap_or_else(|f| { 
                        println!("Fehler beim erstellen des Threadpools: {}", f);
                        process::exit(1);});

            gruppe.bench_with_input(BenchmarkId::from_parameter("Rayon"), &n, |messen, &n| {
                // Matrizen initialisieren
                let a: Vec<Vec<f64>> = zufallsmatrix_2d(n);
                let b: Vec<Vec<f64>> = zufallsmatrix_2d(n);
                let mut c: Vec<Vec<f64>> = vec![vec![0.0; n]; n];

                // Benchmark ausführen
                messen.iter(|| {
                    pool.install(|| { mein_rayon::ausführen(&a, &b, &mut c, n); });
                    black_box(&c);
                });
            });
        }
    }
    // Benchmark abschließen und Statistiken erstellen
    gruppe.finish();
}

pub fn run_crossbeam(einstellungen: &mut Criterion) {
    let mut gruppe: criterion::BenchmarkGroup<'_, criterion::measurement::WallTime> = einstellungen.benchmark_group("Crossbeam");
    
    // Benchmark Einstellungen
    gruppe.sample_size(ANZAHL);
    gruppe.measurement_time(Duration::from_secs(ZEIT));

    // Kern für cpu pinning
    let kerne: Vec<core_affinity::CoreId> = get_core_ids().unwrap();

    for &n in MATRIZEN {
        for threads in 2..kerne.len() {
            gruppe.bench_with_input(BenchmarkId::from_parameter("Crossbeam"), &n, |messen, &n| {
                // Matrizen initialisieren
                let a: Vec<Vec<f64>> = zufallsmatrix_2d(n);
                let b: Vec<Vec<f64>> = zufallsmatrix_2d(n);
                let mut c: Vec<Vec<f64>> = vec![vec![0.0; n]; n];

                // Benchmark ausführen
                messen.iter(|| {
                    crossbeam::ausführen(&a, &b, &mut c, n, threads, &kerne);
                    black_box(&c);
                });
            });
        }
    }
    // Benchmark abschließen und Statistiken erstellen
    gruppe.finish();
}

// Einzelne Benchmarks definieren
criterion_group!(
    name = single;
    config = Criterion::default();
    targets = run_single
);

criterion_group!(
    name = manuell_sicher;
    config = Criterion::default();
    targets = run_manuell_sicher
);

criterion_group!(
    name = manuell_unsicher;
    config = Criterion::default();
    targets = run_manuell_unsicher
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

criterion_main!(single, manuell_sicher, manuell_unsicher, unroll, tiling, simd, simd_tiling, rayon, crossbeam);