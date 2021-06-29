
type Matrix = Vec<Vec<i32>>;

fn matrix_print(a: &Matrix) {
    println!("{}X{}s—ñ",a.len(), a[0].len());
    for i in a {
        println!("{:?}",i);
    }
}


fn square_matrix_multiple(a: &Matrix, b: &Matrix) -> Matrix {
    let n: usize = a.len();
    let mut c = vec![vec![0;n];n];

    for i in 0..n {
        for j in 0..n {
            c[i][j] = 0;
            for k in 0..n {
                c[i][j] = c[i][j] + a[i][k]*b[k][j];
            }
        }
    }

    c
}

fn square_matrix_multiple_recursive(a: &Matrix, b: &Matrix) -> Matrix {
    let n: usize = a.len();
    let mut c = vec![vec![0;n];n];

    if 1 == n {
        c[0][0] = a[0][0]*b[0][0];
    }
    else
    {
        let mut a11 = vec![vec![0;n/2];n/2];
        let mut a12 = vec![vec![0;n/2];n/2];
        let mut a21 = vec![vec![0;n/2];n/2];
        let mut a22 = vec![vec![0;n/2];n/2];

        let mut b11 = vec![vec![0;n/2];n/2];
        let mut b12 = vec![vec![0;n/2];n/2];
        let mut b21 = vec![vec![0;n/2];n/2];
        let mut b22 = vec![vec![0;n/2];n/2];

        for i in 0..n/2 {
            for j in 0..n/2 {
                a11[i][j] = a[i][j];
                a12[i][j] = a[i][n/2+j];
                a21[i][j] = a[n/2+i][j];
                a22[i][j] = a[n/2+i][n/2+j];

                b11[i][j] = b[i][j];
                b12[i][j] = b[i][n/2+j];
                b21[i][j] = b[n/2+i][j];
                b22[i][j] = b[n/2+i][n/2+j];
            }
        }
        /*matrix_print(&a11);
        matrix_print(&a12);
        matrix_print(&a21);
        matrix_print(&a22);*/

        let mut s1 = vec![vec![0;n/2];n/2];
        let mut s2 = vec![vec![0;n/2];n/2];
        let mut s3 = vec![vec![0;n/2];n/2];
        let mut s4 = vec![vec![0;n/2];n/2];
        let mut s5 = vec![vec![0;n/2];n/2];
        let mut s6 = vec![vec![0;n/2];n/2];
        let mut s7 = vec![vec![0;n/2];n/2];
        let mut s8 = vec![vec![0;n/2];n/2];
        let mut s9 = vec![vec![0;n/2];n/2];
        let mut s10 = vec![vec![0;n/2];n/2];

        for i in 0..n/2 {
            for j in 0..n/2 {
                s1[i][j] = b12[i][j]-b22[i][j];
                s2[i][j] = a11[i][j]+a12[i][j];
                s3[i][j] = a21[i][j]+a22[i][j];
                s4[i][j] = b21[i][j]-b11[i][j];
                s5[i][j] = a11[i][j]+a22[i][j];
                s6[i][j] = b11[i][j]+b22[i][j];
                s7[i][j] = a12[i][j]-a22[i][j];
                s8[i][j] = b21[i][j]+b22[i][j];
                s9[i][j] = a11[i][j]-a21[i][j];
                s10[i][j] = b11[i][j]+b12[i][j];
            }
        }

        let mut p1 = vec![vec![0;n/2];n/2];
        let mut p2 = vec![vec![0;n/2];n/2];
        let mut p3 = vec![vec![0;n/2];n/2];
        let mut p4 = vec![vec![0;n/2];n/2];
        let mut p5 = vec![vec![0;n/2];n/2];
        let mut p6 = vec![vec![0;n/2];n/2];
        let mut p7 = vec![vec![0;n/2];n/2];

        p1 = square_matrix_multiple(&a11, &s1);
        p2 = square_matrix_multiple(&s2, &b22);
        p3 = square_matrix_multiple(&s3, &b11);
        p4 = square_matrix_multiple(&a22, &s4);
        p5 = square_matrix_multiple(&s5, &s6);
        p6 = square_matrix_multiple(&s7, &s8);
        p7 = square_matrix_multiple(&s9, &s10);

        let mut c11 = vec![vec![0;n/2];n/2];
        let mut c12 = vec![vec![0;n/2];n/2];
        let mut c21 = vec![vec![0;n/2];n/2];
        let mut c22 = vec![vec![0;n/2];n/2];

        for i in 0..n/2 {
            for j in 0..n/2 {
                c11[i][j] = p5[i][j]+p4[i][j]-p2[i][j]+p6[i][j];
                c12[i][j] = p1[i][j]+p2[i][j];
                c21[i][j] = p3[i][j]+p4[i][j];
                c22[i][j] = p5[i][j]+p1[i][j]-p3[i][j]-p7[i][j];
            }
        }

        for i in 0..n/2 {
            for j in 0..n/2 {
                c[i][j] = c11[i][j];
                c[i][n/2+j] = c12[i][j];
                c[n/2+i][j] = c21[i][j];
                c[n/2+i][n/2+j] = c22[i][j];
            }
        }

    }
    c
}

fn main() {
    let a: Matrix = vec![vec![1,2,3,4,5,6], vec![7,8,9,10,11,12], vec![13,14,15,16,17,18], vec![19,20,21,22,23,24], vec![25,26,27,28,29,30], vec![31,32,33,34,35,36]];
    let b: Matrix = vec![vec![37,38,39,40,41,42], vec![43,44,45,46,47,48], vec![49,50,51,52,53,54], vec![55,56,57,58,59,60], vec![61,62,63,64,65,66], vec![67,68,69,70,71,72], ];
    matrix_print(&a);
    matrix_print(&b);

    matrix_print(&square_matrix_multiple_recursive(&a, &b));
}