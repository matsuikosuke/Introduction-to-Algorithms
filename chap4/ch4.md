---
title: 個人的勉強メモ「アルゴリズム・イントロダクション」をRustで実装する：4章
tags: アルゴリズムイントロダクション
author: Kosuke_Matsui
slide: false
---
# test


# 4.1

## 総当たり戦略

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

    println!("{}日目終了時に{}ドルで購入し、{}日目終了時に{}ドルで売却すると、最大の利益{}を得る", start, v[start], end, v[end], max);
}

fn main() {
    let v: Vec<i32> = vec![100, 113, 110, 85, 105, 102, 86, 63, 81, 101, 94, 106, 101, 79, 94, 90, 97];
    //let v: Vec<i32> = vec![1, 900, 110, 85, 105, 102, 86, 63, 81, 101, 94, 106, 101, 79, 94, 90, 97];
    //let v: Vec<i32> = vec![1, 113, 110, 85, 105, 102, 86, 63, 81, 101, 94, 106, 101, 79, 94, 90, 900];
    println!("v={:?}", &v);

    find_max_round_robin(v);
}
```

## 最大部分配列の総当たり戦略

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

    println!("{}日目終了時に購入し、{}日目終了時に売却すると、最大の利益{}を得る", start, end, max);
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

$A[0 \dots j]$までの最大部分配列が分かっている場合、$A[0 \dots j+1]$の最大部分配列と比較します。

最初の要素から順番に足して、暫定的な部分配列の和を求めます。
部分配列の和が現時点の最大値を超えた場合、暫定的な部分配列の和を最大値として更新します。また開始インデックス、終了インデックスも更新します。

売買の開始日をどのように取得するかに工夫が必要です。
暫定的な部分配列の総和が負になった時は、損をする開始日であると判定し、部分配列の総和をリセットし、暫定開始インデックスを更新します。



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

    println!("{}日目終了時に購入し、{}日目終了時に売却すると、最大の利益{}を得る", start, end, max);
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
