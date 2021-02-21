fn search(v: i8)-> usize {
    let a: [i8; 6] = [31, 41, 59, 26, 41, 58];
    println!("{:?}", &a[0..6]);

    for i in 0..a.len() {
        if a[i] == v
        {
            return i+1;
        }
    }

    return 0;
}

fn main()
{
    let v = 26;
    println!("{}は{}番目の値", v, search(v));

    let v = 100;
    println!("{}は{}番目の値", v, search(v));
}