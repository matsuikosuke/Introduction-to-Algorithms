---
title: 個人的勉強メモ「アルゴリズム・イントロダクション」をRustで実装する：6章
tags: アルゴリズムイントロダクション
author: Kosuke_Matsui
slide: false
---
# 記事の概要
アルゴリズムイントロダクション 第3版 総合版：世界標準MIT教科書のアルゴリズムをRustで実装しながら勉強したので、その結果を整理しました。
まとめた内容には確実に誤りがあると思われるので、参照にあたっては、あらかじめご了承ください。

個人的な勉強メモなので他人様に見やすいものになっていません。
また、本の著作権を守るために、本を参照しながらでないと理解できないように書いています。

今回は6章について勉強しました。

RUSTコードの動作は以下のサイトで確認できます。
https://play.rust-lang.org/

# 6.1　ヒープ

（2分木）ヒープデータ構造は、以下の性質を持ちます。

- 節点からノードが2個ずつ分岐していく木構造である
    - maxヒープは、根が最大値になる
        - 親ノードは子ノードより値が大きい
    - minヒープは、根が最小値になる
        - 親ノードは子ノードより値が小さい
- 高さ$h$の木の最下段は、ヒープの要素数が$2^{h+1}-1$未満の場合、左からノードを埋めていき、1個もしくは0個の葉しか持たない節点ができる


## A6.1-1

高さ$h$のヒープが持つ要素数は

高さ0の場合：$1$
高さ1の場合：$1+2$
高さ2の場合：$1+2+2^2$
高さ3の場合：$1+2+2^2+2^3$
高さh-1の場合：$\sum_{i=0}^{h-1}2^i$

高さ$h-1$までの要素数を先に計算しておきます。
$x = \sum_{i=0}^{h-1}2^i$として

```math
\begin{eqnarray}
x &=& 2x -x \\
&=& \sum_{i=1}^{h}2^i - \sum_{i=0}^{h-1}2^i \\
&=& 2^h -1
\end{eqnarray}
```

です。

最大の場合は高さ$h$番目の要素数は$2^h$なので、$2^h-1+2^h = 2^{h+1}-1$になります。

最小の場合は高さ$h$番目の要素数は$1$なので、$2^h-1+1 = 2^h$になります。


## A6.1-2
要素数を$n=2^m+k$とします。$m$は任意の値で、$k$は$0 \leq k \leq 2^m-1$の値とします。
要素数が$2^m \leq n \leq 2^{m+1}-1$として、最小のヒープ要素数$2^m$でも、必ず高さ$m$を持ちます。
よって$2^m \leq n$より、$m \leq \lg n$となります。

また、最大のヒープ要素数$2^{m+1}-1$でも高さ$m+1$の要素数は満たしません。
よって$n < 2^{m+1}$なので、$\lg n -1 < m$となります。

よって高さは$\lg n-1 < m \leq \lg n$を満たすので$\lfloor \lg n \rfloor$となります。

## A6.1-3
最大要素が根にあるのは定義から自明だと思うのですが、何を証明すればいいのか分からないので解答保留です。

## A6.1-4
maxヒープにおいて最小要素は葉に置かれます

## A6.1-5
既ソート配列は最小要素が根に来るのでminヒープです

## A6.1-6
23 → 17, 14
17 → 6, 13
14 → 10, 1
6 → 5, 7：親ノードが子ノードより大きくなっている
13 → 12

親ノードが子ノードより大きくなっているのでmaxヒープではありません。

## A6.1-7
高さ0：1
高さ1：from 2 to 3
高さ2：from 4 to 7
高さ3：from 8 to 15
高さh-1：from $2^{h-1}$ to $2^h-1$
高さh：from $2^h$ to $2^{h+1}-1$

要素数が$n=2^h$の場合は、葉は高さh-1の2番目の要素から最後の要素までと高さhの最初の要素です。
つまり$2^{h-1}+1$ から $2^h-1$と$2^h$です。これは$n/2+1, n/2+2, \cdots , n$です。

