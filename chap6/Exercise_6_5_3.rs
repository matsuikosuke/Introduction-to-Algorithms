fn left(i: usize) -> usize {
    2*i+1
}

fn right(i: usize) -> usize {
    2*i+2
}

fn min_heap(v: &mut Vec<i32>, i: usize) {
    let l = left(i);
    let r = right(i);
    let heap_size = v.len()-1;
    let mut smallest;
    let temp = v[i];

    if l <= heap_size && v[l] < v[i] {
        smallest = l;
    } else {
        smallest = i;
    }


    if r <= heap_size && v[r] < v[smallest] {
        smallest = r;
    } 

    if smallest != i {
        v[i] = v[smallest];
        v[smallest] = temp;
        println!("v={:?}, i={}", &v, i);
        min_heap(v, smallest);
    }
}

fn build_min_heap(v: &mut Vec<i32>) {
    let heap_size = v.len();
    let start_index = heap_size/2-1;

    for i in (0..start_index+1).rev() {
        println!("v={:?}, i={}", &v, i);
        min_heap(v, i);
    }
}

fn heap_minimun(v: &Vec<i32>) -> i32 {
    v[0]
}

fn heap_extract_min(v: &mut Vec<i32>) -> Result<i32, &str> {
    let heap_size = v.len();
    if heap_size < 1 {
        return Err("Heap underflow");
    }
    let min = v[0];
    v[0] = v[heap_size-1];
    v.pop();
    min_heap(v, 0);
    
    Ok(min)
}

fn heap_decrease_key(v: &mut Vec<i32>, i: usize, key: i32) -> Result<(), &str> {
    let mut tmp;
    let mut index = i-1;
    if key > v[index] {
        return Err("The new key is larger than the current key");
    }
    v[index] = key;
    println!("v={:?}, i={}", &v, index);
    
    while index>0 && v[(index+1)/2-1] > v[index]
    {
        tmp = v[(index+1)/2-1];
        v[(index+1)/2-1] = v[index];
        v[index] = tmp;
        index = (index+1)/2-1;
        
        println!("v={:?}, i={}", &v, index);
    }
    
    Ok(())
}

fn min_heap_insert(v: &mut Vec<i32>, key: i32) {
    v.push(1000);
    heap_decrease_key(v, v.len(), key);
}

fn main() {
    let mut v: Vec<i32> = vec![15,13,9,5,12,8,7,4,0,6,2,1];
    println!("v={:?}", &v);
    
    build_min_heap(&mut v);
    println!("min heap v={:?}", &v);
    
    
    println!("minimum element is ={}", heap_minimun(&v));
    println!("minimum element is ={}", heap_extract_min(&mut v).unwrap());
    println!("min heap with minimum element removed ={:?}", &v);
    
    
    let mut new_v: Vec<i32> = Vec::new();
    println!("{}", heap_extract_min(&mut new_v).unwrap_err());

    min_heap_insert(&mut v, 3);
    
    println!("inserted v={:?}", &v);
    

    println!("{}", heap_decrease_key(&mut v, 7, 100).unwrap_err());
}