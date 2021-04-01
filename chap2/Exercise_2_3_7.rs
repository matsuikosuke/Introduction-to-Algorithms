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

fn search_sum(x: i32, v: &mut Vec<i32>) {
    //println!("v={:?}", &v);
    let mut start_index;
    let mut end_index;
    let mut search_index;

    for j in 0..v.len()-1 {
        let key = x-v[j];

        start_index = j;
        end_index = v.len()-1;
        //println!("v={:?}, x={}, end_index={}", &v, x, end_index);
    
        loop {
            search_index = start_index + (end_index-start_index)/2;
            //println!("search_index={}", search_index);
    
             if key == v[search_index] {
                println!("{} = v[{}]({}) + v[{}]({})", x, j, v[j], search_index, v[search_index]);
                return ;
             } else if key > v[search_index] {
                if search_index == end_index {
                    break;
                } else {
                    start_index = search_index+1;
                }
            } else {
                if search_index == start_index {
                    break;
                } else {
                    end_index = search_index-1;
                }
            }
        }
    }
    
    println!("notihng");
}

fn main()
{
    let mut v: Vec<i32> = vec![31, 41, 26, 59, 41, 58];
    println!("v={:?}", &v);
    
    insert_sort(&mut v);
    println!("v={:?}", &v);
    
    let x = 1000; //85; //100; // 
    search_sum(x, &mut v);
}