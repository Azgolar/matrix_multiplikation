#[cfg(test)]
mod tests {
    use rayon::ThreadPoolBuilder;
    use crate::matrix::zufallsmatrix_2d;
    use core_affinity::{get_core_ids, set_for_current, CoreId};
    use std::process;

    use crate::algorithmen::crossbeam;
    use crate::algorithmen::manuell_sicher;
    use crate::algorithmen::manuell_unsicher;
    use crate::algorithmen::rayon as mein_rayon;
    use crate::algorithmen::simd;
    use crate::algorithmen::simd_tiling;
    use crate::algorithmen::single;
    use crate::algorithmen::tiling;
    use crate::algorithmen::unroll;

    fn vergleich(a: &Vec<Vec<f64>>, b: &Vec<Vec<f64>>, n: usize) -> bool {
        let genauigkeit = 1e-10;
        for i in 0..n {
            for j in 0..n {
                if (a[i][j] - b[i][j]).abs() > genauigkeit {
                    return false;
                }
            }
        }
        true
    }

    #[test]
    fn testen() {
        // testen mit geraden und ungerade Matritzen 
        let groessen: Vec<usize> = vec![4, 5, 12, 13, 30, 33, 68, 71, 126, 131, 256, 271, 300];

        let kerne: Vec<CoreId> = get_core_ids().unwrap();

        // Es soll mit einer geraden und ungeraden Anzahl an Threads getestet werden
        let threads: Vec<usize>;
        if kerne.len() >= 5 {
            threads = vec![4,5];
        }
        else if kerne.len() >= 2 {
            threads = vec![kerne.len() - 1, kerne.len()];
        }
        else {
            threads = vec![1];
        }

        let mut i: u32 = 1;
        for thread in threads {
            println!("Testen von Threads = {}", thread);
            for &n in &groessen {
                println!("Test {}/{}", i, groessen.len());
                i = i + 1;

                // Matrizen initialisieren
                let a: Vec<Vec<f64>> = zufallsmatrix_2d(n);
                let b: Vec<Vec<f64>> = zufallsmatrix_2d(n);
                let mut c: Vec<Vec<f64>> = vec![vec![0.0; n]; n];

                // single Thread als Basis für Vergleich
                single::ausführen(&a, &b, &mut c, n, &kerne[0]);
                
                let mut ergebnis: Vec<Vec<f64>> = vec![vec![0.0; n]; n];
                manuell_sicher::ausführen(&a, &b, &mut ergebnis, n, thread, &kerne);
                assert!(vergleich(&c, &ergebnis, n), "manuell_sicher.rs ist falsch für threads = {}, n = {}", thread, n);
            
                let mut ergebnis: Vec<Vec<f64>> = vec![vec![0.0; n]; n];
                manuell_unsicher::ausführen(&a, &b, &mut ergebnis, n, thread, &kerne);
                assert!(vergleich(&c, &ergebnis, n), "manuell_unsicher.rs ist falsch für threads = {}, n = {}", thread, n);

                // Bibliothek Crossbeam testen
                ergebnis = vec![vec![0.0; n]; n];
                crossbeam::ausführen(&a, &b, &mut ergebnis, n, thread, &kerne);
                assert!(vergleich(&c, &ergebnis, n), "crossbeam.rs ist falsch für threads = {}, n = {}", thread, n);

                // Bibliothek Rayon testen
                ergebnis = vec![vec![0.0; n]; n];
                let kopie: Vec<CoreId> = kerne.clone();
                let pool: rayon::ThreadPool = ThreadPoolBuilder::new().num_threads(thread)
                    .start_handler(move |id| { set_for_current(kopie[id]); })
                    .build().unwrap_or_else(|f| { 
                        println!("Fehler beim erstellen des Threadpools: {}", f);
                        process::exit(1)});
                pool.install(|| { mein_rayon::ausführen(&a, &b, &mut ergebnis, n); });
                assert!(vergleich(&c, &ergebnis, n), "rayon.rs ist falsch für threads = {}, n = {}", thread, n);

                ergebnis = vec![vec![0.0; n]; n];
                simd::ausführen(&a, &b, &mut ergebnis, n, thread, &kerne);
                assert!(vergleich(&c, &ergebnis, n), "simd.rs ist falsch für threads = {}, n = {}", thread, n);

                ergebnis = vec![vec![0.0; n]; n];
                tiling::ausführen(&a, &b, &mut ergebnis, n, thread, &kerne);
                assert!(vergleich(&c, &ergebnis, n), "tiling.rs ist falsch für threads = {}, n = {}", thread, n);

                ergebnis = vec![vec![0.0; n]; n];
                simd_tiling::ausführen(&a, &b, &mut ergebnis, n, thread, &kerne);
                assert!(vergleich(&c, &ergebnis, n), "simd_tiling.rs ist falsch für threads = {}, n = {}", thread, n);

                ergebnis = vec![vec![0.0; n]; n];
                unroll::ausführen(&a, &b, &mut ergebnis, n, thread, &kerne);
                assert!(vergleich(&c, &ergebnis, n), "unroll.rs ist falsch für threads = {}, n = {}", thread, n);
            }
            i = 1;
        }
        
        println!("\nAlle Funktionen sind korrekt");
    }
}