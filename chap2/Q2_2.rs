fn bubble_sort(v: &mut Vec<i32>) {
    //println!("insert v={:?}", &v);
    let mut key;
    
    for i in 0..v.len()-1 {
        for j in (i+1..v.len()).rev() {
            if v[j] < v[j-1] {
                key = v[j];
                v[j] = v[j-1];
                v[j-1] = key;
            }
        }
    }
    //println!("insert sorted v={:?}", &v);
}

fn bubble_sort_check(v: &mut Vec<i32>)->bool {
    for i in 0..v.len()-1 {
        if v[i] <= v[i+1] {
        
        } else {
            println!("Not sorted");
            return false;
        }
    }
    
    println!("Sorted");
    return true;
}

fn main() {
    let mut v: Vec<i32> = vec![3,7,100,5,9,1,99,12,45,76,98,1,34,76,4,900,342];
    println!("v={:?}", &v);

    bubble_sort(&mut v);

    println!("sorted v={:?}", &v);
    
    bubble_sort_check(&mut v);
}