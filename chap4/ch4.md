---
title: �l�I�׋������u�A���S���Y���E�C���g���_�N�V�����v��Rust�Ŏ�������F4��
tags: �A���S���Y���C���g���_�N�V����
author: Kosuke_Matsui
slide: false
---
# �L���̊T�v
�A���S���Y���C���g���_�N�V���� ��3�� �����ŁF���E�W��MIT���ȏ��̃A���S���Y����Rust�Ŏ������Ȃ���׋������̂ŁA���̌��ʂ𐮗����܂����B
�܂Ƃ߂����e�ɂ͊m���Ɍ�肪����Ǝv����̂ŁA�Q�Ƃɂ������ẮA���炩���߂��������������B

�l�I�ȕ׋������Ȃ̂ő��l�l�Ɍ��₷�����̂ɂȂ��Ă��܂���B
�܂��A�{�̒��쌠����邽�߂ɁA�{���Q�Ƃ��Ȃ���łȂ��Ɨ����ł��Ȃ��悤�ɏ����Ă��܂��B

�����4�͂ɂ��ĕ׋����܂����B

# 4.1

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
