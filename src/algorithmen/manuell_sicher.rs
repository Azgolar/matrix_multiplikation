use std::{thread, sync::atomic::{AtomicUsize, Ordering}};
use core_affinity::{set_for_current, CoreId};

/*
    dynamische Arbeitsverteilung mit Rust Threads. In dieser Variante wurde kein unsafe benutzt
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

                    // ende des aktuellen Zeilenbereichs
                    let ende: usize = (anfang + zeilen).min(n);

                    for i in anfang..ende {
                        let mut zeile: Vec<f64> = vec![0.0; n];

                        for j in 0..n {
                            let mut summe: f64 = 0.0;
                            for k in 0..n {
                                summe = summe + a[i][k] * b[k][j];
                            }
                            zeile[j] = summe;
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