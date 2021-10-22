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

fn heap_sort(v: &mut Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    let mut temp;
    build_max_heap(v);
    println!("v={:?}", &v);

    let length = v.len();

    for i in (1..length).rev() {
        temp = v[i];
        v[i] = v[0];
        v[0] = temp;
        println!("v={:?}, sorted_v={:?}, {}", &v, &result, i);

        temp = v.pop().unwrap();
        result.push(temp);

        max_heap(v, 0);
        println!("v={:?}, sorted_v={:?}, i={}", &v, &result, i);
    }    

    temp = v.pop().unwrap();
    result.push(temp);
    result

}

fn main() {
    //let mut v: Vec<i32> = vec![16, 4, 10, 14, 7, 9, 3, 2, 8, 1];
    //let mut v: Vec<i32> = vec![4,1,3,2,16,9,10,14,8,7];
    //let mut v: Vec<i32> = vec![27,17,3,16,13,10,1,5,7,12,4,8,9,0];
    let mut v: Vec<i32> = vec![5,3,17,10,84,19,6,22,9];
    println!("v={:?}", &v);

    println!("sorted v={:?}", heap_sort(&mut v));
}