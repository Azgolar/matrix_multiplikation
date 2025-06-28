use rand::random_range;

/*
    erstellt eine "2D Matrix mit Zufallswerten im Bereich [-1.0, 1,0]"
*/
pub fn zufallsmatrix_2d(n: usize) -> Vec<Vec<f64>> {
    let mut matrix: Vec<Vec<f64>> = vec![vec![0.0; n]; n];
    
    for i in 0..n {
        for j in 0..n {
            matrix[i][j] = random_range(-1.0..=1.0);
        }
    }
    matrix
}