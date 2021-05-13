---
title: 個人的勉強メモ「アルゴリズム・イントロダクション」をRustで実装する：4章
tags: アルゴリズムイントロダクション
author: Kosuke_Matsui
slide: false
---
# 記事の概要
アルゴリズムイントロダクション 第3版 総合版：世界標準MIT教科書のアルゴリズムをRustで実装しながら勉強したので、その結果を整理しました。
まとめた内容には確実に誤りがあると思われるので、参照にあたっては、あらかじめご了承ください。

個人的な勉強メモなので他人様に見やすいものになっていません。
また、本の著作権を守るために、本を参照しながらでないと理解できないように書いています。

今回は4章について勉強しました。

# 4.1

## 総当たり戦略
配列の任意の2要素を選び、その差分が最大になる組み合わせを選ぶアルゴリズムです。
全ての組み合わせを計算して最大値を最後まで残すのが総当たり戦略です。

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
隣接する要素の差分を取った配列を作ります。
その差分配列の任意の個数の連続した要素の和が最大になる組み合わせ選ぶアルゴリズムです。

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

## 分割統治
先ほどの差分配列を右配列と左配列の２つに分割したとします。
この場合、最大部分配列は以下の３つのどれかになります。

1. 最大部分配列は右要素だけで構成される
2. 最大部分配列は左要素だけで構成される
3. 最大部分配列は右配列と左配列の両方で構成される

パターン３の場合は、最大部分配列は、配列を分割する中央要素を必ず含みます。
そこで中央要素から開始して、左配列の要素を１つずつ左に移動しながら足していった中の最大値と、右配列の要素を１つずつ右に移動しながら足していった中の最大値の和が最大部分配列になります。
開始要素は左配列で最大値を更新した時の要素で、終了要素は右配列で最大値を更新した時の要素です。


そしてパターン１、２、３の最大部分配列を求めて、その３つの中で最大になるものが本当の最大部分配列になります。
では、右配列と左配列の最大部分配列をどうやって求めるかと言えば、各配列を右配列と左配列に分けて……と再帰的に上記と同様の処理を繰り返します。
これは分割した配列の要素数が１個になるまで繰り返されます。

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
    println!("{}日目終了時に{}ドルで購入し、{}日目終了時に{}ドルで売却すると、最大の利益{}を得る", low, v[low], high+1, v[high+1], sum);

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
