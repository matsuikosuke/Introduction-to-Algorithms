fn find_max_cross_subarray(v: &Vec<i32>, low: usize , mid: usize , high: usize ) -> (usize , usize , i32 ) {
    let mut left_sum = -9999;
    let mut right_sum = -9999;
    let mut sum = 0;
    let mut max_left = 0;
    let mut max_right = 0;

    for i in (low..mid+1).rev() {
        sum += v[i];
        if sum > left_sum
        {
            left_sum = sum;
            max_left = i;
        }
    }

    sum = 0;

    for i in mid+1..high+1 {
        sum += v[i];
        if sum > right_sum
        {
            right_sum = sum;
            max_right = i;
        }
    }

    return (max_left, max_right, left_sum + right_sum);
}

fn find_maximum_subarray(v: &Vec<i32>, low: usize , high: usize ) -> (usize , usize , i32 ) {   
    if high == low {
        return (low, high, v[low]);
    }
    else
    {
        let mid = (low+high)/2;
        let (left_low, left_high, left_sum) = find_maximum_subarray(&v, low, mid);
        let (right_low, right_high, right_sum) = find_maximum_subarray(&v, mid+1, high);
        let (cross_low, cross_high, cross_sum) = find_max_cross_subarray(&v, low, mid, high);

        if left_sum >= right_sum && left_sum >= cross_sum {
            //println!("LEFT: {},{},{}", left_low, left_high, left_sum);
            return (left_low, left_high, left_sum);
        }
        else if right_sum >= left_sum && right_sum >= cross_sum {
            //println!("RIGHT: {},{},{}", right_low, right_high, right_sum);
            return (right_low, right_high, right_sum);
        }
        else {
            //println!("CROSS: {},{},{}", cross_low, cross_high, cross_sum);
            return (cross_low, cross_high, cross_sum);
        }
    }
}

fn main() {
    let v: Vec<i32> = vec![100, 113, 110, 85, 105, 102, 86, 63, 81, 101, 94, 106, 101, 79, 94, 90, 97];
    //let v: Vec<i32> = vec![1, 900, 110, 85, 105, 102, 86, 63, 81, 101, 94, 106, 101, 79, 94, 90, 97];
    //let v: Vec<i32> = vec![1, 113, 110, 85, 105, 102, 86, 63, 81, 101, 94, 106, 101, 79, 94, 90, 900];
    println!("v={:?}", &v);

    let mut dv: Vec<i32> = vec![];
    for j in 0..v.len()-1 {
        dv.push(v[j+1]-v[j])
    }
    println!("dv={:?}", dv);

    let (low, high, sum) = find_maximum_subarray(&dv, 0, dv.len()-1);
    println!("{}日目終了時に{}ドルで購入し、{}日目終了時に{}ドルで売却すると、最大の利益{}を得る", low, v[low], high+1, v[high+1], sum);

}