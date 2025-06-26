#[cfg(test)]
mod tests {
    use rayon::ThreadPoolBuilder;
    use crate::matrix::zufallsmatrix_2d;
    use core_affinity::{get_core_ids, CoreId};

    use crate::algorithmen::crossbeam;
    use crate::algorithmen::manuell;
    use crate::algorithmen::rayon as mein_rayon;
    use crate::algorithmen::simd;
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
        // testen mit geraden und ungeraden Anzahl von Threads
        let threads: Vec<u32> = vec![4, 5];

        let kerne: Vec<CoreId> = get_core_ids().unwrap();

        for thread in threads {
            println!("Testen von Threads = {}", thread);
            for &n in &groessen {
                println!("n = {}", n);

                // Matrizen initialisieren
                let a: Vec<Vec<f64>> = zufallsmatrix_2d(n);
                let b: Vec<Vec<f64>> = zufallsmatrix_2d(n);
                let mut c: Vec<Vec<f64>> = vec![vec![0.0; n]; n];

                // single Thread als Basis für Vergleich
                single::ausführen(&a, &b, &mut c, n, &kerne[0]);
                
                let mut ergebnis: Vec<Vec<f64>> = vec![vec![0.0; n]; n];
                manuell::ausführen(&a, &b, &mut ergebnis, n, &kerne);
                assert!(vergleich(&c, &ergebnis, n), "manuell.rs ist falsch für threads = {}, n = {}", thread, n);

                ergebnis = vec![vec![0.0; n]; n];
            }
        }
        
        println!("\nAlle Funktionen sind korrekt");
    }
}