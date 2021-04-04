fn insert_sort(v: &mut Vec<i32>, p: usize, r: usize) {
    println!("insert v={:?}, p={}, r={}", &v[p..r+1], p, r);
    for j in p+1..r+1 {//14..15
        let key = v[j];
        let mut i = j;

        while i>p && v[i-1]>key
        {
            v[i] = v[i-1];
            i = i-1;
        }
        v[i] = key;
    }
    println!("insert sorted v={:?}", &v[p..r+1]);
}

fn merge(v: &mut Vec<i32>, p: usize, q: usize, r: usize) {//p=0,q=2,r=5
    //println!("v={:?}, p={}, q={}, r={}", &v[p..r+1], p,q,r);
    let n1 = q-p+1;
    let n2 = r-q;

    let mut vl: Vec<i32> = Vec::new();
    let mut vr: Vec<i32> = Vec::new();

    for i in 0..n1 {
        vl.push(v[p+i]);
    }

    for i in 0..n2 {
        vr.push(v[q+1+i]);
    }

    vl.push(0xFFFF);
    vr.push(0xFFFF);

    //println!("vl={:?}", &vl);
    //println!("vr={:?}", &vr);

    let mut index_al = 0;
    let mut index_ar = 0;

    for k in p..r+1 {
        if vl[index_al] <= vr[index_ar] {
            v[k] = vl[index_al];
            //println!("vl[{}]={}->v[{}], v={:?}", index_al, vl[index_al], k, &v[0..r+1]);
            index_al += 1;
        } else {
            v[k] = vr[index_ar];
            //println!("vr[{}]={}->v[{}], v={:?}", index_ar, vr[index_ar], k, &v[0..r+1]);
            index_ar += 1;
        }
    }
}

fn insert_merge_sort(v: &mut Vec<i32>, p: usize, r: usize, k: usize, t: usize) {

    if p<r {
        if k<=t {
            insert_sort(v, p, r);
        } else {
                let q = (p+r)/2;
                insert_merge_sort(v, p, q, k, t+1);
                insert_merge_sort(v, q+1, r, k, t+1);
                merge(v, p, q, r);
        }
    }
}

fn main() {
    let mut v: Vec<i32> = vec![3,7,100,5,9,1,99,12,45,76,98,1,34,76,4,900,342];
    println!("v={:?}", &v);

    let len = v.len();
    let r = len-1;
    insert_merge_sort(&mut v, 0, r, 2, 0);

    println!("sorted v={:?}", &v);
}