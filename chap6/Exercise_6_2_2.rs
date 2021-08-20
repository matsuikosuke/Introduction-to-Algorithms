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
        println!("v={:?}, smallest={}, i={}", &v, smallest, i);
        min_heap(v, smallest);
    }
}
fn main() {
    let mut v: Vec<i32> = vec![1,9,2,3,4,5,6,7,8,10,11,12];
    println!("v={:?}", &v);

    min_heap(&mut v, 1);

    println!("sorted v={:?}", &v);
}