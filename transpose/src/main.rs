fn transpose(matrix: &mut [[i32; 3]; 3]) {
    for i in 0..3 {
        for j in i + 1..3 {
            let temp = matrix[i][j];
            matrix[i][j] = matrix[j][i];
            matrix[j][i] = temp;
        }
    }
}

fn main() {
    let mut matrix = [
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9]
    ];
    transpose(&mut matrix);
    for row in matrix.iter() {
        println!("{:?}", row);
    }
}
