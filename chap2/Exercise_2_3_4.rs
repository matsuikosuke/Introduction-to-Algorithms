fn insert_last(v: &mut Vec<i32>) {
    //println!("v={:?}", &v);
    
    let key = v[v.len()-1];
    let mut i = v.len()-1;
    //println!("v={:?}, key={}", &v, key);

    while i>0 && v[i-1]>key
    {
        //println!("v[{}]={}, v[{}]={}", i,v[i],i-1,v[i-1]);
        v[i] = v[i-1];
        i = i-1;
    }
    v[i] = key;
        
    //println!("v={:?}", &v);
}

fn insert_recursion(v: &mut Vec<i32>) {
    let n = v.len();
    
    if n>1 {
        let a = v[v.len()-1];
        v.pop();
        insert_recursion(v);
        v.push(a);
    } 

    insert_last(v);
}

fn main() {
    let mut v: Vec<i32> = vec![3,7,100,5,9,1,99,12,45,76,56,99,300,9,5,95];
    println!("v={:?}", &v);
    
    insert_recursion(&mut v);

    println!("sorted v={:?}", &v);
}