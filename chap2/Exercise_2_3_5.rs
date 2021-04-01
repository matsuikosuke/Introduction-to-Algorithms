fn search(x: i32, v: &mut Vec<i32>)-> usize {
    //println!("v={:?}", &v);
    let mut start_index;
    let mut end_index;
    let mut search_index;

    start_index = 0;
    end_index = v.len()-1;

    //println!("v={:?}, x={}, end_index={}", &v, x, end_index);

    loop {
        search_index = start_index + (end_index-start_index)/2;
        println!("search_index={}", search_index);

         if x == v[search_index] {
            return search_index+1;
         } else if x > v[search_index] {
            if search_index == end_index {
                return 0;
            } else {
                start_index = search_index+1;
            }
        } else {
            if search_index == start_index {
                return 0;
            } else {
                end_index = search_index-1;
            }
        }
    }
}

fn main()
{
    let mut v: Vec<i32> = vec![26, 31, 41, 41, 59, 58];
    
    let x = 26;
    println!("{}は{}番目の値", x, search(x, &mut v));
    
    let x = 31;
    println!("{}は{}番目の値", x, search(x, &mut v));
    
    let x = 41;
    println!("{}は{}番目の値", x, search(x, &mut v));
    
    let x = 59;
    println!("{}は{}番目の値", x, search(x, &mut v));

    let x = 100;
    println!("{}は{}番目の値", x, search(x, &mut v));
}