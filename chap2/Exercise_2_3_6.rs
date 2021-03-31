fn insert_sort(v: &mut Vec<i32>) {
    //println!("v={:?}", &v);
    let mut start_index;
    let mut end_index;
    let mut search_index;

    for j in 1..v.len() {
        let key = v[j];
        v.remove(j);
        
        start_index = 0;
        end_index = j-1;
        
        println!("v={:?}, key={}, end_index={}", &v, key, end_index);
        
        loop {
            search_index = start_index + (end_index-start_index)/2;
            println!("search_index={}", search_index);
            
            if key > v[search_index] {
                if search_index == end_index {
                    v.insert(search_index+1, key);
                    break;
                } else {
                    start_index = search_index+1;
                }
            } else {
                if search_index == start_index {
                    v.insert(search_index, key);
                    break;
                } else {
                    end_index = search_index-1;
                }
            }
        }
    }

    //println!("v={:?}", &v);
}

fn main() {
    let mut v: Vec<i32> = vec![4, 2, 6, 1, 3, 5];
    println!("v={:?}", &v);

    insert_sort(&mut v);

    println!("sorted v={:?}", &v);
}