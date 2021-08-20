fn left(i: usize) -> usize {
    2*i+1
}

fn right(i: usize) -> usize {
    2*i+2
}

fn max_heap(v: &mut Vec<i32>, i: usize) {
    let mut l;
    let mut r;
    let mut index = i;
    let heap_size = v.len()-1;
    let mut largest;
    let mut temp;

    while index < heap_size {
        l = left(index);
        r = right(index);
        temp = v[index];

        if l <= heap_size && v[l] > v[index] {
            largest = l;
        } else {
            largest = index;
        }    

        if r <= heap_size && v[r] > v[largest] {
            largest = r;
        } 

        if largest != index {
            v[index] = v[largest];
            v[largest] = temp;
            index = largest;
            println!("v={:?}, largest={}, index={}", &v, largest, index);
        } else {
            return;
        }
    }
}

fn main() {
    let mut v: Vec<i32> = vec![16, 4, 10, 14, 7, 9, 3, 2, 8, 1];
    println!("v={:?}", &v);

    max_heap(&mut v, 1);

    println!("sorted v={:?}", &v);
}