use std::{thread, simd::f64x4, sync::atomic::{AtomicUsize, Ordering}};
use core_affinity::{set_for_current, CoreId};

/*
    dynamische Arbeitsverteilung mit Rust Threads. Es wurde die Instruktion simd verwendet.

    Zum testen wurde ein i7-14700k verwendet. Der Prozessor hat eine AVX2 Registerbreite von 256 bit (= 4 * 64 bit)
*/
pub fn ausführen(a: &Vec<Vec<f64>>, b: &Vec<Vec<f64>>, c: &mut Vec<Vec<f64>>, n: usize, threads: usize, pinnen: &Vec<CoreId>) {

    // jeder Thread darf sich jedesmal 4 Zeilen nehmen
    let zeilen: usize = 4;

    // atomarer Zähler für die dynamische Arbeitsverteilung mit Startwert null (= nächste zu verarbeitende Zeile)
    let zähler: AtomicUsize = AtomicUsize::new(0);
    
    thread::scope(|s| {
        // Thread Handles fürs joinen sammeln
        let mut sammeln: Vec<thread::ScopedJoinHandle<'_, Vec<(usize, Vec<f64>)>>>= Vec::with_capacity(threads);

        for z in 0..threads {
            let kern: CoreId = pinnen[z];

            let zähler_neu: &AtomicUsize = &zähler;

            let handle: thread::ScopedJoinHandle<'_, Vec<(usize, Vec<f64>)>> = s.spawn(move || {
                set_for_current(kern);

                // berechnete Zeilen sammeln
                let mut berechnet: Vec<(usize, Vec<f64>)> = Vec::new();

                // Schleife für die dynamischen Zeilenverteilung
                loop {
                    // anfang des aktuellen Zeilenbereichs
                    let anfang: usize = zähler_neu.fetch_add(zeilen, Ordering::Relaxed);
                    if anfang >= n {
                        break;
                    }

                    let rest: usize = (n / 4) * 4;

                    // ende des aktuellen Zeilenbereichs
                    let ende: usize = (anfang + zeilen).min(n);

                    for i in anfang..ende {
                        let mut zeile: Vec<f64> = vec![0.0; n];

                        for j in (0..rest).step_by(4) {
                            let mut summe:std::simd::Simd<f64, 4> = f64x4::splat(0.0);
                            for k in 0..n {
                                let teil1: std::simd::Simd<f64, 4> = f64x4::splat(a[i][k]);

                                let teil2: std::simd::Simd<f64, 4> = f64x4::from_array([b[k][j], b[k][j + 1], 
                                    b[k][j + 2], b[k][j + 3]]);
                                    summe = summe + teil1 * teil2;
                            }

                            // Ergebnisse speichern
                            let zwischen: [f64; 4] = summe.to_array();
                            for l in 0..4 {
                                zeile[j + l] = zwischen[l];
                            }
                    
                        }

                        // restliche Spalten einzelen berechnen
                        for x in rest..n {
                            let mut summe2 = 0.0;
                            for y in 0..n {
                                summe2 = summe2 + a[i][y] * b[y][x];
                            }
                            zeile[x] = summe2;
                        }
                        berechnet.push((i, zeile));
                    }
                }
                // Rückgabe von Thread
                berechnet
            });
            sammeln.push(handle);
        }

        // Speichern der Zeilen in der Ergebnismatrix durch austauschen der Zeiger (es werden keine Daten kopiert)
        for h in sammeln {
            let rückgabe: Vec<(usize, Vec<f64>)> = h.join().unwrap();
            for (i, zeile) in rückgabe {
                c[i] = zeile;
            } 
        }
    });
}