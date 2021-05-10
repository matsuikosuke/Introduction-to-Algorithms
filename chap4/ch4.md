---
title: �l�I�׋������u�A���S���Y���E�C���g���_�N�V�����v��Rust�Ŏ�������F4��
tags: �A���S���Y���C���g���_�N�V����
author: Kosuke_Matsui
slide: false
---
# test


# 4.1

## ��������헪

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
