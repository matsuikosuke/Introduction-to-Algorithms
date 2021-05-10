fn find_max_round_robin(v: Vec<i32>) {
    let mut start = 0;
    let mut end = start;
    let mut max = 0;

    for j in 0..v.len()-1 {
        for i in j+1..v.len() {
            if (v[i] - v[j]) > max
            {
                start = j;
                end = i;
                max = v[i] - v[j];
            }
        }
    }

    println!("{}日目終了時に{}ドルで購入し、{}日目終了時に{}ドルで売却すると、最大の利益{}を得る", start, v[start], end, v[end], max);
}

fn main() {
    let v: Vec<i32> = vec![100, 113, 110, 85, 105, 102, 86, 63, 81, 101, 94, 106, 101, 79, 94, 90, 97];
    //let v: Vec<i32> = vec![1, 900, 110, 85, 105, 102, 86, 63, 81, 101, 94, 106, 101, 79, 94, 90, 97];
    //let v: Vec<i32> = vec![1, 113, 110, 85, 105, 102, 86, 63, 81, 101, 94, 106, 101, 79, 94, 90, 900];
    println!("v={:?}", &v);

    find_max_round_robin(v);
}