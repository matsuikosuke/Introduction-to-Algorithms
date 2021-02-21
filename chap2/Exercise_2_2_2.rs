fn main() {
    let mut a: [i8; 6] = [4, 2, 6, 1, 3, 5];
    println!("{:?}", &a[0..6]);

    for j in 0..a.len()-1 {
        let mut key = j;
        let mut min = a[key];

        for i in j+1..a.len() {
            if min >  a[i]
            {
                key = i;
                min = a[i];
            }
        }
        a[key] = a[j];
        a[j] = min;
    }
    println!("{:?}", &a[0..6]);
}