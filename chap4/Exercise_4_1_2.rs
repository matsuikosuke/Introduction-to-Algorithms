fn find_max_round_robin(v: Vec<i32>) {
    let mut start = 0;
    let mut end = start;
    let mut max = 0;
    let mut temp_max;
    
    for j in 0..v.len()-1 {
        temp_max = 0;
        for i in j..v.len() {
            temp_max += v[i];
            if temp_max > max
            {
                start = j;
                end = i+1;
                max = temp_max;
            }
        }
    }

    println!("{}“ú–ÚI—¹‚Éw“ü‚µA{}“ú–ÚI—¹‚É”„‹p‚·‚é‚ÆAÅ‘å‚Ì—˜‰v{}‚ğ“¾‚é", start, end, max);
}

fn main() {
    let v: Vec<i32> = vec![100, 113, 110, 85, 105, 102, 86, 63, 81, 101, 94, 106, 101, 79, 94, 90, 97];
    //let v: Vec<i32> = vec![1, 900, 110, 85, 105, 102, 86, 63, 81, 101, 94, 106, 101, 79, 94, 90, 97];
    //let v: Vec<i32> = vec![1, 113, 110, 85, 105, 102, 86, 63, 81, 101, 94, 106, 101, 79, 94, 90, 900];
    println!("v={:?}", &v);
    
    let mut dv: Vec<i32> = vec![];
    for j in 0..v.len()-1 {
        dv.push(v[j+1]-v[j])
    }
    println!("dv={:?}", dv);

    find_max_round_robin(dv);
}