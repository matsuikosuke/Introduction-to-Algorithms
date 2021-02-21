fn main() {
    let a: [i8; 5] = [1, 1, 0, 0, 1];
    let b: [i8; 5] = [0, 1, 1, 0, 1];
    let mut c: [i8; 6] = [0, 0, 0, 0, 0, 0];
    println!("{:?}", &a[0..5]);
    println!("{:?}", &b[0..5]);

    let mut carry = 0;

    for i in (0..a.len()).rev() {
        c[i] = (a[i]+b[i]+carry)%2;
        carry = (a[i]+b[i]+carry)/2;
    }
    c[a.len()] = carry;
    c = c.rev();

    println!("{:?}", &c[0..a.len()+1]);
}