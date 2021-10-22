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


fn heap_maximun(v: &Vec<i32>) -> i32 {
    v[0]
}

fn heap_extract_max(v: &mut Vec<i32>) -> Result<i32, &str> {
    let heap_size = v.len();
    if heap_size < 1 {
        return Err("Heap underflow");
    }
    let max = v[0];
    v[0] = v[heap_size-1];
    v.pop();
    max_heap(v, 0);
    
    Ok(max)
}

fn main() {
    //let mut v: Vec<i32> = vec![27,17,3,16,13,10,1,5,7,12,4,8,9,0];
    //let mut v: Vec<i32> = vec![16,14,10,8,7,9,3,2,4,1];
    let mut v: Vec<i32> = vec![15,13,9,5,12,8,7,4,0,6,2,1];
    println!("v={:?}", &v);
    
    build_max_heap(&mut v);
    println!("max heap v={:?}", &v);
        
    println!("maximum element is ={}", heap_maximun(&v));
    println!("maximum element is ={}", heap_extract_max(&mut v).unwrap());
    println!("max heap with maximum element removed ={:?}", &v);
        
    let mut new_v: Vec<i32> = Vec::new();
    println!("maximum element is ={}", heap_extract_max(&mut new_v).unwrap_err());
}