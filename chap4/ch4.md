# �L���̊T�v
�A���S���Y���C���g���_�N�V���� ��3�� �����ŁF���E�W��MIT���ȏ��̃A���S���Y����Rust�Ŏ������Ȃ���׋������̂ŁA���̌��ʂ𐮗����܂����B
�܂Ƃ߂����e�ɂ͊m���Ɍ�肪����Ǝv����̂ŁA�Q�Ƃɂ������ẮA���炩���߂��������������B

�l�I�ȕ׋������Ȃ̂ő��l�l�Ɍ��₷�����̂ɂȂ��Ă��܂���B
�܂��A�{�̒��쌠����邽�߂ɁA�{���Q�Ƃ��Ȃ���łȂ��Ɨ����ł��Ȃ��悤�ɏ����Ă��܂��B

�����4�͂ɂ��ĕ׋����܂����B

RUST�R�[�h�̓���͈ȉ��̃T�C�g�Ŋm�F�ł��܂��B
https://play.rust-lang.org/

# 4.1�@��������

## ��������헪
�z��̔C�ӂ�2�v�f��I�сA���̍������ő�ɂȂ�g�ݍ��킹��I�ԃA���S���Y���ł��B
�S�Ă̑g�ݍ��킹���v�Z���čő�l���Ō�܂Ŏc���̂���������헪�ł��B

```rust
fn find_max_round_robin(v: Vec<i32>) {
    let mut start = 0;
    let mut end = start;
    let mut max = 0;

    for j in 0..v.len()-1 {
        for i in j+1..v.len() {
            if (v[i] - v[j]) > max
            {
                start = j;
                end = i;
                max = v[i] - v[j];
            }
        }
    }

    println!("{}���ڏI������{}�h���ōw�����A{}���ڏI������{}�h���Ŕ��p����ƁA�ő�̗��v{}�𓾂�", start, v[start], end, v[end], max);
}

fn main() {
    let v: Vec<i32> = vec![100, 113, 110, 85, 105, 102, 86, 63, 81, 101, 94, 106, 101, 79, 94, 90, 97];
    //let v: Vec<i32> = vec![1, 900, 110, 85, 105, 102, 86, 63, 81, 101, 94, 106, 101, 79, 94, 90, 97];
    //let v: Vec<i32> = vec![1, 113, 110, 85, 105, 102, 86, 63, 81, 101, 94, 106, 101, 79, 94, 90, 900];
    println!("v={:?}", &v);

    find_max_round_robin(v);
}
```

## �ő啔���z��̑�������헪
�אڂ���v�f�̍�����������z������܂��B
���̍����z��̔C�ӂ̌��̘A�������v�f�̘a���ő�ɂȂ�g�ݍ��킹�I�ԃA���S���Y���ł��B

```rust
fn find_max_round_robin(v: Vec<i32>) {
    let mut start = 0;
    let mut end = start;
    let mut max = 0;
    let mut temp_max;
    
    for j in 0..v.len()-1 {
        temp_max = 0;
        for i in j..v.len() {
            temp_max += v[i];
            if temp_max > max
            {
                start = j;
                end = i+1;
                max = temp_max;
            }
        }
    }

    println!("{}���ڏI�����ɍw�����A{}���ڏI�����ɔ��p����ƁA�ő�̗��v{}�𓾂�", start, end, max);
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

    find_max_round_robin(dv);
}
```

�v�Z�ʂ́A�z��̗v�f����n�Ƃ��āA�����z��̗v�f����n-1�ɂȂ�܂��Bn-1����2�̗v�f��I�ԑS�Ă̑g�ݍ��킹���v�Z����̂ŗ��_��̌v�Z���� ${}_{n-1}C_2$ �ɔ�Ⴕ�܂��B

���ۂ̃A���S���Y���ɂ����ẮAfor���[�v�� $\sum_{i=1}^{n-2} i = (n-2)*(n-1)/2$ ��̌v�Z���s���̂�$\Theta(n^2)$ �ɂȂ�܂��B

## ��������
��قǂ̍����z����E�z��ƍ��z��̂Q�ɕ��������Ƃ��܂��B
���̏ꍇ�A�ő啔���z��͈ȉ��̂R�̂ǂꂩ�ɂȂ�܂��B

1. �ő啔���z��͉E�v�f�����ō\�������
2. �ő啔���z��͍��v�f�����ō\�������
3. �ő啔���z��͉E�z��ƍ��z��̗����ō\�������

�p�^�[���R�̏ꍇ�́A�ő啔���z��́A�z��𕪊����钆���v�f��K���܂݂܂��B
�����Œ����v�f����J�n���āA���z��̗v�f���P�����Ɉړ����Ȃ��瑫���Ă��������̍ő�l�ƁA�E�z��̗v�f���P���E�Ɉړ����Ȃ��瑫���Ă��������̍ő�l�̘a���ő啔���z��ɂȂ�܂��B
�J�n�v�f�͍��z��ōő�l���X�V�������̗v�f�ŁA�I���v�f�͉E�z��ōő�l���X�V�������̗v�f�ł��B


�����ăp�^�[���P�A�Q�A�R�̍ő啔���z������߂āA���̂R�̒��ōő�ɂȂ���̂��{���̍ő啔���z��ɂȂ�܂��B
�ł́A�E�z��ƍ��z��̍ő啔���z����ǂ�����ċ��߂邩�ƌ����΁A�e�z����E�z��ƍ��z��ɕ����āc�c�ƍċA�I�ɏ�L�Ɠ��l�̏������J��Ԃ��܂��B
����͕��������z��̗v�f�����P�ɂȂ�܂ŌJ��Ԃ���܂��B

```rust
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
    println!("{}���ڏI������{}�h���ōw�����A{}���ڏI������{}�h���Ŕ��p����ƁA�ő�̗��v{}�𓾂�", low, v[low], high+1, v[high+1], sum);

}
```

����find_maximum_subarray�̌v�Z��$T(n)$���l���܂��B
�ȉ��̕����͒萔���Ԃł��B

```rust
if high == low {
    return (low, high, v[low]);
}
```

```rust
let mid = (low+high)/2;
```

```rust
if left_sum >= right_sum && left_sum >= cross_sum {
    return (left_low, left_high, left_sum);
}
else if right_sum >= left_sum && right_sum >= cross_sum {
    return (right_low, right_high, right_sum);
}
else {
    return (cross_low, cross_high, cross_sum);
}
```

�ȉ��̊֐�find_max_cross_subarray��$\Theta(n)$�ł��B

```rust
let (cross_low, cross_high, cross_sum) = find_max_cross_subarray(&v, low, mid, high);
```

�ȉ��̊֐�find_maximum_subarray�͍ċA�I�ȏ����ŁA�v�f���͔����ɂȂ�A2����̂�$2T(n/2)$�ł��B

```rust
let (left_low, left_high, left_sum) = find_maximum_subarray(&v, low, mid);
let (right_low, right_high, right_sum) = find_maximum_subarray(&v, mid+1, high);
```

����đQ�����́A�v�Z�ʂɊ�^���Ȃ��萔���Ԃ̌v�Z�𖳎������

```math
\begin{eqnarray}
T(n) = 2T(n/2) + \Theta(n)
\end{eqnarray}
```

�ƂȂ�܂��B�}�X�^�[�@��p����ƁA���̌v�Z�̎�����$\Theta(n \lg n)$�ɂȂ�܂��B

## A4.1-1

�Ⴆ�Έȉ��̔z���p����ƁA�����z��̑S�Ă̗v�f�����ɂȂ�܂��B

```rust
let v: Vec<i32> = vec![900, 850, 800, 750, 700, 650, 600, 550, 500, 450, 400, 350, 300, 250, 200, 150, 100];
```

����ɂ��find_maximum_subarray�̕Ԃ�l�́A���low�v�f�ɂȂ�܂��B�J�n�v�f�ƏI���v�f��low��Ԃ��܂��B

## A4.1-2

[�ő啔���z��̑�������](## �ő啔���z��̑�������헪)�����Q�Ƃ��������B

## A4.1-3

�v�Z���Ԃ��r����v���O�������쐬���܂����B

```rust
use std::time::Instant;
//use quanta::Clock;
use rand::Rng;
use rand::distributions::Uniform;

fn find_max_round_robin(v: &Vec<i32>) {
    let mut start = 0;
    let mut end = 0;
    let mut max = 0;
    let mut temp_max;

    for j in 0..v.len()-1 {
        temp_max = 0;
        for i in j..v.len() {
            temp_max += v[i];
            if temp_max > max
            {
                start = j;
                end = i+1;
                max = temp_max;
            }
        }
    }

    //println!("{}���ڏI�����ɍw�����A{}���ڏI�����ɔ��p����ƁA�ő�̗��v{}�𓾂�", start, end, max);
}

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
    let mut rng = rand::thread_rng();

    let mut find_start;
    let mut find_stop;
    let mut round_start;
    let mut round_stop;
    
    let mut v: Vec<u32> = vec![100, 200];
    //println!("v={:?}", &v);
    
    let mut dv: Vec<i32> = vec![];
    for j in 0..v.len()-1 {
        dv.push(v[j+1] as i32-v[j] as i32);
    }
    //println!("dv={:?}", dv);

    for k in 0..30 {
        find_start = Instant::now();
        find_maximum_subarray(&dv, 0, dv.len()-1);
        //println!("{}���ڏI������{}�h���ōw�����A{}���ڏI������{}�h���Ŕ��p����ƁA�ő�̗��v{}�𓾂�", low, v[low], high+1, v[high+1], sum);
        find_stop = Instant::now();
        
        
        round_start = Instant::now();
        find_max_round_robin(&dv);
        round_stop = Instant::now();
        
        println!("�v�f��  :  ��������헪  :  �������� = {}  :  {:?}  :  {:?}", v.len(), round_stop.duration_since(round_start), find_stop.duration_since(find_start));
        
        /*if round_stop.duration_since(round_start) < find_stop.duration_since(find_start) {
            break;
        }*/
        
        v.push(rng.sample(Uniform::new(10u32, 300)));
        //println!("v={:?}", &v);
        dv.push(v[v.len()-1] as i32-v[v.len()-2] as i32);
        //println!("dv={:?}", dv);
    }
}
```


�ȉ��̂悤�Ȍ��ʂɂȂ�܂����B
�v�f����15���炢�܂ł́A�����_���Ȓl�ɂ���đ召���t�]���܂����B

```Console
�v�f��  :  ��������헪  :  �������� = 2  :  822ns  :  1.51?s
�v�f��  :  ��������헪  :  �������� = 3  :  1.446?s  :  2.849?s
�v�f��  :  ��������헪  :  �������� = 4  :  2.14?s  :  1.845?s
�v�f��  :  ��������헪  :  �������� = 5  :  2.088?s  :  2.333?s
�v�f��  :  ��������헪  :  �������� = 6  :  2.975?s  :  3.357?s
�v�f��  :  ��������헪  :  �������� = 7  :  3.31?s  :  3.885?s
�v�f��  :  ��������헪  :  �������� = 8  :  4.544?s  :  3.936?s
�v�f��  :  ��������헪  :  �������� = 9  :  5.102?s  :  4.886?s
�v�f��  :  ��������헪  :  �������� = 10  :  6.477?s  :  5.598?s
�v�f��  :  ��������헪  :  �������� = 11  :  8.212?s  :  6.14?s
�v�f��  :  ��������헪  :  �������� = 12  :  9.69?s  :  7.486?s
�v�f��  :  ��������헪  :  �������� = 13  :  11.617?s  :  8.533?s
�v�f��  :  ��������헪  :  �������� = 14  :  12.691?s  :  8.582?s
�v�f��  :  ��������헪  :  �������� = 15  :  12.828?s  :  8.71?s
�v�f��  :  ��������헪  :  �������� = 16  :  16.715?s  :  10.895?s
�v�f��  :  ��������헪  :  �������� = 17  :  17.261?s  :  10.473?s
�v�f��  :  ��������헪  :  �������� = 18  :  20.797?s  :  12.249?s
�v�f��  :  ��������헪  :  �������� = 19  :  22.781?s  :  13.097?s
�v�f��  :  ��������헪  :  �������� = 20  :  24.218?s  :  14.101?s
�v�f��  :  ��������헪  :  �������� = 21  :  26.412?s  :  14.086?s
�v�f��  :  ��������헪  :  �������� = 22  :  29.178?s  :  15.375?s
�v�f��  :  ��������헪  :  �������� = 23  :  32.537?s  :  16.605?s
�v�f��  :  ��������헪  :  �������� = 24  :  33.34?s  :  17.281?s
�v�f��  :  ��������헪  :  �������� = 25  :  35.704?s  :  17.682?s
�v�f��  :  ��������헪  :  �������� = 26  :  40.877?s  :  19.403?s
�v�f��  :  ��������헪  :  �������� = 27  :  40.812?s  :  20.598?s
�v�f��  :  ��������헪  :  �������� = 28  :  44.214?s  :  20.364?s
�v�f��  :  ��������헪  :  �������� = 29  :  42.266?s  :  21.559?s
�v�f��  :  ��������헪  :  �������� = 30  :  51.377?s  :  21.642?s
�v�f��  :  ��������헪  :  �������� = 31  :  53.039?s  :  22.858?s
```

## A4.1-4

���܂ł̃A���S���Y������̕����z������Ƃ��ĔF�߂Ȃ��A�Ɩ�蕶�ɂ͏����Ă���܂������A�Ӗ����悭������܂���ł����B
�����z�񂪋�ł����Ȃ����ʂ��o���Ă���悤�Ɍ����邩��ł��B

������Ȃ��̂ŉ𓚕ۗ��ł��B

## A4.1-5

$A[0 \dots j]$�܂ł̍ő啔���z�񂪕������Ă���ꍇ�A$A[0 \dots j+1]$�̍ő啔���z��Ɣ�r���܂��B

�ŏ��̗v�f���珇�Ԃɑ����āA�b��I�ȕ����z��̘a�����߂܂��B
�����z��̘a�������_�̍ő�l�𒴂����ꍇ�A�b��I�ȕ����z��̘a���ő�l�Ƃ��čX�V���܂��B�܂��J�n�C���f�b�N�X�A�I���C���f�b�N�X���X�V���܂��B

�����̊J�n�����ǂ̂悤�Ɏ擾���邩�ɍH�v���K�v�ł��B
�b��I�ȕ����z��̑��a�����ɂȂ������́A��������J�n���ł���Ɣ��肵�A�����z��̑��a�����Z�b�g���A�b��J�n�C���f�b�N�X���X�V���܂��B

```rust
fn find_max_round_robin(v: Vec<i32>) {
    let mut start = 0;
    let mut end = start;
    let mut max = 0;
    let mut temp_max = 0;
    let mut temp_start = 0;
    
    for j in 0..v.len() {
        temp_max += v[j];
        
        if temp_max > max
        {
            start = temp_start;
            end = j+1;
            max = temp_max;
        }
        else
        {
            if temp_max < 0
            {
                temp_max = 0;
                temp_start = j+1;
            }
        }
    }

    println!("{}���ڏI�����ɍw�����A{}���ڏI�����ɔ��p����ƁA�ő�̗��v{}�𓾂�", start, end, max);
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

    find_max_round_robin(dv);
}
```

# 4.2�@Strassen�̃A���S���Y��

## �s��̗v�f�̑�������ώZ

�s��͊|���鏇�ԂŌ��ʂ��ς��̂ł����ӂ��������B
�܂��A�ŏ��̍s��̗񐔂Ǝ��̍s��̍s������v���Ă��Ȃ��ƌv�Z�͐������܂���B

```rust
type Matrix = Vec<Vec<i32>>;

fn matrix_print(a: &Matrix) {
    println!("{}X{}�s��",a.len(), a[0].len());
    for i in a {
        println!("{:?}",i);
    }
}

fn matrix_multiple(a: &Matrix, b: &Matrix) -> Matrix {
    let c_row = a.len();
    let n = b.len();
    let c_col = b[0].len();
    
    let mut c = vec![vec![0;c_col];c_row];

    for i in 0..c_row {
        for j in 0..c_col {
            c[i][j] = 0;
            for k in 0..n {
                c[i][j] = c[i][j] + a[i][k]*b[k][j];
            }
        }
    }

    c
}

fn main() {
    //let a: Matrix = vec![vec![1,2,3], vec![4,5,6], vec![7,8,9]];
    //let b: Matrix = vec![vec![10,11,12], vec![13,14,15], vec![16,17,18]];
    let a: Matrix = vec![vec![1,2,3,4,5], vec![6,7,8,9,10], vec![11,12,13,141,15]];
    let b: Matrix = vec![vec![16,17], vec![18,19], vec![20,21], vec![22,23], vec![24,25]];
    matrix_print(&a);
    matrix_print(&b);
    
    matrix_print(&matrix_multiple(&a, &b));
}
```

������Rust�ɂ��s��v�Z�͈ȉ����Q�Ƃ��܂����B

[�s��̑����Z�Ŋw��Rust�̃G���[�n���h�����O](https://qiita.com/es__135/items/5a4121de7f5eaa3f4973)

ndarray���g�p����̂���ʓI���Ǝv���܂����Andarray���g���ƍs��̐ς�����1�s�Ŏ��s�ł��܂��B
�֗��ŗǂ��̂ł����A����ł̓A���S���Y���̗��K�ɂȂ�Ȃ��̂ō����vec�̑��d����q��p���܂����B

## �����s��ւ̕���

�s��𕔕��s��ɕ������鎞�́A�Y�����̌v�Z�Ŕ͈͂��߂邱�Ƃ���������Ă��܂������A�ȉ��ł͍s��̗v�f���R�s�[���Ă��܂��B
����ɂ��$\Theta(n^2)$�̒萔�{�A�v�Z���Ԃ������܂��B


```rust
type Matrix = Vec<Vec<i32>>;

fn matrix_print(a: &Matrix) {
    println!("{}X{}�s��",a.len(), a[0].len());
    for i in a {
        println!("{:?}",i);
    }
}

fn square_matrix_multiple(a: &Matrix, b: &Matrix) -> Matrix {
    let n: usize = a.len();
    let mut c = vec![vec![0;n];n];
    
    for i in 0..n {
        for j in 0..n {
            c[i][j] = 0;
            for k in 0..n {
                c[i][j] = c[i][j] + a[i][k]*b[k][j];
            }
        }
    }

    c
}

fn square_matrix_multiple_recursive(a: &Matrix, b: &Matrix) -> Matrix {
    let n: usize = a.len();
    let mut c = vec![vec![0;n];n];
    
    if 1 == n {
        c[0][0] = a[0][0]*b[0][0];
    }
    else
    {
        let mut a11 = vec![vec![0;n/2];n/2];
        let mut a12 = vec![vec![0;n/2];n/2];
        let mut a21 = vec![vec![0;n/2];n/2];
        let mut a22 = vec![vec![0;n/2];n/2];
        
        let mut b11 = vec![vec![0;n/2];n/2];
        let mut b12 = vec![vec![0;n/2];n/2];
        let mut b21 = vec![vec![0;n/2];n/2];
        let mut b22 = vec![vec![0;n/2];n/2];
        
        for i in 0..n/2 {
            for j in 0..n/2 {
                a11[i][j] = a[i][j];
                a12[i][j] = a[i][n/2+j];
                a21[i][j] = a[n/2+i][j];
                a22[i][j] = a[n/2+i][n/2+j];
                
                b11[i][j] = b[i][j];
                b12[i][j] = b[i][n/2+j];
                b21[i][j] = b[n/2+i][j];
                b22[i][j] = b[n/2+i][n/2+j];
            }
        }
        /*matrix_print(&a11);
        matrix_print(&a12);
        matrix_print(&a21);
        matrix_print(&a22);*/
        
        let mut c11_1st = vec![vec![0;n/2];n/2];
        let mut c12_1st = vec![vec![0;n/2];n/2];
        let mut c21_1st = vec![vec![0;n/2];n/2];
        let mut c22_1st = vec![vec![0;n/2];n/2];
        
        let mut c11_2nd = vec![vec![0;n/2];n/2];
        let mut c12_2nd = vec![vec![0;n/2];n/2];
        let mut c21_2nd = vec![vec![0;n/2];n/2];
        let mut c22_2nd = vec![vec![0;n/2];n/2];
        
        let mut c11 = vec![vec![0;n/2];n/2];
        let mut c12 = vec![vec![0;n/2];n/2];
        let mut c21 = vec![vec![0;n/2];n/2];
        let mut c22 = vec![vec![0;n/2];n/2];
        
        c11_1st = square_matrix_multiple(&a11, &b11);
        c11_2nd = square_matrix_multiple(&a12, &b21);
        c12_1st = square_matrix_multiple(&a11, &b12);
        c12_2nd = square_matrix_multiple(&a12, &b22);
        c21_1st = square_matrix_multiple(&a21, &b11);
        c21_2nd = square_matrix_multiple(&a22, &b21);
        c22_1st = square_matrix_multiple(&a21, &b12);
        c22_2nd = square_matrix_multiple(&a22, &b22);
        
        
        for i in 0..n/2 {
            for j in 0..n/2 {
                c11[i][j] = c11_1st[i][j]+c11_2nd[i][j];
                c12[i][j] = c12_1st[i][j]+c12_2nd[i][j];
                c21[i][j] = c21_1st[i][j]+c21_2nd[i][j];
                c22[i][j] = c22_1st[i][j]+c22_2nd[i][j];
            }
        }
        
        for i in 0..n/2 {
            for j in 0..n/2 {
                c[i][j] = c11[i][j];
                c[i][n/2+j] = c12[i][j];
                c[n/2+i][j] = c21[i][j];
                c[n/2+i][n/2+j] = c22[i][j];
            }
        }
    
    }
    c
}

fn main() {
    let a: Matrix = vec![vec![1,2,3,4,5,6], vec![7,8,9,10,11,12], vec![13,14,15,16,17,18], vec![19,20,21,22,23,24], vec![25,26,27,28,29,30], vec![31,32,33,34,35,36]];
    let b: Matrix = vec![vec![37,38,39,40,41,42], vec![43,44,45,46,47,48], vec![49,50,51,52,53,54], vec![55,56,57,58,59,60], vec![61,62,63,64,65,66], vec![67,68,69,70,71,72], ];
    matrix_print(&a);
    matrix_print(&b);
    
    matrix_print(&square_matrix_multiple_recursive(&a, &b));
}
```

���ɂ��̃A���S���Y���̌v�Z��$T(n)$�����߂܂��B

$n=1$�̎��͐ώZ��1�񂾂����s����̂�$T(1) = \Theta(1)$�ł��B

```rust
c[0][0] = a[0][0]*b[0][0];
```

$n>1$�̎���$n/2$��̌v�Z���s��for���[�v��2��J��Ԃ��ӏ��̌v�Z�ʂ͑S��$\Theta(n^2/4)$�ɂȂ�܂��B
$\Theta$�̈�����萔�{���Ă��v�Z�̎����͕ς��Ȃ��̂ŁA$\Theta(n^2) = \Theta(n^2/4)$�ł���A�܂�$\Theta(n^2) = \Theta(cn^2)$�ł�����̂ŁA�܂Ƃ߂�$\Theta(n^2)$�ɏW��ł��܂��B

�ċA�Ăяo����$T(n/2)$��8����s����܂��B
�ċA�Ăяo����萔�{����Όv�Z�̎����ɉe����^����̂ŁA�萔���v�Z�ʂɊ܂߂Ȃ��Ƃ����܂���B

����Čv�Z�ʂ�

```math
\begin{eqnarray}
T(n) &=& \left \{
\begin{array}{l}
\Theta(1)  \ \ \ \ \ \ \ \ \ \ \ \ \ \ \ \ \ \ \ \ \ \ \ \ \ \ (n=1�̏ꍇ)\\
8T(n/2) + \Theta(n^2) \ \ \ \ \ (n>1�̏ꍇ)
\end{array}
\right.
\end{eqnarray}
```

�ƂȂ�܂��B

## Strassen�̕��@

����͍ċA�����̉񐔂�8�񂩂�7��Ɍ��炷���ƂŁA�v�Z�ʂ����炷�A���S���Y���ł��B

�܂��͐�قǓ��l��2�̍s���8�̕����s��ɕ������܂��B
���ɁA����8�̕����s�񂩂�X��10�̍s��𐶐����܂��B
����10�̍s���p����ƍċA�Ăяo����7��ōς݂܂��B

```rust

type Matrix = Vec<Vec<i32>>;

fn matrix_print(a: &Matrix) {
    println!("{}X{}�s��",a.len(), a[0].len());
    for i in a {
        println!("{:?}",i);
    }
}


fn square_matrix_multiple(a: &Matrix, b: &Matrix) -> Matrix {
    let n: usize = a.len();
    let mut c = vec![vec![0;n];n];
    
    for i in 0..n {
        for j in 0..n {
            c[i][j] = 0;
            for k in 0..n {
                c[i][j] = c[i][j] + a[i][k]*b[k][j];
            }
        }
    }

    c
}

fn square_matrix_multiple_recursive(a: &Matrix, b: &Matrix) -> Matrix {
    let n: usize = a.len();
    let mut c = vec![vec![0;n];n];
    
    if 1 == n {
        c[0][0] = a[0][0]*b[0][0];
    }
    else
    {
        let mut a11 = vec![vec![0;n/2];n/2];
        let mut a12 = vec![vec![0;n/2];n/2];
        let mut a21 = vec![vec![0;n/2];n/2];
        let mut a22 = vec![vec![0;n/2];n/2];
        
        let mut b11 = vec![vec![0;n/2];n/2];
        let mut b12 = vec![vec![0;n/2];n/2];
        let mut b21 = vec![vec![0;n/2];n/2];
        let mut b22 = vec![vec![0;n/2];n/2];
        
        for i in 0..n/2 {
            for j in 0..n/2 {
                a11[i][j] = a[i][j];
                a12[i][j] = a[i][n/2+j];
                a21[i][j] = a[n/2+i][j];
                a22[i][j] = a[n/2+i][n/2+j];
                
                b11[i][j] = b[i][j];
                b12[i][j] = b[i][n/2+j];
                b21[i][j] = b[n/2+i][j];
                b22[i][j] = b[n/2+i][n/2+j];
            }
        }
        /*matrix_print(&a11);
        matrix_print(&a12);
        matrix_print(&a21);
        matrix_print(&a22);*/
        
        let mut s1 = vec![vec![0;n/2];n/2];
        let mut s2 = vec![vec![0;n/2];n/2];
        let mut s3 = vec![vec![0;n/2];n/2];
        let mut s4 = vec![vec![0;n/2];n/2];
        let mut s5 = vec![vec![0;n/2];n/2];
        let mut s6 = vec![vec![0;n/2];n/2];
        let mut s7 = vec![vec![0;n/2];n/2];
        let mut s8 = vec![vec![0;n/2];n/2];
        let mut s9 = vec![vec![0;n/2];n/2];
        let mut s10 = vec![vec![0;n/2];n/2];
        
        for i in 0..n/2 {
            for j in 0..n/2 {
                s1[i][j] = b12[i][j]-b22[i][j];
                s2[i][j] = a11[i][j]+a12[i][j];
                s3[i][j] = a21[i][j]+a22[i][j];
                s4[i][j] = b21[i][j]-b11[i][j];
                s5[i][j] = a11[i][j]+a22[i][j];
                s6[i][j] = b11[i][j]+b22[i][j];
                s7[i][j] = a12[i][j]-a22[i][j];
                s8[i][j] = b21[i][j]+b22[i][j];
                s9[i][j] = a11[i][j]-a21[i][j];
                s10[i][j] = b11[i][j]+b12[i][j];
            }
        }
        
        let mut p1 = vec![vec![0;n/2];n/2];
        let mut p2 = vec![vec![0;n/2];n/2];
        let mut p3 = vec![vec![0;n/2];n/2];
        let mut p4 = vec![vec![0;n/2];n/2];
        let mut p5 = vec![vec![0;n/2];n/2];
        let mut p6 = vec![vec![0;n/2];n/2];
        let mut p7 = vec![vec![0;n/2];n/2];
        
        p1 = square_matrix_multiple(&a11, &s1);
        p2 = square_matrix_multiple(&s2, &b22);
        p3 = square_matrix_multiple(&s3, &b11);
        p4 = square_matrix_multiple(&a22, &s4);
        p5 = square_matrix_multiple(&s5, &s6);
        p6 = square_matrix_multiple(&s7, &s8);
        p7 = square_matrix_multiple(&s9, &s10);
        
        let mut c11 = vec![vec![0;n/2];n/2];
        let mut c12 = vec![vec![0;n/2];n/2];
        let mut c21 = vec![vec![0;n/2];n/2];
        let mut c22 = vec![vec![0;n/2];n/2];
        
        for i in 0..n/2 {
            for j in 0..n/2 {
                c11[i][j] = p5[i][j]+p4[i][j]-p2[i][j]+p6[i][j];
                c12[i][j] = p1[i][j]+p2[i][j];
                c21[i][j] = p3[i][j]+p4[i][j];
                c22[i][j] = p5[i][j]+p1[i][j]-p3[i][j]-p7[i][j];
            }
        }
        
        for i in 0..n/2 {
            for j in 0..n/2 {
                c[i][j] = c11[i][j];
                c[i][n/2+j] = c12[i][j];
                c[n/2+i][j] = c21[i][j];
                c[n/2+i][n/2+j] = c22[i][j];
            }
        }
    
    }
    c
}

fn main() {
    let a: Matrix = vec![vec![1,2,3,4,5,6], vec![7,8,9,10,11,12], vec![13,14,15,16,17,18], vec![19,20,21,22,23,24], vec![25,26,27,28,29,30], vec![31,32,33,34,35,36]];
    let b: Matrix = vec![vec![37,38,39,40,41,42], vec![43,44,45,46,47,48], vec![49,50,51,52,53,54], vec![55,56,57,58,59,60], vec![61,62,63,64,65,66], vec![67,68,69,70,71,72], ];
    matrix_print(&a);
    matrix_print(&b);
    
    matrix_print(&square_matrix_multiple_recursive(&a, &b));
}
```

```
[1197, 1218, 1239, 1260, 1281, 1302]
[3069, 3126, 3183, 3240, 3297, 3354]
[4941, 5034, 5127, 5220, 5313, 5406]
[6813, 6942, 7071, 7200, 7329, 7458]
[8685, 8850, 9015, 9180, 9345, 9510]
[10557, 10758, 10959, 11160, 11361, 11562]
```

## A4.2-1

```math
\begin{eqnarray}
S_1 &=& 8-2 = 6 \\
S_2 &=& 1+3 = 4 \\
S_3 &=& 7+5 = 12 \\
S_4 &=& 4-6 = -2 \\
S_5 &=& 1+5 = 6 \\
S_6 &=& 6+2 = 8 \\
S_7 &=& 3-5 = -2 \\
S_8 &=& 4+2 = 6 \\
S_9 &=& 1-7 = -6 \\
S_{10} &=& 6+8 = 14
\end{eqnarray}
```

```math
\begin{eqnarray}
P_1 &=& 1*6 = 6 \\
P_2 &=& 4*2 = 8 \\
P_3 &=& 12*6 = 72 \\
P_4 &=& 5*(-2) = -10 \\
P_5 &=& 6*8 = 48 \\
P_6 &=& -2*6 = -12 \\
P_7 &=& -6*14 = -84
\end{eqnarray}
```

```math
\begin{eqnarray}
C_{11} &=& 48-10-8-12 = 18 \\
C_{12} &=& 6+8 = 14 \\
C_{21} &=& 72-10 = 62 \\
C_{22} &=& 48+6-72+84 = 66
\end{eqnarray}
```

�����

```math
\begin{eqnarray}
\begin{pmatrix} 1 & 3 \\ 7 & 5 \end{pmatrix}
\begin{pmatrix} 6 & 8 \\ 4 & 2 \end{pmatrix}
=
\begin{pmatrix} 18 & 14 \\ 62 & 66 \end{pmatrix}
\end{eqnarray}
```

�����܂�܂��B

## A4.2-2

[Strassen�̕��@](## Strassen�̕��@)�����Q�Ƃ��������B

## A4.2-3

�^����ꂽ���s��̍s��������������𔻕ʂ��܂��B
�����Ȃ�΁A���܂łƓ����悤�ɉ����܂��B
��Ȃ�΂S�̕����s��A$2n-1 \times 2n-1$�s��A$1 \times 2n-1$�s��A$2n-1 \times 1$�s��A$1 \times 1$�s��ɕ����܂��B
������$2n-1 \times 2n-1$�s��ɂ��č��܂łƓ����悤�ɋ��߁A�c��̍s��𑍓�����헪�ŉ����܂��B

```rust
type Matrix = Vec<Vec<i32>>;

fn matrix_print(a: &Matrix) {
    println!("{}X{}�s��",a.len(), a[0].len());
    for i in a {
        println!("{:?}",i);
    }
}

fn matrix_multiple(a: &Matrix, b: &Matrix) -> Matrix {
    let c_row = a.len();
    let n = b.len();
    let c_col = b[0].len();

    let mut c = vec![vec![0;c_col];c_row];

    for i in 0..c_row {
        for j in 0..c_col {
            c[i][j] = 0;
            for k in 0..n {
                c[i][j] = c[i][j] + a[i][k]*b[k][j];
            }
        }
    }

    c
}

fn square_matrix_multiple(a: &Matrix, b: &Matrix) -> Matrix {
    let n: usize = a.len();
    let mut c = vec![vec![0;n];n];
    
    for i in 0..n {
        for j in 0..n {
            c[i][j] = 0;
            for k in 0..n {
                c[i][j] = c[i][j] + a[i][k]*b[k][j];
            }
        }
    }

    c
}

fn square_matrix_multiple_recursive(a: &Matrix, b: &Matrix) -> Matrix {
    let n: usize = a.len();
    let mut c = vec![vec![0;n];n];
    
    if 1 == n {
        c[0][0] = a[0][0]*b[0][0];
    }
    else
    {
        let mut a11 = vec![vec![0;n/2];n/2];
        let mut a12 = vec![vec![0;n/2];n/2];
        let mut a21 = vec![vec![0;n/2];n/2];
        let mut a22 = vec![vec![0;n/2];n/2];
        
        let mut b11 = vec![vec![0;n/2];n/2];
        let mut b12 = vec![vec![0;n/2];n/2];
        let mut b21 = vec![vec![0;n/2];n/2];
        let mut b22 = vec![vec![0;n/2];n/2];
        
        for i in 0..n/2 {
            for j in 0..n/2 {
                a11[i][j] = a[i][j];
                a12[i][j] = a[i][n/2+j];
                a21[i][j] = a[n/2+i][j];
                a22[i][j] = a[n/2+i][n/2+j];
                
                b11[i][j] = b[i][j];
                b12[i][j] = b[i][n/2+j];
                b21[i][j] = b[n/2+i][j];
                b22[i][j] = b[n/2+i][n/2+j];
            }
        }
        /*matrix_print(&a11);
        matrix_print(&a12);
        matrix_print(&a21);
        matrix_print(&a22);*/
        
        let mut c11_1st = vec![vec![0;n/2];n/2];
        let mut c12_1st = vec![vec![0;n/2];n/2];
        let mut c21_1st = vec![vec![0;n/2];n/2];
        let mut c22_1st = vec![vec![0;n/2];n/2];
        
        let mut c11_2nd = vec![vec![0;n/2];n/2];
        let mut c12_2nd = vec![vec![0;n/2];n/2];
        let mut c21_2nd = vec![vec![0;n/2];n/2];
        let mut c22_2nd = vec![vec![0;n/2];n/2];
        
        let mut c11 = vec![vec![0;n/2];n/2];
        let mut c12 = vec![vec![0;n/2];n/2];
        let mut c21 = vec![vec![0;n/2];n/2];
        let mut c22 = vec![vec![0;n/2];n/2];
        
        c11_1st = square_matrix_multiple(&a11, &b11);
        c11_2nd = square_matrix_multiple(&a12, &b21);
        c12_1st = square_matrix_multiple(&a11, &b12);
        c12_2nd = square_matrix_multiple(&a12, &b22);
        c21_1st = square_matrix_multiple(&a21, &b11);
        c21_2nd = square_matrix_multiple(&a22, &b21);
        c22_1st = square_matrix_multiple(&a21, &b12);
        c22_2nd = square_matrix_multiple(&a22, &b22);
        
        
        for i in 0..n/2 {
            for j in 0..n/2 {
                c11[i][j] = c11_1st[i][j]+c11_2nd[i][j];
                c12[i][j] = c12_1st[i][j]+c12_2nd[i][j];
                c21[i][j] = c21_1st[i][j]+c21_2nd[i][j];
                c22[i][j] = c22_1st[i][j]+c22_2nd[i][j];
            }
        }
        
        for i in 0..n/2 {
            for j in 0..n/2 {
                c[i][j] = c11[i][j];
                c[i][n/2+j] = c12[i][j];
                c[n/2+i][j] = c21[i][j];
                c[n/2+i][n/2+j] = c22[i][j];
            }
        }
    
    }
    c
}

fn main() {
    //let a: Matrix = vec![vec![1,2,3,4,5,6], vec![7,8,9,10,11,12], vec![13,14,15,16,17,18], vec![19,20,21,22,23,24], vec![25,26,27,28,29,30], vec![31,32,33,34,35,36]];
    //let b: Matrix = vec![vec![37,38,39,40,41,42], vec![43,44,45,46,47,48], vec![49,50,51,52,53,54], vec![55,56,57,58,59,60], vec![61,62,63,64,65,66], vec![67,68,69,70,71,72], ];
    let a: Matrix = vec![vec![1,2,3,4,5], vec![7,8,9,10,11], vec![13,14,15,16,17], vec![19,20,21,22,23], vec![25,26,27,28,29]];
    let b: Matrix = vec![vec![37,38,39,40,41], vec![43,44,45,46,47], vec![49,50,51,52,53], vec![55,56,57,58,59], vec![61,62,63,64,65]];
    
    matrix_print(&a);
    matrix_print(&b);
    
    let n = a.len();
    
    if 0 == n%2
    {
        matrix_print(&square_matrix_multiple_recursive(&a, &b));
    } else {
        let mut c = vec![vec![0;n];n];
    
        let mut a11 = vec![vec![0;n-1];n-1];
        let mut a12 = vec![vec![0;1];n-1];
        let mut a21 = vec![vec![0;n-1];1];
        let mut a22 = vec![vec![0;1];1];
        let mut b11 = vec![vec![0;n-1];n-1];
        let mut b12 = vec![vec![0;1];n-1];
        let mut b21 = vec![vec![0;n-1];1];
        let mut b22 = vec![vec![0;1];1];
        
        for i in 0..n-1 {
            for j in 0..n-1 {
                a11[i][j] = a[i][j];
                b11[i][j] = b[i][j];
            }
        }
        
        for i in 0..n-1 {
            a12[i][0] = a[i][n-1];
            b12[i][0] = b[i][n-1];
        }
        
        for i in 0..n-1 {
            a21[0][i] = a[n-1][i];
            b21[0][i] = b[n-1][i];
        }
        
        a22[0][0] = a[n-1][n-1];
        b22[0][0] = b[n-1][n-1];
        
        
    
        let mut c11_1st = vec![vec![0;n-1];n-1];
        let mut c12_1st = vec![vec![0;1];n-1];
        let mut c21_1st = vec![vec![0;n-1];1];
        let mut c22_1st = vec![vec![0;1];1];
        
        let mut c11_2nd = vec![vec![0;n-1];n-1];
        let mut c12_2nd = vec![vec![0;1];n-1];
        let mut c21_2nd = vec![vec![0;n-1];1];
        let mut c22_2nd = vec![vec![0;1];1];
        
        let mut c11 = vec![vec![0;n-1];n-1];
        let mut c12 = vec![vec![0;1];n-1];
        let mut c21 = vec![vec![0;n-1];1];
        let mut c22 = vec![vec![0;1];1];
        
        c11_1st = square_matrix_multiple(&a11, &b11);
        c11_2nd = matrix_multiple(&a12, &b21);
        c12_1st = matrix_multiple(&a11, &b12);
        c12_2nd = matrix_multiple(&a12, &b22);
        c21_1st = matrix_multiple(&a21, &b11);
        c21_2nd = matrix_multiple(&a22, &b21);
        c22_1st = matrix_multiple(&a21, &b12);
        c22_2nd = matrix_multiple(&a22, &b22);
        
        
        
        for i in 0..n-1 {
            for j in 0..n-1 {
                c11[i][j] = c11_1st[i][j]+c11_2nd[i][j];
            }
        }
        
        for i in 0..n-1 {
            c12[i][0] = c12_1st[i][0]+c12_2nd[i][0];
        }
        
        for i in 0..n-1 {
            c21[0][i] = c21_1st[0][i]+c21_2nd[0][i];
        }
        
        c22[0][0] = c22_1st[0][0]+c22_2nd[0][0];
        
        for i in 0..n-1 {
            for j in 0..n-1 {
                c[i][j] = c11[i][j];
            }
        }
        
        for i in 0..n-1 {
            c[i][n-1] = c12[i][0];
        }
        
        for i in 0..n-1 {
            c[n-1][i] = c21[0][i];
        }
        
        c[n-1][n-1] = c22[0][0];
        
        matrix_print(&c);
    }
    
}
```

## A4.2-7

```math
\begin{eqnarray}
(a+ib)(c+di) 
&=& ac-bd+i(cb+ad) \\
&=& ac+ad-ad-bd + i(cb+ac-ac+ad) \\
&=& a(c+d)-d(a+b) + i(a(c+d)+c(b-a))
\end{eqnarray}
```

�����$a(c+d)$��$d(a+b)$��$c(b-a)$��3��̏�Z�ɂ��v�Z�����s�ł��܂��B


# 4.3 �u�������@

## A4.3-1

```math
\begin{eqnarray}
T(n) &\leq& c(n-1)^2 + n = cn^2 + (1-2c)n + 1 \\
&\leq& cn^2 + 1 - 2c + 1 \\
&\leq& cn^2 
\end{eqnarray}
```

$ c \geq 1$ �ɂȂ�悤�ɑI�ԂƁA$1-2c < 0$��$n \geq 1$�Ȃ̂� $(1-2c)n \leq 1-2c$ �ɂȂ�܂��B
�X��$1-c \leq 0$�Ȃ̂ōŌ�̎��W�J���������܂��B

## A4.3-2
����� $T(n) \leq cn$ ���������Ȃ��ꍇ�Ȃ̂ŁA$T(n) \leq cn -d$�������܂��B

$\lceil a/b \rceil \leq (a+b-1)/b$���

```math
\begin{eqnarray}
T(n) &\leq& c\lg (\lceil n/2 \rceil) -d + 1 \\
&\leq& c \lg (n+1)/2 -d + 1 \\
&\leq& c \lg (1+k)n/2 -d + 1 
= c\lg (1+k) + c \lg n - c -d + 1
\end{eqnarray}
```

������$d=1$�Ƃ��܂��B
�܂��A$c \lg (1+k) - c \leq c \lg(1+k)$���

```math
\begin{eqnarray}
T(n) &\leq& c\lg (1+k) + c \lg n
\end{eqnarray}
```

�Q�����𖞂����ɂ́A$c \lg (1+k) = -1$ �ɂȂ�΂����̂ŁA$c=t,1+k=1/2^{1/t}$ �Ƃ�����

```math
\begin{eqnarray}
t \lg 1/2^{1/t} = - t \frac{1}{t} = -1  
\end{eqnarray}
```

�ƂȂ�܂��B�����$c \geq 1$�𖞂����C�ӂ̐������I�ׂ܂��B$c=1$�ł�$c=3$�ł�$c=645363728$�ł��ł��B
�����$T(n) \leq 645363728 \lg n -1$���������܂��B


## A4.3-3

$\lfloor a/b \rfloor \geq (a-b+1)/b$���


```math
\begin{eqnarray}
T(n) &\geq& 2(c\lfloor n/2 \rfloor \lg (\lfloor n/2 \rfloor) + n \\
&\geq& 2(c \frac{n-1}{2} \lg\frac{n-1}{2}) + n \\
&=&c(n-1) \lg(n-1) - c(n-1) + n \\
&=& cn \lg n - c \lg n + c(n-1) \lg ((n-1)/n) -c(n-1) + n
\end{eqnarray}
```

������


```math
\begin{eqnarray}
0 \leq - c \lg n + c(n-1) \lg ((n-1)/n) -c(n-1) + n
\end{eqnarray}
```

��������΁A$T(n) \geq cn \lg n$����������$T(n) = \Omega(n \lg n)$���ؖ��ł��܂��B

$0 < c \leq 1/3$�Ƃ���$n \geq 2$�ɂ����Đ��l�v�Z�����

```rust
extern crate num;
use num::Float;

fn terms(c: f32, n: f32) {
    let x = -c*Float::log(n, 2.0) +c*(n-1.0)*Float::log((n-1.0)/n, 2.0) -c*(n-1.0)+n;
    println!("{}", x) 
}

fn main() {
    //println!("{}", Float::ln(2.0));
    //println!("{}", Float::log(2.0, 2.0));
    terms(0.333,2.0);
}
```

```
1.0009999
```

���0�ȏ�ł��B
�^����$n$�̑����ɑ΂��ĒP����������̂ŁA�^���͏��0�ȏ�ɂȂ�A$T(n) = \Omega(n \lg n)$���������܂��B

## A4.3-4

�܂�$T(n) \leq cn\lg n -d$�Ɖ��肵�܂��B
������$c \geq 1$�Ƃ��܂��B

�܂��A$T(1)=1$�𖞂����Ȃ��Ƃ����Ȃ��̂�$d = -1$�Ƃ���$T(n) \leq cn\lg n +1$�łȂ��Ƃ����܂���B

$\lfloor a/b \rfloor \leq a/b$���

```math
\begin{eqnarray}
T(n) &\leq& 2(c \lfloor n/2 \rfloor \lg (\lfloor n/2 \rfloor)) +2 + n \\
&\leq& c n \lg (n/2) -2d + n = cn \lg n -cn +2 + n 
\end{eqnarray}
```

������$ -cn +2 + n  \leq 1$��������΁A$T(n) \leq cn\lg n +1$���ؖ��ł��܂��B
����$c=1$�Ȃ��$2 \geq 1$�ƂȂ�̂ŕs�����ł��B
$c \geq 2$�Ȃ��$(1-c)n \leq -1$�ƂȂ�̂�$ -cn +2 + n  \leq 1$���������܂��B

�����

```math
\begin{eqnarray}
T(n) &\leq& cn\lg n +1
\end{eqnarray}
```

�ƂȂ�܂��B

## A4.3-5

�}�[�W�\�[�g��$T(n)$�̈����������łȂ��Ƃ����Ȃ��̂ŁA$n$�������̏ꍇ�Ɗ�̏ꍇ�ňقȂ�܂��B

�����̏ꍇ��$\lceil n/2 \rceil$��$\lfloor n/2 \rfloor$�������Ȃ̂�

```math
\begin{eqnarray}
T(n) &=& T(\lceil n/2 \rceil)+T(\lfloor n/2 \rfloor)+\Theta(n) \\
&=& 2T(n/2)+\Theta(n) 
\end{eqnarray}
```
�ł��B


$\Theta(n)$��$O(n)$��$\Omega(n)$���������̂ŁA

```math
\begin{eqnarray}
T(n) &\leq& 2c(n/2)\lg (n/2) + O(n) \\
&=& cn \lg n - cn + O(n) \\
&=& cn \lg n + O(n)
\end{eqnarray}
```

���$T(n) = O(n \lg n + O(n)) = O(n \lg n)$

```math
\begin{eqnarray}
T(n) &\geq& 2c(n/2)\lg (n/2) + \Omega(n) \\
&=& cn \lg n - cn + \Omega(n) \\
&=& cn \lg n + \Omega(n)
\end{eqnarray}
```

���$T(n) = \Omega(n \lg n + \Omega(n)) = \Omega(n \lg n)$

�����$T(n) = \Theta(n \lg n)$���������܂��B


��̏ꍇ��$\lceil n/2 \rceil$��$\lfloor n/2 \rfloor$�͐����ł͂Ȃ��̂ŁA$n-1 < \lfloor n \rfloor \leq n \leq \lceil n \rceil < n+1$�A$\lfloor n/2 \rfloor \geq (n-2+1)/2$�A $\lceil n/2 \rceil \leq (n+2-1)/2$ ���

```math
\begin{eqnarray}
T(n) &=& T(\lceil n/2 \rceil)+T(\lfloor n/2 \rfloor)+\Theta(n) \\
&=& T((n+1)/2)+T((n-1)/2)+\Theta(n) \\
&=& c\frac{n+1}{2} \lg \frac{n+1}{2} + c \frac{n-1}{2} \lg \frac{n-1}{2} +\Theta(n) \\
&=& c\frac{n+1}{2} \lg (n+1) + c \frac{n-1}{2} \lg (n-1) +\Theta(n) \\
&=& c\frac{n}{2} \lg (n^2-1) + c \frac{1}{2} \lg \frac{n+1}{n-1} +\Theta(n) \\
\end{eqnarray}
```

�ł��B

$cn \lg n$���㎮���傫����΂����̂ł����A������ǂ�����ďؖ�����΂����̂��������ł��B

�𓚕ۗ��Ƃ��܂��B



## A4.3-6

�܂�$T(n) \leq cn\lg n -d$�Ɖ��肵�܂��B
������$c \geq 1$�Ƃ��܂��B

```math
\begin{eqnarray}
T(n) &\geq& 2c(n/2 + 17)\lg (n/2 + 17) - d) + n \\  
&=& 2cn\lg (n/2 + 17) + 34 c \lg (n/2 + 17) - 2d + n \\
\end{eqnarray}
```

�ȍ~��A4.3-2�Ɠ������Ǝv���܂��B
�i���̉𓚂ɂ͂��܂莩�M���Ȃ��̂ŁA�Č������邩������܂���j


## A4.3-7

$T(n) \leq cn^{\log_3 4}$�𖞂����Ɖ��肵�܂��B

```math
\begin{eqnarray}
T(n) &=& 4T(n/3) + n \\
&\leq& 4c(n/3)^{\log_3 4} + n \\
&=& cn^{\log_3 4} + n \\
\end{eqnarray}
```

������$3^{\log_3 4} = 4^{\log_3 3} = 4$��p���܂����B
$n \geq 1$�Ȃ̂ŁA���ꂪ$T(n) \leq cn^{\log_3 4}$�Ƃ����`���ɂȂ邱�Ƃ͂���܂���B


����$T(n) \leq cn^{\log_3 4} - dn$�𖞂����Ɖ��肵�܂��B

```math
\begin{eqnarray}
T(n) &=& 4T(n/3) + n \\
&\leq& 4c(n/3)^{\log_3 4} -4dn/3+ n \\
&=& cn^{\log_3 4} -4dn/3 + n \\
\end{eqnarray}
```

������$d=3$�Ȃ��

```math
\begin{eqnarray}
T(n) 
&\leq& cn^{\log_3 4} - 3n
\end{eqnarray}
```

�ƂȂ艼��𖞂����܂��B


## A4.3-8

$T(n) \leq cn^2$�𖞂����Ɖ��肵�܂��B

```math
\begin{eqnarray}
T(n) 
&\leq& c4n^2/4 + n^2 = (c+1)n^2
\end{eqnarray}
```

$c>0$�Ȃ̂ŁA���ꂪ$T(n) \leq cn^2$�𖞂������Ƃ͂���܂���B


����$T(n) \leq cn^2 -dn$�𖞂����Ɖ��肵�܂��B

```math
\begin{eqnarray}
T(n) 
&\leq& c4n^2/4 -dn + n^2 = (c+1)n^2 - dn
\end{eqnarray}
```

�����$T(n) \leq cn^2$�𖞂������Ƃ͂���܂���B
�ȏ�̂悤�ɁA��蕶�́u�᎟�̍����������u�������@�Ɋ�Â��ďؖ��ł���v�Ə����Ă���̂ł����A���ۂɂ͏ؖ��ł��܂���B

�����Ŏ��͖�蕶��$T(n)=4T(n/2)+n^2$��$T(n)=4T(n/2)+n^2$��$T(n)=4T(n/2)+n$�̌�A�ł͂Ȃ����Ƌ^���܂����B
�O�҂�$n^{2 \log_2 4} = n^4$���}�X�^�[�@��K�p�ł�������𖞂������A$\Theta(n^2)$�ɂ��Ȃ�܂���B
��҂Ȃ�΁A�}�X�^�[�@��K�p�����$n^{\log_2 4} = n^2 = \Theta(n^2)$�ƂȂ��蕶�ƈ�v���܂��B

�����Ō�҂��������Ƃ��Ă�蒼���Ă݂܂��B


$T(n) \leq cn^2$�𖞂����Ɖ��肵�܂��B

```math
\begin{eqnarray}
T(n) 
&\leq& c4n^2/4 + n = cn^2 + n
\end{eqnarray}
```

$c>0$�Ȃ̂ŁA���ꂪ$T(n) \leq cn^2$�𖞂������Ƃ͂���܂���B
����$T(n) \leq cn^2 -dn$�𖞂����Ɖ��肵�܂��B

```math
\begin{eqnarray}
T(n) 
&\leq& c4n^2/4 -4dn/2 + n = cn^2 + (1-2d)n
\end{eqnarray}
```

$d=1$�Ƃ���΁A$T(n) \leq cn^2 -n$�𖞂����܂��B


## A4.3-9

$m = \lg n$�Ƃ���$T(2^m)=3T(2^{m/2}) + m$�Ǝ��ό`���܂��B

������$S(m) = T(2^m)$�Ƃ����

```math
\begin{eqnarray}
S(m) 
&=& 3S(m/2) + m
\end{eqnarray}
```

$S(m) \leq cm^{log_2 3} - dm$�𖞂����Ƃ��܂��B

```math
\begin{eqnarray}
S(m) 
&\leq& 3c(m/2)^{log_2 3}  -3dm/2 + m \\
&=& 3cm^{log_2 3}  -3dm/2 + m
\end{eqnarray}
```

�㎮�̓W�J�ł́A$2^{log_2 3} = 3^{log_2 2}= 3$��p���܂����B
$d=2$�Ƃ���΁A$S(m) \leq cm^{log_2 3} - 2m$ �𖞂����܂��B

$S(1) = c -2$���$c = S(1)+2$�ł�

����ĉ��� 

```math
\begin{eqnarray}
T(n) = T(2^m) = S(m) = (S(1)+2)m^{lg 3} - 2m = (T(2)+2) \lg n^{lg 3} - 2\lg n
\end{eqnarray}
```

 �ƂȂ�܂��B

# 4.4 �Q�������������߂̍ċA�ؖ@

## A4.4-1

�ċA�؂̊e���x���̃T�C�Y��$n/2^i$�ł��B
�ċA�؂̃��x���̊K�w�̓T�C�Y��1�ɂȂ�܂Ő[���Ȃ�̂ŁA$n/2^i = 1$�̏ꍇ�A�܂�$i = \log_2 n$�w�ɂȂ�܂��B
�����$i=0,1, \cdots , \log_2 n-1$�ł��B
�܂��e���x���̐ߓ_�̐���$3^i$�ł��B
���x��$\log_2 n$�ł̐ߓ_����$3^{\log_2 n} = n^{\log_2 3}$�ł��B
���x��$\log_2 n$�ł̃R�X�g��$n^{\log_2 3}T(1) = \Theta(n^{\log_2 3})$�ɂȂ�܂��B

�ؑS�̂̃R�X�g��

```math
\begin{eqnarray}
T(n) 
&=& n + \frac{3}{2} n + \frac{3^2}{2^2} n + \cdots + \frac{3^{\log_2 n-1}}{2^{\log_2 n-1}} n + n^{\log_2 3}T(1) \\
&=& \sum_{i=0}^{\log_2 n-1} \Bigl( \frac{3}{2} \Bigl)^i n + n^{\log_2 3}T(1)\\
&\leq& \sum_{i=0}^{\infty} \Bigl( \frac{3}{2} \Bigl)^i n + n^{\log_2 3}T(1)\\
&=& \frac{1}{1-3/2} n + n^{\log_2 3}T(1) \\
&=& O(n^{\log_2 3})
\end{eqnarray}
```

�ɂȂ�܂��B
����đQ�ߓI�����$O(n^{\log_2 3})$�ł��B

���ɂ����u�������@��p���Č��؂��܂��B

$T(n) \leq cn^{\log_2 3} -dn$�𖞂����Ɖ��肵�܂��B

```math
\begin{eqnarray}
T(n) &=& 3T(\lfloor n/2 \rfloor) + n \\
&\leq& 3 \frac{cn^{\log_2 3}}{2^{\log_2 3}} - \frac{3dn}{2} + n \\
&=& cn^{\log_2 3} - \frac{3dn}{2} + n 
\end{eqnarray}
```

$d=2$�Ƃ���΁A$T(n) \leq cn^{\log_2 3} - 2n$���������܂��B

## A4.4-2

�ċA�̊e���x���̃T�C�Y��$n/2^i$�ł��B
�ċA�؂̃��x���̊K�w�̓T�C�Y��1�ɂȂ�܂Ő[���Ȃ�̂ŁA$n/2^i = 1$�̏ꍇ�A�܂�$i = \log_2 n$�w�ɂȂ�܂��B
�����$i=0,1, \cdots , \log_2 n-1$�ł��B
�܂��e���x���̐ߓ_�̐���$1$�ł��B
���x��$\log_2 n$�ł̐ߓ_����$1$�ł��B
���x��$\log_2 n$�ł̃R�X�g��$T(1) = \Theta(1)$�ɂȂ�܂��B

�ؑS�̂̃R�X�g��

```math
\begin{eqnarray}
T(n) 
&=& n + \Bigl( \frac{1}{2} n \Bigl)^2 + \Bigl( \frac{1}{2^2} n  \Bigl)^2 + \cdots + \Bigl( \frac{1}{2^{\log_2 n-1}} n  \Bigl)^2 + T(1) \\
&=& \sum_{i=0}^{\log_2 n-1} \Bigl( \frac{1}{4} \Bigl)^i n^2 + T(1)\\
&\leq& \sum_{i=0}^{\infty} \Bigl( \frac{1}{4}\Bigl)^i n^2 + T(1)\\
&=& \frac{1}{1-1/4} n^2 + T(1) \\
&=& O(n^2)
\end{eqnarray}
```

�ɂȂ�܂��B
����đQ�ߓI�����$O(n^2)$�ł��B

���ɂ����u�������@��p���Č��؂��܂��B

$T(n) \leq cn^2$�𖞂����Ɖ��肵�܂��B

```math
\begin{eqnarray}
T(n) &=& T(n/2) + n^2 \\
&\leq& cn^2/2^2 + n^2 \\
&=& (c/4 + 1) n^2 \leq cn^2
\end{eqnarray}
```

���������܂��B
������$c/4 + 1 \leq c$�ƂȂ�悤��$4/3 \leq c$�Ƃ��܂��B

## A4.4-3

�ċA�؂̊e���x���̃T�C�Y��$n/2^i+2$�ł��B
�ċA�؂̃��x���̊K�w�̓T�C�Y��3�ɂȂ�܂Ő[���Ȃ�̂ŁA$n/2^i+2 = 3$�̏ꍇ�A�܂�$i = \log_2 n$�w�ɂȂ�܂��B
�����$i=0,1, \cdots , \log_2 n-1$�ł��B
�܂��e���x���̐ߓ_�̐���$4^i$�ł��B
���x��$\log_2 n$�ł̐ߓ_����$4^{\log_2 n} = n^{\log_2 4}=n^2$�ł��B
���x��$\log_2 n$�ł̃R�X�g��$n^2 T(3) = \Theta(n^2)$�ɂȂ�܂��B

�ؑS�̂̃R�X�g��

```math
\begin{eqnarray}
T(n) 
&=& n + \Bigl(\frac{4}{2} n + 8 \Bigl) + \Bigl(\frac{4^2}{2^2} n + 4^2*2 \Bigl) \cdots + \Bigl(\frac{4^{\log_2 n-1}}{2^{\log_2 n-1}} n + 4^{\log_2 n-1}*2 \Bigl) + n^2 T(3) \\  
&=& n + (2 n + 8 ) + (4 n + 32 ) \cdots + (2^{\log_2 n-1} n + 4^{\log_2 n-1}*2 ) + n^2 T(3) \\
&=& n + (2 n + 8 ) + (4 n + 32 ) \cdots + ((n-1)n + 2(n-1)^2) + n^2 T(3) \\
&=& O(n^2)
\end{eqnarray}
```

�ɂȂ�܂��B
����đQ�ߓI�����$O(n^2)$�ł��B

���ɂ����u�������@��p���Č��؂��܂��B

$T(n) \leq cn^2 -dn$�𖞂����Ɖ��肵�܂��B������$c \geq 0$�Ƃ��܂��B

```math
\begin{eqnarray}
T(n) &=& 4T(n/2+2) + n \\
&\leq& 4c(n/2+2)^2 -4d(n/2+2) + n \\
&=& c(n^2 + 8n + 16 - 2dn/c -8d/c)+n
\end{eqnarray}
```

$c(8-2d/c)+1 \leq -d$��$c(16-8d/c) \leq 0$�𖞂����Ή��肪�������܂��B

$c(16-8d/c) \leq 0$���A$d \geq 2c$�ƂȂ�܂��B

$d/c=2$�̏ꍇ�A$4c+1 \leq -2$�ł����A$c \geq 0$�Ȃ̂Ő������܂���B
$d/c=3$�̏ꍇ�A$2c+1 \leq -3$�ł����A$c \geq 0$�Ȃ̂Ő������܂���B
$d/c=4$�̏ꍇ�A$1 \leq -4$�ł����A�������܂���B
$d/c=5$�̏ꍇ�A$-2c+1 \leq -5$�Ȃ̂ŁA$c \geq 3$�̏ꍇ�ɐ������܂��B
������$d \geq 6$�ƂȂ�܂��B$d/c=5$���������Ȃ��Ƃ����Ȃ��̂ŁA�����ł�$d=5c$�Ƃ��܂��B


����āA$c \geq 3$�𖞂����΁A$T(n) \leq cn^2 -5cn$���������܂��B



## A4.4-4

�ċA�؂̊e���x���̃T�C�Y��$1^{n-i}$�ł��B
�ċA�؂̃��x���̊K�w�̓T�C�Y��$1^1$�ɂȂ�܂Ő[���Ȃ�̂ŁA$n$�w�ɂȂ�܂��B
�����$i=0,1, \cdots , n-1$�ł��B
�܂��e���x���̐ߓ_�̐���$2^i$�ł��B
���x��$n$�ł̐ߓ_����$2^n$�ł��B
���x��$n$�ł̃R�X�g��$2^n T(1) = \Theta(2^n)$�ɂȂ�܂��B

�ؑS�̂̃R�X�g��

```math
\begin{eqnarray}
T(n) 
&=& 1 + 2 +  \cdots + 2^{n-1} + 2^n T(1) 
&=& O(2^n)
\end{eqnarray}
```

�ɂȂ�܂��B
����đQ�ߓI�����$O(2^n)$�ł��B

���ɂ����u�������@��p���Č��؂��܂��B

$T(n) \leq c2^n -d$�𖞂����Ɖ��肵�܂��B������$c \geq 0$�Ƃ��܂��B

```math
\begin{eqnarray}
T(n) &=& 2T(n-1) + 1 \\
&\leq& 2c2^{n-1} -2d + 1 \\
&=& c2^n -2d+1
\end{eqnarray}
```

$d=1$�Ƃ���΁A$T(n) \leq c2^n -1$���������܂��B

## A4.4-5

$T(n/2)$�ɂ��āA�ċA�̊e���x���̃T�C�Y��$n/2^i$�ł��B
�ċA�؂̃��x���̊K�w�̓T�C�Y��1�ɂȂ�܂Ő[���Ȃ�̂ŁA$n/2^i = 1$�̏ꍇ�A�܂�$i = \log_2 n$�w�ɂȂ�܂��B
�����$i=0,1, \cdots , \log_2 n-1$�ł��B
�܂��e���x���̐ߓ_�̐���$2^i$�ł��B�i$T(n-1)$�ɂ��ߓ_����G�c�Ȑ���Ƃ��Ċ܂߂Ă��܂��܂��B�j
���x��$\log_2 n$�ł̐ߓ_����$2^{\log_2 n} = n$�ł��B
���x��$\log_2 n$�ł̃R�X�g��$nT(1) = \Theta(n)$�ɂȂ�܂��B

�ؑS�̂̃R�X�g��

```math
\begin{eqnarray}
T(n) 
&=& n + \frac{1}{2} n + \frac{1}{2^2} n + \cdots +  \frac{1}{2^{\log_2 n-1}} n  + nT(1) \\
&=& \sum_{i=0}^{\log_2 n-1} \Bigl( \frac{1}{2} \Bigl)^i n + nT(1)\\
&\leq& \sum_{i=0}^{\infty} \Bigl( \frac{1}{2} \Bigl)^i n + nT(1)\\
&=& \frac{1}{1-1/2} n + nT(1) \\
&=& O(n)
\end{eqnarray}
```

$T(n-1)$�ɂ��āA�ċA�؂̊e���x���̃T�C�Y��$1^{n-i}$�ł��B
�ċA�؂̃��x���̊K�w�̓T�C�Y��$1^1$�ɂȂ�܂Ő[���Ȃ�̂ŁA$n$�w�ɂȂ�܂��B
�����$i=0,1, \cdots , n-1$�ł��B
�܂��e���x���̐ߓ_�̐���$2$�ł��B�i$T(n/2)$�ɂ��ߓ_����G�c�Ȑ���Ƃ��Ċ܂߂Ă��܂��܂��B�j
���x��$n$�ł̐ߓ_����$2^n$�ł��B
���x��$n$�ł̃R�X�g��$2^n T(1) = \Theta(2^n)$�ɂȂ�܂��B

�ؑS�̂̃R�X�g��

```math
\begin{eqnarray}
T(n) 
&=& n + n +  \cdots + n + 2^n T(1) 
&=& O(2^n)
\end{eqnarray}
```

�ɂȂ�܂��B

�S�̓I�ɂ͂�����̌v�Z����^���āA�Q�ߓI�����$O(2^n)$�ɂȂ�Ɨ\�����܂��B

���ɂ����u�������@��p���Č��؂��܂��B

$T(n) \leq 2^n$�𖞂����Ɖ��肵�܂��B������$c \geq 0$�Ƃ��܂��B

```math
\begin{eqnarray}
T(n) 
&\leq& 2^{n-1} + \sqrt{2^n} + n
\end{eqnarray}
```

$sqrt{2^n} + n \leq 2^{n-1}$�Ȃ̂�

```math
\begin{eqnarray}
T(n) 
&\leq& 2^n
\end{eqnarray}
```

���������܂��B

## A4.4-6

�{���ɂ����ẮA�Q�ߓI��E���o�H�̒������̍ċA��$T(2n/3)$����̐���ŋ��߂Ă��܂����B

�Q�ߓI���E�͌o�H���Z�����̍ċA�؂�$T(n/3)$����̐���ŋ��܂�܂��B

�ċA�̊e���x���̃T�C�Y��$n/3^i$�ł��B
�ċA�؂̃��x���̊K�w�̓T�C�Y��1�ɂȂ�܂Ő[���Ȃ�̂ŁA$n/3^i = 1$�̏ꍇ�A�܂�$i = \log_3 n$�w�ɂȂ�܂��B
�����$i=0,1, \cdots , \log_3 n-1$�ł��B
�܂��e���x���̐ߓ_�̐���$2^i$�ł��B
���x��$\log_3 n$�ł̐ߓ_����$2^{\log_3 n}= n^{\log_3 2}$�ł��B
���x��$\log_3 n$�ł̃R�X�g��$n^{\log_3 2} T(1) = \Theta(n^{\log_3 2})$�ɂȂ�܂��B

�ؑS�̂̃R�X�g��

```math
\begin{eqnarray}
T(n) 
&=& cn + \frac{2}{3} cn + \frac{2^2}{3^2} cn + \cdots + \frac{2^{\log_3 n-1}}{3^{\log_3 n-1}} cn + n^{\log_3 2} T(1) \\
&=& \sum_{i=0}^{\log_3 n-1} \Bigl( \frac{2}{3} \Bigl)^i cn + n^{\log_3 2} T(1)\\
&=& \frac{(2/3)^{\log_3 n}-1}{(2/3)-1}cn  + n^{\log_3 2} T(1) \\
&=& \frac{1}{3} (1 - 2^{\log_3 n}/n)cn + n^{\log_3 2} T(1) \\
\end{eqnarray}
```

�ɂȂ�܂��B
���ꂪ$T(n) \geq dn \lg n$�𖞂������Ƃ��������Ƃ��܂������A�ǂ��v�Z����΂����̂�������܂���ł����B
����āA�ȏ�̌v�Z�͑S�ĂȂ��Ƃ��܂����A�����Ɏ���Ȃ������l�@�̔��Y�^�Ƃ��Ďc���܂��B
�����čŏ������蒼���܂��B

�{���̋c�_�Ƃ͋t��$d \leq c/(\lg 3 - 2/3)$�𖞂����悤�ɑI�ׂ�$T(n) \geq dn \lg n$�ɂȂ�̂�$\Omega(n \lg n)$���������܂��B


## A4.4-7
$T(n) \leq dn^2$�Ɖ��肵�܂��B

```math
\begin{eqnarray}
T(n) 
&\leq& 4T(\lfloor n/2 \rfloor) + cn \\
&=& dn^2 + cn
\end{eqnarray}
```

$dn^2 + cn \leq n^2$�𖞂�����$n^2$���^�C�g�ȏ���Ƃ݂Ȃ��܂��B�����$d+c/n \leq 1$�𖞂����邩���l���܂��B
�\���ɑ傫��$n$�ł�2���ڂ�0�ɋ߂Â��܂��B$d \leq 1$�𖞂����Ȃ�Η^���͐������܂��B

�t��$T(n) \geq dn^2$�Ɖ��肵�܂��B

```math
\begin{eqnarray}
T(n) 
&\geq& 4T(\lfloor n/2 \rfloor) + cn \\
&=& dn^2 + cn
\end{eqnarray}
```

$dn^2 + cn \geq n^2$�𖞂�����$n^2$���^�C�g�ȉ����Ƃ݂Ȃ��܂��B�����$d+c/n \geq 1$�𖞂����邩���l���܂��B
�\���ɑ傫��$n$�ł�2���ڂ�0�ɋ߂Â��܂��B$d \geq 1$�𖞂����Ȃ�Η^���͐������܂��B

����ă^�C�g�Ȍ��E��$n^2$�ƂȂ�܂��B


## A4.4-8

�ċA�؂̍�����$n$��$a$�Ŋ���؂��Ȃ��$n/a$�ł��B
$n$��$a$�Ŋ���؂ꂸ�ɗ]�肪�o��Ȃ��$(n/a)-(�]��)$�ł��B
����čċA�؂̍�����$\lceil n/a \rceil$�ƂȂ�܂��B

�ؑS�̂̃R�X�g��

```math
\begin{eqnarray}
T(n) 
&=& T(n-a) + T(a) + cn \\
&=& T((n-a)-a) + T(a) + c(n-a) + T(a) + cn  \\
&=& T((n-a)-2a) + T(a) + c((n-a)-a) + T(a) + c(n-a) + T(a) + cn   \\
&=& T((n-a)-3a) + T(a) + c((n-a)-2a) + T(a) + c((n-a)-a) + T(a) + c(n-a) + T(a) + cn \\
&=& T(1) + \sum_{i=0}^{\lceil n/a \rceil-1} (T(a) + c(n-ia) \\
&=& T(1) + \lceil n/a \rceil T(a) + \lceil n/a \rceil cn -ca \frac{\lceil n/a \rceil (\lceil n/a \rceil - 1)}{2}\\
&=& T(1) + n \Bigl( \frac{T(a)}{a} + \frac{c}{2} \Bigl) + \frac{c}{2a}n^2 \\
&=& \Theta(n^2)
\end{eqnarray}
```

�ƂȂ�܂��B

## A4.4-9

���̍ċA���́A�T�C�Y$n$��$\alpha n$��$(1-\alpha)n$�ɕ������܂��B
���̊K�w�ł́A$\alpha n$��$\alpha^2 n$��$(1-\alpha)\alpha n$�ɕ������A$(1-\alpha) n$��$(1-\alpha)\alpha n$��$(1-\alpha)^2 n$�ɕ������܂��B
���̂悤��2�����𑱂���ċA����$\alpha = 1/2$�Ȃ��2�͂ł̃}�[�W�\�[�g���̂��̂ł��B

�����Ń}�[�W�\�[�g�Ƃ̗ސ�����A�v�Z�̃T�C�Y��$n \lg n$�Ɖ��肵�܂��B

$T(n) \leq k n \lg n -tn$�𖞂����Ɖ��肵�܂��B������$k \geq 0$�Ƃ��܂��B

```math
\begin{eqnarray}
T(n) 
&\leq& T(\alpha n) + T((1-\alpha)n) + cn \\
&=& k \alpha n \lg (\alpha n) -t \alpha n + k (1-\alpha)n \lg ((1-\alpha)n) -t (1-\alpha)n + cn  \\
&=& k \alpha n \lg \alpha + k \alpha n \lg n - t \alpha n + k (1-\alpha)n \lg (1-\alpha) + k (1-\alpha)n \lg n -t (1-\alpha)n + cn  \\
&=& k \alpha n \lg \alpha + (k \alpha n + k n - k \alpha n) \lg n  + k (1-\alpha)n \lg (1-\alpha) - t \alpha n  -t n- t \alpha n + cn\\
&=& k \alpha n \lg \alpha + k n \lg n  + k (1-\alpha)n \lg (1-\alpha) -t n + cn
\end{eqnarray}
```

������$t=c$�ɂȂ�悤�ɑI�т܂��B
�܂�$0 < \alpha < 1$���$\lg n$��$\lg (1-\alpha)$�͕K�������̑ΐ�$\log(a/b)=\log a - \log b$�ɂȂ�A�����ꂪ���q�����傫���̂�$\log a - \log b \leq 0$�ɂȂ�܂��B
������$a$��K���ɂ���Ԃ��Ƃ�$k \alpha n \lg \alpha + k (1-\alpha)n \lg (1-\alpha) \leq -c$�ɂł��܂��B

�����$T(n) \leq  k n \lg n -cn$���������܂��B

# 4.5 �Q�������������߂̃}�X�^�[�@

## �}�X�^�[�藝

$a \geq 1$�A$b > 1$�Ƃ��܂��B

```math
\begin{eqnarray}
T(n) 
&=& aT(n/b)+f(n)
\end{eqnarray}
```

�ɂ��āA$f(n)$���ǂ̂悤�ȑQ�ߓI���E�������ɂ��A$T(n)$�ɂ͈ȉ��̂R�̑Q�ߓI���E���������܂��B

### �Q�ߓI��E

$\epsilon > 0$�ɑ΂��đQ�ߓI��E$f(n) = O(n^{\log_b a - \epsilon})$�𖞂����Ȃ�΁A$T(n) = \Theta(n^{\log_b a})$�ɂȂ�܂��B

��ʓI�ɂ�$f(n) = O(f(n)�̍ő原��)$�Ƃ݂Ȃ��Ă����Ǝv���܂��B

�ł����A����$f(n)/n^{\log_b a}$��$1/n^{\epsilon}$���Q�ߓI�ɑ傫���ꍇ��$f(n)$��$O(n^{\log_b a - \epsilon})$�̏�E�𒴂��邱�ƂɂȂ�A$f(n) = O(n^{\log_b a - \epsilon})$�𖞂����Ȃ��̂Œ��ӂ��Ȃ��Ƃ����܂���B

�����$n^{\log_b a}/f(n)$��$n^{\epsilon}$���Q�ߓI�ɏ������ꍇ�ƌ��������邱�Ƃ��ł��܂��B

### �Q�ߓI���E

�Q�ߓI���E$f(n) = \Theta(n^{\log_b a})$�𖞂����Ȃ�΁A$T(n) = \Theta(n^{\log_b a} \lg n)$�ɂȂ�܂��B


### �Q�ߓI���E

$\epsilon > 0$�ɑ΂��đQ�ߓI���E$f(n) = \Omega(n^{\log_b a + \epsilon})$�𖞂����A����$c<1$�Ə\���傫��$n$�ɑ΂���$af(n/b) \leq cf(n)$�Ȃ�΁A$T(n) = \Theta(f(n))$�ɂȂ�܂��B

����$f(n)/n^{\log_b a}$��$n^{\epsilon}$���Q�ߓI�ɏ������ꍇ��$f(n)$��$\Omega(n^{\log_b a + \epsilon})$�̉��E������邱�ƂɂȂ�A$f(n) = \Omega(n^{\log_b a + \epsilon})$�𖞂����Ȃ��̂Œ��ӂ��Ȃ��Ƃ����܂���B

## A4.5-1

### a

$T(n) = 2T(n/4)+1$

$a=2$�A$b=4$�A$f(n)=1$

$f(n) = 1 = \Theta(1)$

$n^{\log_b a + \epsilon} = n^{\log_4 2 + \epsilon}$�ɂ���$\epsilon = -\log_4 2$�ɑI�ׂ�$n^0=1=O(1)$�ɂȂ�̂ŁA$T(n)=\Theta(n^{\log_4 2})=\Theta(n^{1/2})$���������܂��B

### b

$T(n) = 2T(n/4)+\sqrt{n}$

$a=2$�A$b=4$�A$f(n)=n^{1/2}$

$f(n) = \sqrt{n} = \Theta(n^{1/2})$

$n^{\log_b a + \epsilon} = n^{\log_4 2 + \epsilon}$�ɂ���$\epsilon = 0$�ɑI�ׂ�$n^{1/2}=\Theta(n^{1/2})$�ɂȂ�̂ŁA$T(n)=\Theta(n^{1/2} \lg n)$���������܂��B

### c

$T(n) = 2T(n/4)+n$

$a=2$�A$b=4$�A$f(n)=n$

$f(n) = n = \Theta(n)$

$n^{\log_b a + \epsilon} = n^{\log_4 2 + \epsilon}$�ɂ���$\epsilon = +\log_4 2$�ɑI�ׂ�$n=\Omega(n)$�ɂȂ�܂��B

$an/b = n/2 \leq cn$��$1/2 \leq c < 1$�ɂ��Đ�������̂ŁA$T(n)=\Theta(n)$���������܂��B

### d

$T(n) = 2T(n/4)+n^2$

$a=2$�A$b=4$�A$f(n)=n^2$

$f(n) = n^2 = \Theta(n^2)$

$n^{\log_b a + \epsilon} = n^{\log_4 2 + \epsilon}$�ɂ���$\epsilon = 1+\log_4 2$�ɑI�ׂ�$n^2=\Omega(n^2)$�ɂȂ�܂��B

$an/b = n^2/2 \leq cn^2$��$1/2 \leq c < 1$�ɂ��Đ�������̂ŁA$T(n)=\Theta(n^2)$���������܂��B


## A4.5-2

�{�����Strassen�̃A���S���Y���̑Q�ߓI����$\Theta(n^{\lg 7})$�ł��B�i$2.80 < \lg 7 < 2.81$�j

$T(n) = aT(n/4)+\Theta(n^2)$���A$n^{\log_b a + \epsilon} = n^{\log_4 a + \epsilon}$�ɂ���$\epsilon = 0$�Ȃ��$T(n)=\Theta(\log_4 a \lg n)$�ɂȂ�܂��B
�ł��������$\Theta(n^{\lg 7})$�������I�ɂ͂Ȃ�Ȃ��̂Ő������܂���B

$T(n) = aT(n/4)+\Theta(n^2)$���A$n^{\log_b a + \epsilon} = n^{\log_4 a + \epsilon}$�ɂ���$\epsilon < 0$�Ȃ��$T(n)=\Theta(\log_4 a)$�ɂȂ�܂��B
�����$\log_4 a < \lg 7$�𖞂����ő�̐���$a$��48�ł��B
�Ȃ��A$n^{\log_b a + \epsilon} = n^{\log_4 48 + \epsilon}$�ɂ���$\log_4 48 + \epsilon = 2$�ɂȂ�悤��$\epsilon$��I�Ԃ��Ƃ��ł��܂��B


$T(n) = aT(n/4)+\Theta(n^2)$���A$n^{\log_b a + \epsilon} = n^{\log_4 a + \epsilon}$�ɂ���$\epsilon > 0$����$\Theta(a n^2/4) \leq \Theta(c n^2)$�Ȃ��$T(n)=\Theta(n^2)$�ɂȂ�܂��B
��L���������邽�߂ɂ́A$\log_4 a + \epsilon = 2$�ɂȂ�悤�ɑI�΂Ȃ��Ƃ����܂���B�ł����A$a$�̍ő�l��$a=4^2$�𒴂��邱�Ƃ͂Ȃ��̂ŁA$a=48$���ő�̐����ɂȂ�܂��B


## A4.5-3

$T(n) = T(n/2)+\Theta(1)$

$a=1$�A$b=2$�A$f(n)=\Theta(1)$

$n^{\log_b a + \epsilon} = n^{\log_2 1 + \epsilon}$�ɂ���$\epsilon = 0$�ɑI�ׂ�$n^0=1=\Theta(1)$�ɂȂ�̂ŁA$T(n)=\Theta(\lg n)$���������܂��B

## A4.5-4

$T(n) = 4T(n/2)+ n^2 \lg n$

$a=4$�A$b=2$�A$f(n) = n^2 \lg n$


$n^{\log_b a + \epsilon} = n^{\log_2 4 + \epsilon} = n^{2 + \epsilon}$�ɂ��čl���܂��B
$\epsilon = 0$�ɑI�ׂ�$n^2$�ɂ���$\Theta(n^2 \lg n)$���������܂���B
$\epsilon < 0$�ɑI�ׂ�$\lg n$��$1/n^{\epsilon}$���Q�ߓI�ɑ傫���̂�$O(n^2 \lg n)$���������܂���B
$\epsilon > 0$�ɑI�ׂ�$\lg n$��$n^{\epsilon}$���Q�ߓI�ɏ������̂�$\Omega(n^2 \lg n)$���������܂���B

����ă}�X�^�[�藝�͓K�p�ł��܂���B
������$T(n)$�̑Q�߉���$\Theta(n^2)$�A$\Theta(n^2 \lg n)$�ł͂���܂���B


$T(n) \leq n^2 \lg^2 n$�Ɖ��肵�܂��B

```math
\begin{eqnarray}
T(n) 
&=& 4T(n/2)+n^2 \lg n \\
&\leq& 4 \frac{n^2}{4} (\lg n - \lg 2)^2 +n^2 \lg n�@\\
&=& n^2 \lg^2 n +n^2 -2 n^2 \lg n +n^2 \lg n\\
&=& n^2 \lg^2 n -n^2(\lg n -1) \\
&\leq& n^2 \lg^2 n
\end{eqnarray}
```

���������܂��B�����$T(n)=O(n^2 \lg^2 n)$�ɂȂ�܂��B


## A4.5-5

������$af(n/b) \leq cf(n)$�𖞂����Ȃ�$f(n)$�����߂���ł��B
$f(n) < f(n/b)$�𖞂����悤�Ȋ֐���T���܂��B
������$f(n)$��$\Omega(n^{\log_b a + \epsilon})$���������Ȃ��Ƃ����܂���B

�v�����Ȃ��̂ŉ𓚕ۗ��ł��B


# 4.6 �}�X�^�[�藝�̏ؖ�

## ���m�ȃx�L��ɑ΂���ؖ�

### ���4.2

�Q�������ċA�ؖ@�ŉ����܂��B

�ċA�؂̊e���x���̃T�C�Y��$n/b^i$�ł��B
�ċA�؂̃��x���̊K�w�̓T�C�Y��1�ɂȂ�܂Ő[���Ȃ�̂ŁA$n/b^i = 1$�̏ꍇ�A�܂�$i = \log_b n$�w�ɂȂ�܂��B
�����$i=0,1, \cdots , \log_b n-1$�ł��B
�܂��e���x���̐ߓ_�̐���$a^i$�ł��B
���x��$\log_b n$�ł̐ߓ_����$a^{\log_b n} = n^{\log_b a}$�ł��B
���x��$\log_b n$�ł̃R�X�g��$n^{\log_b a} T(1) = \Theta(n^{\log_b a})$�ɂȂ�܂��B

�ؑS�̂̃R�X�g��

```math
\begin{eqnarray}
T(n) 
&=& f(n) + a f(n/b) + a^2 f(n/b^2) + \cdots + a^{\log_2 n-1} f(n/b^{\log_2 n-1}) + \Theta(n^{\log_b a}) \\
&=& \sum_{j=0}^{\log_b n-1} a^j f(n/b^j) + \Theta(n^{\log_b a})
\end{eqnarray}
```

�ɂȂ�܂��B

�}�X�^�[��̂R�̃p�^�[���́A�t�̃R�X�g$\Theta(n^{\log_b a})$����v�R�X�g�ɂȂ�ꍇ�A���̃R�X�g$f(n)$���t�̃R�X�g$\Theta(n^{\log_b a})$�Ɠ������ꍇ�A���̃R�X�g$f(n)$����v�R�X�g�ɂȂ�ꍇ��\���Ă���Ɖ��߂ł��܂��B



# ���̋L��
- [�l�I�׋������u�A���S���Y���E�C���g���_�N�V�����v��Rust�Ŏ�������F2��](https://qiita.com/Kosuke_Matsui/items/eea26e88cd261173a292)
- [�l�I�׋������u�A���S���Y���E�C���g���_�N�V�����v��Rust�Ŏ�������F3��](https://qiita.com/Kosuke_Matsui/private/b424d8eb30dd16455b12)
- [�l�I�׋������u�A���S���Y���E�C���g���_�N�V�����v��Rust�Ŏ�������F4��](https://qiita.com/Kosuke_Matsui/private/3b93d6b8a7ee5e90a2b5)

