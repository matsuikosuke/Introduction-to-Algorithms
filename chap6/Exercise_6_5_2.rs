fn left(i: usize) -> usize {
    2*i+1
}

fn right(i: usize) -> usize {
    2*i+2
}

fn max_heap(v: &mut Vec<i32>, i: usize) {
    let l = left(i);
    let r = right(i);
    let heap_size = v.len()-1;
    let mut largest;
    let temp = v[i];

    if l <= heap_size && v[l] > v[i] {
        largest = l;
    } else {
        largest = i;
    }    

    if r <= heap_size && v[r] > v[largest] {
        largest = r;
    } 

    if largest != i {
        v[i] = v[largest];
        v[largest] = temp;
        println!("v={:?}, i={}", &v, i);
        max_heap(v, largest);
    }
}

fn build_max_heap(v: &mut Vec<i32>) {
    let heap_size = v.len();
    let start_index = heap_size/2-1;

    for i in (0..start_index+1).rev() {
        println!("v={:?}, i={}", &v, i);
        max_heap(v, i);
    }
}

fn heap_increase_key(v: &mut Vec<i32>, i: usize, key: i32) -> Result<(), &str> {
    let mut tmp;
    let mut index = i-1;
    if key < v[index] {
        return Err("The new key is smaller than the current key");
    }
    v[index] = key;
    println!("v={:?}, i={}", &v, index);
    
    while index>0 && v[(index+1)/2-1] < v[index]
    {
        tmp = v[(index+1)/2-1];
        v[(index+1)/2-1] = v[index];
        v[index] = tmp;
        index = (index+1)/2-1;
        
        println!("v={:?}, i={}", &v, index);
    }
    
    Ok(())
}

fn max_heap_insert(v: &mut Vec<i32>, key: i32) {
    v.push(-1000);
    heap_increase_key(v, v.len(), key);
}

fn main() {
    //let mut v: Vec<i32> = vec![27,17,3,16,13,10,1,5,7,12,4,8,9,0];
    let mut v: Vec<i32> = vec![15,13,9,5,12,8,7,4,0,6,2,1];
    println!("v={:?}", &v);
    
    build_max_heap(&mut v);
    println!("max heap v={:?}", &v);
    
    max_heap_insert(&mut v, 10);
    
    println!("inserted v={:?}", &v);
    
    println!("{}", heap_increase_key(&mut v, 1, 10).unwrap_err());
}