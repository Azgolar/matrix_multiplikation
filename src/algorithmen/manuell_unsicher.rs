use core_affinity::{CoreId, set_for_current};
use std::{
    sync::atomic::{AtomicPtr, AtomicUsize, Ordering},
    thread,
};

/*
    dynamische Arbeitsverteilung mit Rust Threads. In dieser Variante wurde unsafe benutzt
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

    // atomarer Zähler für die dynamische Arbeitsverteilung mit Startwert null (= nächste zu verarbeitende Zeile)
    let zähler: AtomicUsize = AtomicUsize::new(0);

    // Thread sichere verteilung des rohen Zeigers zwishcen den Threads.
    // Atomar wird nicht wegen dem Zugriff benötigt sondern weil es Send/Sync kompatibel ist
    let c_zeiger: AtomicPtr<Vec<f64>> = AtomicPtr::new(c.as_mut_ptr());

    thread::scope(|s| {
        for z in 0..threads {
            let kern: CoreId = pinnen[z];
            let zähler_neu: &AtomicUsize = &zähler;
            // atomar ist nicht wegen der Atomarität notwendig, sondern dass der Zeiger Thread sicher an
            // die Threads verteilt werden darf.
            let c_neu: &AtomicPtr<Vec<f64>> = &c_zeiger;

            s.spawn(move || {
                set_for_current(kern);

                // Zeiger auf Ergebnismatrix laden
                let zeiger: *mut Vec<f64> = c_neu.load(Ordering::Relaxed);

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
                        // Jeder Thread arbeitet zwar in unterschiedlichen Zeilen, aber der Compiler kann dies zu
                        // compilezeit nicht garantieren. Daher ist ein unsicherer Zugriff auf i-te Zeile notwendig
                        let ergebnis: &mut Vec<f64> = unsafe { &mut *zeiger.add(i) };

                        for j in 0..n {
                            let mut summe: f64 = 0.0;
                            for k in 0..n {
                                summe = summe + a[i][k] * b[k][j];
                            }
                            ergebnis[j] = summe;
                        }
                    }
                }
            });
        }
    });
}
