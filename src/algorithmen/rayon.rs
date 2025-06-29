use rayon::iter::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};

/*
    Implementierung mit der Parallelisierungsbibliothek Rayon

    Die Parallelisierung erfolgt in der äußeren Schleife, da es sich für die inneren Schleifen nicht lohnt. Dabei
    ersetzt man nur den sequentiellen iterator iter_mut() durch den parallelen Iterator von Rayon par_iter_mut()
*/
pub fn ausführen(a: &Vec<Vec<f64>>, b: &Vec<Vec<f64>>, c: &mut Vec<Vec<f64>>, n: usize) {
    c.par_iter_mut()
        .enumerate()
        .for_each(|(i, zeile): (usize, &mut Vec<f64>)| {
            for j in 0..n {
                let mut summe: f64 = 0.0;
                for k in 0..n {
                    summe = summe + a[i][k] * b[k][j];
                }
                zeile[j] = summe;
            }
        });
}
