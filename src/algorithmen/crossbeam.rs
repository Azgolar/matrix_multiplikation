use core_affinity::{CoreId, set_for_current};
use crossbeam::{channel::unbounded, thread};

/*
    dynamische Arbeitsverteilung mit Crossbeam Channels
    --> Threads holen sich selbständig Zeilenabschnitte aus einer globalen Warteschlange
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

    // Sender und Empfänger als unbounded erstellen da die Anzahl der Zeilen in der Warteschlange unbekannt ist
    let (sender, empfänger) = unbounded::<usize>();

    // Warteschlange mit start indizes der jeweiligen Zeilen füllen (0, 4, 8, ...)
    for i in (0..n).step_by(zeilen) {
        sender.send(i).unwrap();
    }

    // Sender wird nicht mehr gebraucht da die komplette Arbeit (Indizes der Zeilen) in die Warteschlange eingreiht wurden
    drop(sender);

    // Crossbeam Scope
    thread::scope(|s| {
        // Alle Thread handles speichern
        let mut sammeln = Vec::with_capacity(threads);

        // Worker Threads erstellen
        for z in 0..threads {
            let kern: CoreId = pinnen[z];

            // Empfänger für jeden Thread clonen
            let empfänger_kopie = empfänger.clone();

            // Crossbeam Threads erzeugen
            let handle = s.spawn(move |_| {
                set_for_current(kern);

                // berechnete Zeile
                let mut berechnet: Vec<(usize, Vec<f64>)> = Vec::new();

                // Matrixmultiplikation durchführen solange bis alle Zeilen berechnet wurden
                for anfang in empfänger_kopie {
                    // Ende des aktuellen Zeilenbereichs berechnen
                    let ende: usize = (anfang + zeilen).min(n);

                    for i in anfang..ende {
                        let mut ergebnis: Vec<f64> = vec![0.0; n];

                        for j in 0..n {
                            let mut summe = 0.0;

                            for k in 0..n {
                                summe = summe + a[i][k] * b[k][j];
                            }
                            ergebnis[j] = summe;
                        }
                        // Zeilenindex und Zeile für Rückgabe speichern
                        berechnet.push((i, ergebnis));
                    }
                }
                // Rückgabe
                berechnet
            });
            sammeln.push(handle);
        }

        // Nur Zeiger in der Ergebnismatrix ändern, keine Daten kopieren
        for h in sammeln {
            // Threads joinen und berechnet Werte auslesen
            let rückgabe = h.join().unwrap();

            for (i, zeile) in rückgabe {
                c[i] = zeile;
            }
        }
    })
    .unwrap();
}