要素数が$n=2^h+1$の場合は、葉は高さh-1の2番目の要素から最後の要素までと高さhの最初の要素です。
つまり$2^{h-1}+1$ から $2^h-1$と$2^h$ から $2^h+1$です。これは$\lfloor n/2 \rfloor +1, \lfloor n/2 \rfloor+2, \cdots , n$です。

要素数が$n=2^h+2$の場合は、葉は高さh-1の3番目の要素から最後の要素までと高さhの最初の要素です。
つまり$2^{h-1}+2$ から $2^h-1$と$2^h$ から $2^h+2$です。これは$n/2+1, n/2+2, \cdots , n$です。

要素数が$n=2^h+3$の場合は、葉は高さh-1の2番目の要素から最後の要素までと高さhの最初の要素です。
つまり$2^{h-1}+2$ から $2^h-1$と$2^h$ から $2^h+3$です。これは$\lfloor n/2 \rfloor +1, \lfloor n/2 \rfloor+2, \cdots , n$です。

以降も同様になり、要素数が最大の$n=2^{h+1}-1$の場合は、葉は高さhの1番目の要素から最後の要素までです。
つまり$2^h$ から $2^{h+1}-1$です。これは$\lfloor n/2 \rfloor +1, \lfloor n/2 \rfloor+2, \cdots , n$です。

よって全ての要素数$n$に対して、葉は$\lfloor n/2 \rfloor +1, \lfloor n/2 \rfloor+2, \cdots , n$になります。

## 6.2 ヒープ条件の維持
maxヒープ条件を満たすように配列を並べ替えるアルゴリズムをRustで実装します。

本書と異なり配列のindexは0から開始するのでleft関数とright関数の計算は1を加算しています。
つまり本書では親ノードから子ノードへのindexの増加が、1->1*2と1*2+1になっているのに対して、0->0*2+1と0*2+2に修正します。

本書の例では4のindexは2ですが、indexが0から開始するのならば1になります。

```rust
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

fn main() {
    let mut v: Vec<i32> = vec![16, 4, 10, 14, 7, 9, 3, 2, 8, 1];
    println!("v={:?}", &v);

    max_heap(&mut v, 1);

    println!("sorted v={:?}", &v);
}
```

この再帰アルゴリズムの計算量は


```math
\begin{eqnarray}
T(n) \leq T(2n/3) + \Theta(1)
\end{eqnarray}
```

になります。
$A[i]$が右と左のどちらに分岐するかの計算量が$\Theta(1)$です。
そして再帰呼び出しは、最悪の場合、最も下のレベルが埋まっているので$2n/3$の要素について実行されます。

例えば要素$A[i]$から最下段までの高さが$h$として、高さ$h-1$までの要素数は$2^h-1$です。要素$A[i]$を除いた半分の要素は$2^{h-1}-1$です。
高さ$h$番目が半分埋まっている場合の要素数は$2^{h-1}$です。
全要素数は$n=3*2^{h-1}-1$であり、再帰呼び出しには$2*2^{h-1}-1$個の要素が使用されるので、おおよそ$2n/3$個となります。

## A6.2-1
以下の順番で要素3が移動します。

[27, 17, 3, 16, 13, 10, 1, 5, 7, 12, 4, 8, 9, 0]
[27, 17, 10, 16, 13, 3, 1, 5, 7, 12, 4, 8, 9, 0]
[27, 17, 10, 16, 13, 9, 1, 5, 7, 12, 4, 8, 3, 0]


## A6.2-2

