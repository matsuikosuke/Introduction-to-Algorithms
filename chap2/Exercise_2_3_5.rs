fn search(v: i8)-> usize {
    let a: [i8; 6] = [26, 31, 41, 41, 59, 58];
    println!("{:?}", &a[0..6]);
    
    let mut start_index;
    let mut end_index;
    let mut search_index;
    
    start_index = 0;
    end_index = a.len()-1;
    
    loop {
        search_index = start_index + (end_index+1-start_index)/2;
        println!("start_index = {}, end_index = {}, search_index = {}", start_index, end_index, search_index);
        
        if search_index == 0 || search_index == a.len()-1 {
            break;
        }
        
        if v == a[search_index] {
            return search_index+1;
        } else if v > a[search_index] {
            start_index = search_index+1;
        } else {
            end_index = search_index-1;
        }
        
    }
    
    for i in start_index..end_index+1 {
        //println!("start_index = {}, end_index = {}, search_index = {}", start_index, end_index, search_index);
        //println!("a[{}] = {}", i, a[i]);
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
    
    let v = 31;
    println!("{}は{}番目の値", v, search(v));
    
    let v = 41;
    println!("{}は{}番目の値", v, search(v));
    
    let v = 59;
    println!("{}は{}番目の値", v, search(v));

    let v = 100;
    println!("{}は{}番目の値", v, search(v));
}