type Matrix = Vec<Vec<i32>>;

fn matrix_print(a: &Matrix) {
    println!("{}X{}s—ñ",a.len(), a[0].len());
    for i in a {
        println!("{:?}",i);
    }
}

fn matrix_multiple(a: &Matrix, b: &Matrix) -> Matrix {
    let c_row = a.len();
    let n = b.len();
    let c_col = b[0].len();

    let mut c = vec![vec![0;c_col];c_row];

    for i in 0..c_row {
        for j in 0..c_col {
            c[i][j] = 0;
            for k in 0..n {
                c[i][j] = c[i][j] + a[i][k]*b[k][j];
            }
        }
    }

    c
}

fn main() {
    //let a: Matrix = vec![vec![1,2,3], vec![4,5,6], vec![7,8,9]];
    //let b: Matrix = vec![vec![10,11,12], vec![13,14,15], vec![16,17,18]];
    let a: Matrix = vec![vec![1,2,3,4,5], vec![6,7,8,9,10], vec![11,12,13,141,15]];
    let b: Matrix = vec![vec![16,17], vec![18,19], vec![20,21], vec![22,23], vec![24,25]];
    matrix_print(&a);
    matrix_print(&b);

    matrix_print(&matrix_multiple(&a, &b));
}