```rust
fn left(i: usize) -> usize {
    2*i+1
}

fn right(i: usize) -> usize {
    2*i+2
}

fn min_heap(v: &mut Vec<i32>, i: usize) {
    let l = left(i);
    let r = right(i);
    let heap_size = v.len()-1;
    let mut smallest;
    let temp = v[i];
    
    if l <= heap_size && v[l] < v[i] {
        smallest = l;
    } else {
        smallest = i;
    }
    

    if r <= heap_size && v[r] < v[smallest] {
        smallest = r;
    } 
    
    if smallest != i {
        v[i] = v[smallest];
        v[smallest] = temp;
        println!("v={:?}, smallest={}, i={}", &v, smallest, i);
        min_heap(v, smallest);
    }
}
fn main() {
    let mut v: Vec<i32> = vec![1,9,2,3,4,5,6,7,8,10,11,12];
    println!("v={:?}", &v);

    min_heap(&mut v, 1);

    println!("sorted v={:?}", &v);
}
```

実行時間はmaxヒープと同じです。

## A6.2-3

要素$A[i]$が左右両方の子より大きければ、左側の子要素についての条件分岐で以下が実行され

```rust
    } else {
        largest = i;
    }    
```


右側の子要素についての条件分岐が当てはまらず、以下の行に飛びます。


```rust
    if largest != i {
```

$largest = i$なので、そこで関数が終了します。


## A6.2-4

$i$がヒープサイズの半分より大きい場合、本書とは異なり$i$が0から開始ならば$i > A.heap-size/2 -1$の場合、left関数もright関数もヒープサイズを超える値を返します。
すると前問と同様に


```rust
    } else {
        largest = i;
    }    
```


へ分岐後、以下の行に飛びます。


```rust
    if largest != i {
```

$largest = i$なので、そこで関数が終了します。

## A6.2-5

```rust
fn left(i: usize) -> usize {
    2*i+1
}

fn right(i: usize) -> usize {
    2*i+2
}

fn max_heap(v: &mut Vec<i32>, i: usize) {
    let mut l;
    let mut r;
    let mut index = i;
    let heap_size = v.len()-1;
    let mut largest;
    let mut temp;
    
    while index < heap_size {
        l = left(index);
        r = right(index);
        temp = v[index];
    
        if l <= heap_size && v[l] > v[index] {
            largest = l;
        } else {
            largest = index;
        }    
    
        if r <= heap_size && v[r] > v[largest] {
            largest = r;
        } 
        
        if largest != index {
            v[index] = v[largest];
            v[largest] = temp;
            index = largest;
            println!("v={:?}, largest={}, index={}", &v, largest, index);
        } else {
            return;
        }
    }
}

fn main() {
    let mut v: Vec<i32> = vec![16, 4, 10, 14, 7, 9, 3, 2, 8, 1];
    println!("v={:?}", &v);

    max_heap(&mut v, 1);

    println!("sorted v={:?}", &v);
}
```

## A6.2-6

最悪の場合、根から葉まで高さ$h=\lg n$回の計算を実行するので$\Omega(\lg n)$

# 6.3　ヒープの構築

```rust
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

fn main() {
    //let mut v: Vec<i32> = vec![16, 4, 10, 14, 7, 9, 3, 2, 8, 1];
    let mut v: Vec<i32> = vec![4,1,3,2,16,9,10,14,8,7];
    println!("v={:?}", &v);

    build_max_heap(&mut v);

    println!("sorted v={:?}", &v);
}
```


# 他の記事
- [個人的勉強メモ「アルゴリズム・イントロダクション」をRustで実装する：2章](https://qiita.com/Kosuke_Matsui/items/eea26e88cd261173a292)
- [個人的勉強メモ「アルゴリズム・イントロダクション」をRustで実装する：3章](https://qiita.com/Kosuke_Matsui/private/b424d8eb30dd16455b12)
- [個人的勉強メモ「アルゴリズム・イントロダクション」をRustで実装する：4章](https://qiita.com/Kosuke_Matsui/private/3b93d6b8a7ee5e90a2b5)
- [個人的勉強メモ「アルゴリズム・イントロダクション」をRustで実装する：6章](https://qiita.com/Kosuke_Matsui/private/8d1586a463a78081d533)
