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
        //println!("v={:?}, largest={}, i={}", &v, largest, i);
        max_heap(v, largest);
    }
}

fn build_max_heap(v: &mut Vec<i32>) {
    let heap_size = v.len()-1;

    for i in (0..heap_size/2).rev() {
        println!("v={:?}, i={}", &v, i);
        max_heap(v, i);
    }
}

fn main() {
    //let mut v: Vec<i32> = vec![16, 4, 10, 14, 7, 9, 3, 2, 8, 1];
    let mut v: Vec<i32> = vec![4,1,3,2,16,9,10,14,8,7];
    println!("v={:?}", &v);

    build_max_heap(&mut v);

    println!("sorted v={:?}", &v);
}