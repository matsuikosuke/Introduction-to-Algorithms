fn main() {
    let mut a: [i8; 6] = [31, 41, 59, 26, 41, 58];
    println!("{:?}", &a[0..6]);

    for j in 1..a.len() {
        let key = a[j];
        let mut i = j;

        while i>0 && a[i-1]>key
        {
            a[i] = a[i-1];
            i = i-1;
        }
        a[i] = key;
    }
    println!("{:?}", &a[0..6]);
}