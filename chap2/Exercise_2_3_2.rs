fn merge(v: &mut Vec<i32>, p: usize, q: usize, r: usize) {//p=0,q=2,r=5
    println!("v={:?}, p={}, q={}, r={}", &v[0..r+1], p,q,r);
    let n1 = q-p+1; //2-0+1=3
    let n2 = r-q; //5-1-2=3
    
    let mut vl: Vec<i32> = Vec::new();
    let mut vr: Vec<i32> = Vec::new();
    
    for i in 0..n1 {
        vl.push(v[p+i]);
    }
    
    for i in 0..n2 {
        vr.push(v[q+1+i]);
    }
    
    println!("vl={:?}", &vl);
    println!("vr={:?}", &vr);
    
    let mut index_al = 0;
    let mut index_ar = 0;
    
    for k in p..r+1 {
        if vl[index_al] <= vr[index_ar] {
            v[k] = vl[index_al];
            println!("vl[{}]={}->v[{}], v={:?}", index_al, vl[index_al], k, &v[0..r+1]);
            index_al += 1;
            if index_al == n1 {
                for t in index_ar..n2 {
                    v[k+1+t-index_ar] = vr[t];
                }
                return;
            }
        } else {
            v[k] = vr[index_ar];
            println!("vr[{}]={}->v[{}], v={:?}", index_ar, vr[index_ar], k, &v[0..r+1]);
            index_ar += 1;
            if index_ar == n2 {
                for t in index_al..n1 {
                    v[k+1+t-index_al] = vl[t];
                }
                return;
            }
        }
    }
}

fn merge_sort(v: &mut Vec<i32>, p: usize, r: usize) {
    if p<r {
        let q = (p+r)/2;
        merge_sort(v, p, q);
        merge_sort(v, q+1, r);
        merge(v, p, q, r);
    }
}

fn main() {
    let mut v: Vec<i32> = vec![3,7,100,5,9,1,99,12,45,76,56,99,300,9,5,95];
    println!("v={:?}", &v);
    
    let len = v.len();
    let r = len-1;
    merge_sort(&mut v, 0, r);
    
    