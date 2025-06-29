use core_affinity::{CoreId, set_for_current};
use std::{
    sync::atomic::{AtomicUsize, Ordering},
    thread,
};

/*
    dynamische Arbeitsverteilung mit Rust Threads. Es wird Block Tiling verwendet
*/
pub fn ausführen(
    a: &Vec<Vec<f64>>,
    b: &Vec<Vec<f64>>,
    c: &mut Vec<Vec<f64>>,
    n: usize,
    threads: usize,
    pinnen: &Vec<CoreId>,
) {
    // jeder Thread darf sich jedesmal 4 Zeilen nehmen
    let zeilen: usize = 4;

    // Blockgröße
    let block = 8;

    // atomarer Zähler für die dynamische Arbeitsverteilung mit Startwert null (= nächste zu verarbeitende Zeile)
    let zähler: AtomicUsize = AtomicUsize::new(0);

    thread::scope(|s| {
        // Thread Handles fürs joinen sammeln
        let mut sammeln: Vec<thread::ScopedJoinHandle<'_, Vec<(usize, Vec<f64>)>>> =
            Vec::with_capacity(threads);

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

                        // äußere Schleife über j-Blöcke um b[k][j] erneut zu benutzen
                        for k_block in (0..n).step_by(block) {
                            let k_max = (k_block + block).min(n);

                            // innere Schleife über k Blöcke
                            for j_block in (0..n).step_by(block) {
                                let j_max = (j_block + block).min(n);

                                for k in k_block..k_max {
                                    // ändert sich in Schleife j nicht
                                    let optimiert = a[i][k];

                                    for j in j_block..j_max {
                                        zeile[j] = zeile[j] + optimiert * b[k][j];
                                    }
                                }
                            }
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
