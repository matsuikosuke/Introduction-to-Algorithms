# 記事の概要

アルゴリズムイントロダクション 第3版 総合版：世界標準MIT教科書のアルゴリズムをRustで実装しながら勉強します。

個人的な勉強メモなので他人様に見やすいものになっていません。
また、本の著作権を守るために、本を参照しながらでないと理解できないように書いています。

今回は2章について勉強しました。

# 2.1 挿入ソート

挿入ソートは配列の数字を順番に並び替えるためのアルゴリズムです。

対象の配列の値を１つ前の配列の値と比較して、対象の配列より値が大きければ配列の順番を交換し、対象の配列より値が小さければそこで終了します。
この操作を、配列の2番目からn番目までを順に実行します。
必ず2番目から実行します。3番目以降から実行すると順番に並びません。

```rust
fn main() {
    let mut a: [i8; 6] = [4, 2, 6, 1, 3, 5];
    println!("{:?}", &a[0..6]);
    
    for j in 1..a.len() {
        let key = a[j];
        let mut i = j;
        
        while i>0 && a[i-1]>key
        {
            a[i] = a[i-1];
            i = i-1;
        }
        a[i] = key;
    }
    println!("{:?}", &a[0..6]);
}
```

https://play.rust-lang.org/ において実行すると、以下の結果を得ます。

```console
[4, 2, 6, 1, 3, 5]
[1, 2, 3, 4, 5, 6]
```

また、ベクトルを用いれば以下のように実装できます。

```rust
fn insert_sort(v: &mut Vec<i32>) {
    for j in 1..v.len() {
        let key = v[j];
        let mut i = j;
    
        while i>0 && v[i-1]>key
        {
            v[i] = v[i-1];
            i = i-1;
        }
        v[i] = key;
    }
}

fn main() {
    let mut v: Vec<i32> = vec![4, 2, 6, 1, 3, 5];
    println!("v={:?}", &v);

    insert_sort(&mut v);

    println!("sorted v={:?}", &v);
}
```

## A2.1-1

```rust
fn main() {
    let mut a: [i8; 6] = [31, 41, 59, 26, 41, 58];
    println!("{:?}", &a[0..6]);
    
    for j in 1..a.len() {
        let key = a[j];
        let mut i = j;
        
        while i>0 && a[i-1]>key
        {
            a[i] = a[i-1];
            i = i-1;
        }
        a[i] = key;
    }
    println!("{:?}", &a[0..6]);
}
```

## A2.1-2
対象の配列の値を１つ後の配列の値と比較して、対象の配列より値が大きければ配列の順番を交換し、対象の配列より値が小さければそこで終了します。
この操作を、配列のn-1番目から1番目までを順に実行します。
必ずn-1番目から実行します。

```rust
fn main() {
    let mut a: [i8; 6] = [31, 41, 59, 26, 41, 58];
    println!("{:?}", &a[0..6]);
    
    for j in (0..a.len()-1).rev() {
        let key = a[j];
        let mut i = j;
        while i<a.len()-1 && a[i+1]>key
        {
            a[i] = a[i+1];
            i = i+1;
        }
        a[i] = key;
        println!("{:?}", &a[0..6]);
    }
    println!("{:?}", &a[0..6]);
}
```

for文で逆順`rev()`を使用して配列の後ろから前方向に計算しています。

## A2.1-3

```rust
fn search(v: i8)-> usize {
    let a: [i8; 6] = [31, 41, 59, 26, 41, 58];
    println!("{:?}", &a[0..6]);
    
    for i in 0..a.len() {
        if a[i] == v
        {
            return i+1;
        }
    }
    
    return 0;
}

fn main()
{
    let v = 26;
    println!("{}は{}番目の値", v, search(v));
    
    let v = 100;
    println!("{}は{}番目の値", v, search(v));
}
```

## A2.1-4
この問題は、意味が分かりにくいですが、要は2進数の加算器についての問題です。

例えば、0b10011+0b10110という2進数の和を、２つの整数の配列、A[5] = [1, 1, 0, 0, 1]とB[5]=[0, 1, 1, 0, 1]で表現した上で、和を求めさせるものです。

```rust
fn main() {
    let a: [i8; 5] = [1, 1, 0, 0, 1];
    let b: [i8; 5] = [0, 1, 1, 0, 1];
    let mut c: [i8; 6] = [0, 0, 0, 0, 0, 0];
    println!("{:?}", &a[0..5]);
    println!("{:?}", &b[0..5]);
    
    let mut carry = 0;
    
    for i in (0..a.len()).rev() {
        c[i] = (a[i]+b[i]+carry)%2;
        carry = (a[i]+b[i]+carry)/2;
    }
    c[a.len()] = carry;
    c = c.rev();
    
    println!("{:?}", &c[0..a.len()+1]);
}
```
計算結果を以下に示します。
AとBのn-1番目同士の和を取り、繰上りがあれば、n番目同士の和に繰上りも可算していることが分かります。
0b10011+0b10110=0b101001を表しています。

```console
[1, 1, 0, 0, 1]
[0, 1, 1, 0, 1]
[1, 0, 0, 1, 0, 1]
```

本の通りにすれば、任意のn要素配列を代入できるようにしないといけませんが、アルゴリズムの本質とは関係ないので、こちらで用意した配列を代入しました。

# 2.2 アルゴリズムの解析

## 挿入ソートの解析

for文は、判定を何回行ったかを計算します。
`for j in 1..a.len()`は1からn-1までn-1回の判定に加え、ループから抜ける際に1回の判定を行うので合計n回。

for文内で1回しか実行しない計算の計算回数はn-1回。
for文内での判定式は各ループ毎に判定回数が異なり、各ループで$t_j$回になる場合は、合計で$\sum_{j=1}^{n-1} t_j$回。
for文内での判定式において実行される計算は、ループから抜ける1回の判定回数を除いて、計算回数×$(t_j-1)$回になります。挿入ソートでは計算回数は1回なので合計で$\sum_{j=1}^{n-1} t_j$回。

最良の条件は、初めからソートされている場合であり、while以下の計算が0になります。

最悪の条件は、初めから逆順にソートされている場合であり、while以下の計算がkeyの添え字より小さい数字の添え字の全てについて計算しないといけないので、$t_j=j$となります。

## A2.2-1
計算量の次元は最大次数の項のみが影響するので、

```math
\begin{eqnarray}
n^3/1000 \in \Theta(n^3)
\end{eqnarray}
```

となります。

## A2.2-2
配列の1番目の要素を2番目からn番目の要素と比較し、最小の要素と1番目の要素を交換します。
配列の2番目の要素を3番目からn番目の要素と比較し、最小の要素と2番目の要素を交換します。
...
配列のn-2番目の要素をn-1番目からn番目の要素と比較し、最小の要素とn-2番目の要素を交換します。
配列のn-1番目の要素をn番目の要素と比較し、最小の要素とn-1番目の要素を交換します。

for loopはn-1番目まで実行すれば、残りのn番目の要素は必ず最大値なので、n個全ての要素について実行する必要はありません。

```rust
fn main() {
    let mut a: [i8; 6] = [4, 2, 6, 1, 3, 5];
    println!("{:?}", &a[0..6]);
    
    for j in 0..a.len()-1 {
        let mut key = j;
        let mut min = a[key];
        
        for i in j+1..a.len() {
            if min >  a[i]
            {
                key = i;
                min = a[i];
            }
        }
        a[key] = a[j];
        a[j] = min;
    }
    println!("{:?}", &a[0..6]);
}
```

計算量は、

`for j in 0..a.len()-1`が、要素数n-1回とloopを抜ける判定の1回で$n-1+1=n$回。
`let mut key = j;`がn-1回。
`let mut min = a[key];`がn-1回。
`for i in j+1..a.len()`が、各ループ毎に、要素数n-j-1回とloopを抜ける判定の1回で$n-j-1+1=n-j$ 回。その総和で$\sum_{j=0}^{n-1}(n-j)$
`if min >  a[i]`が各ループ毎に、n-j-1回。その総和で$\sum_{j=0}^{n-1}(n-j-1)$回。
`key = i;`が各ループ毎に、n-j-1回。その総和で$\sum_{j=0}^{n-1}(n-j-1)$回。
`min = a[i];`が各ループ毎に、n-j-1回。その総和で$\sum_{j=0}^{n-1}(n-j-1)$回。
`a[key] = a[j];`がn-1回。
`a[j] = min;`がn-1回。


最良の条件は、初めからソートされている場合であり、`key = i;`と`min = a[i];`の計算が0になります。

最悪の条件は、初めから逆順にソートされている場合であり、`key = i;`と`min = a[i];`の計算を毎回しないといけません。

ですが、最良の場合も、最悪の場合も、必ず`for i in j+1..a.len()`と`if min >  a[i]`の計算を実行するので、どちらの場合でも計算の次元は$\Theta(n^2)$になります。

```math
\begin{eqnarray}
\sum_{j=0}^{n-1}(n-j-1) &=& \frac{((n-0-1)+(n-n+1-1))*n}{2} \\
&=& \frac{(n-1)*n}{2} 
\end{eqnarray}
```

直観的に考えると、挿入式は配列の並び方の運が良いと途中でループから抜け出ることができ、選択式はどんな配列でも全部比較しないといけないのでループを最後まで抜け出せないという違いです。


## A2.2-3

挿入ソートのアルゴリズムについて、配列要素に同じ数字がある場合の計算量の違いを比較する問題です。
配列内に同じ数字があれば、keyの数字が同じ数字と比較した時点でループから抜けて、`while i>0 && a[i-1]>key`以下を実行しないで済むので、それだけ計算量が減ります。

最良の場合は配列の要素が全て同じ数字の場合で、最初からソートされている場合と同じ計算量になります。
最悪の場合は配列の要素が全て異なる場合です。

例えば、配列が[3,1,2,1,1,2,4,1,5,2]だった場合を考えます。
最初に1をkeyにして比較している時、`while i>0 && a[i-1]>key`以下の計算を1回も省略できません。
次に2をkeyにして比較している時、`while i>0 && a[i-1]>key`以下の計算を1回も省略できません。
次に1をkeyにして比較している時、3回目の比較対象に1が来た時点で、`while i>0 && a[i-1]>key`以下の計算を省略できます。
次に1をkeyにして比較している時、3回目の比較対象に1が来た時点で、`while i>0 && a[i-1]>key`以下の計算を省略できます。

つまり計算量は、配列内に同じ要素が幾つあるか、そしてその要素が比較対象になるかどうかに影響されます。
この２つの条件を満たす確率は、上記の例のように、短い配列だと確率の差が大きいでしょうが、膨大な長さのランダムな配列ならば一定の確率（1/10）に収束すると思われます。
これが問題文中で「探索すべき要素が配列中の任意の要素と等確率で一致する」と表現されていることです。
この確率を$p$とします。


keyの要素との比較が、1回目で一致して終了する確率、2回目で一致して終了する確率、3回目で一致して終了する確率、...、(n-1)回目で一致して終了する確率、そして最後まで一致しない場合の、計算回数の期待値の総和を求めます。

まずkeyの要素を$j$個の要素と比較することになる確率を考えます。こうなる確率は$j-1$回は一致せず、1回一致したということなので、一致しない確率の(j-1)乗と一致する確率の積、$(1-p)^{j-1}*p$ です。

この時の計算回数の期待値は、j回の計算をしているので、$j*(1-p)^{j-1}*p$です。

またkeyの要素がn-1回の計算で1回も一致しない確率は$(1-p)^{n-1}$です。計算回数の期待値は$n-1$回の計算をしているので、$(n-1)*(1-p)^{n-1}$です。

よって期待値の総和は、

```math
\begin{eqnarray}
計算回数の期待値 
&=& (n-1)*(1-p)^{n-1} + \sum_{j=1}^{n-1} j*(1-p)^{j-1}*p \\
&=& (n-1)*(1-p)^{n-1} + \sum_{i=1}^{n-1} \sum_{j=i}^{n-1} (1-p)^{j-1}*p \\
&=& (n-1)*(1-p)^{n-1} + \sum_{i=1}^{n-1}  \frac{(1-p)^{i-1} - (1-p)^{n-1}}{1-(1-p)} *p \\
&=& (n-1)* (1-p)^{n-1} + \sum_{i=1}^{n-1}  \Bigl((1-p)^{i-1} - (1-p)^{n-1} \Bigl) \\
&=& (n-1)* (1-p)^{n-1} + \sum_{i=1}^{n-1}  (1-p)^{i-1} - (n-1)* (1-p)^{n-1} \\
&=& \frac{1 - (1-p)^{n-1}}{1-(1-p)} \\
&=& \frac{1}{p} - \frac{(1-p)^{n-1}}{p} \\
\end{eqnarray}
```

です。つまり、これが問題文中の「入力列の中で調べられる要素数の平均」です。
この期待値の計算量の次元は$\Theta(1)$です。

## A2.2-4

挿入ソートの事例を見れば、可能な限り条件判定を早めに終わらせ、ループを抜け出して終了できるアルゴリズムが最良実行時間を持つアルゴリズムと言えそうです。

# 2.3 アルゴリズムの設計

## マージソート

既にソート済みの配列が2つあると仮定します。
1つは長さ$n_1 = q-p+1$の配列A[p],A[p+1]...,A[q]とします。
もう1つは長さ$n_2 = r-q$の配列A[q+1],A[q+2]...,A[r]とします。
この2つの配列をつなげて、再ソートするアルゴリズムを考えます。

まず、配列のコピーを作成します。
配列A[p],A[p+1]...,A[q]を配列L[0]...L[q-p]にコピーします。
配列A[q+1],A[q+2]...,A[r]を配列R[0]...R[r-q-1]にコピーします。

この時点で3つの配列ができています。LとRと長さ$n_1+n_2$の配列A[p],A[p+1]...,A[r]です。

次に、配列Lと配列Rの最後に要素を追加し、L[q-p+1]とL[r-q]に無限大の値を持つ番兵を置きます。
実際のプログラムでは、無限大はないので、最大値を格納します。

まずA[p]に注目します。A[p]はL[0]の値に等しいです。
L[0]とR[0]と比較して、もしL[0]がR[0]以下ならば、A[p]にL[0]を代入して、次のサイクルではL[1]とR[0]を比較します。
もしL[0]がR[0]より大きいならば、A[p]にR[0]を代入して、次のサイクルではL[0]とR[1]を比較します。

値を代入したら、次のサイクルに移ります。A[p+1]はL[1]の値に等しいです。
前のサイクルの結果に依存してL[1]とR[0]もしくはL[0]とR[1]と比較して、もしL[x]がR[y]以下ならば、A[p+1]にL[x]を代入して、次のサイクルではL[x+1]とR[y]を比較します。（ここでxとyは0 or 1）
もしL[x]がR[y]より大きいならば、A[p+1]にR[y]を代入して、次のサイクルではL[x]とR[y+1]を比較します。


サイクルの途中で、必ずL[x]かR[y]のどちらかが先に番兵の無限大に到達します。
例えば、先にL[x]が無限大に到達すると、以降のサイクルでの比較は常にL[y]が小さくなるので、L[y+1],L[y+2]...とLの配列番号だけが可算されて、番兵に到達するまで続きます。

以下に、ソート済みの配列[1, 10, 100]と[5, 11, 35]をソートする場合をRustで実装してみます。

```rust
fn merge(v: &mut Vec<i32>, p: usize, q: usize, r: usize) {//p=0,q=2,r=5
    println!("v={:?}, p={}, q={}, r={}", &v[0..r+1], p,q,r);
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
    
    println!("vl={:?}", &vl);
    println!("vr={:?}", &vr);
    
    let mut index_al = 0;
    let mut index_ar = 0;
    
    for k in p..r+1 {
        if vl[index_al] <= vr[index_ar] {
            v[k] = vl[index_al];
            println!("vl[{}]={}->v[{}], v={:?}", index_al, vl[index_al], k, &v[0..r+1]);
            index_al += 1;
        } else {
            v[k] = vr[index_ar];
            println!("vr[{}]={}->v[{}], v={:?}", index_ar, vr[index_ar], k, &v[0..r+1]);
            index_ar += 1;
        }
    }
}

fn main() {
    let mut v: Vec<i32> = vec![1, 10, 100, 5, 11, 35];
    println!("v={:?}", &v);
    
    let len = v.len();
    let r = len-1;
    merge(&mut v, 0, 2, r);
    
    println!("sorted v={:?}", &v);
}
```

ここまでの話では、ソート済みの2つの配列が与えられた場合に、その2つの配列を結合してソートしました。
次は、ソート済みではない任意の配列をソートします。

まず配列を2つに分割し、それぞれの配列にマージソートを適用してソート済みにさせてから、この2つの配列をマージします。
この処理をマージソートと呼びます。

今の説明で分かるように、マージソート処理の内部でマージソートを行っています。

マージソートを再帰的に繰り返すことで、最小要素に分解されるまで分割を行い、最下層から順にマージソートを適用してマージして、階層を昇りながらソートしていきます。

Rustで書くと以下のようになります。
ここで`merge`関数は先述のものを使用します。

```rust
fn merge_sort(v: &mut Vec<i32>, p: usize, r: usize) {
    if p<r {
        let q = (p+r)/2;
        merge_sort(v, p, q);
        merge_sort(v, q+1, r);
        merge(v, p, q, r);
    }
}

fn main() {
    let mut v: Vec<i32> = vec![3,7,100,5,9,1,99,12,45,76];
    println!("v={:?}", &v);
    
    let len = v.len();
    let r = len-1;
    //merge(&mut v, 0, 2, r);
    merge_sort(&mut v, 0, r);
    
    println!("sorted v={:?}", &v);
}
```

## マージソートの解析

マージソートアルゴリズムは、決められた処理を再帰的に繰り返します。
その処理の計算時間はサイズに依存するとして$T(n)$と表現します。
サイズ$n$の処理内では、サイズ$n/2$の決められた処理が2回繰り返されます。そして問題を分割する計算時間を$D(n)$、分割した問題を再結合するマージ処理の計算時間を$C(n)$とすると、本に記載されている総計算時間の漸化式が求まります。

マージソートの分割処理は、和と割り算を1回ずつだけなので、定数のオーダー$\Theta(1)$になります。

結合処理`merge`はサイズの1次のオーダー$\Theta(n)$になります。

マスター定理を用いると、これは$T(n)=\Theta(n\log n)$になり$n^2$のオーダーより計算時間が短くなります。

マージソートの場合は$\Theta(n)=cn$として計算すると以下になります。

```math
\begin{eqnarray}
T(n)
&=& 2T(n/2)+cn \\
&=& 2^2 T(n/2^2) + 2cn/2 + cn \\
&=& 2^3 T(n/2^3) + 2^2cn/2^2 +  cn + cn \\
&=& 2^4 T(n/2^4) + 4cn \\
&=& 2^5 T(n/2^5) + 5cn \\
&=& 2^6 T(n/2^6) + 6cn \\
&& ... \\
&=& 2^k T(n/2^k) + kcn \\
&& ... \\
&=& (再帰レベルの数)*cn 
\end{eqnarray}
```

再帰レベルの数が$\lg n+1$であることは漸化式で証明できます。

$n=1$ の時は、再帰レベルの数は1であり、$\lg 1=0$なので$\lg 1 + 1=1$となり正しいです。

サイズを$n-1=2^{k-1}$とした場合、再帰レベルの数が$\lg2^{k-1}+1$であるとします。
そして再帰レベルの数を1つ増やすと、そのレベルでのサイズは$n=2^k$となります。

```math
\begin{eqnarray}
&& サイズnの再帰レベルの数 \\
&=& サイズ(n-1)の再帰レベルの数+1 \\
&=& \lg 2^{k-1}+1+1 \\
&=& k-1+2 \\
&=& k+1 \\
&=& \lg 2^k+1 \\
&=& \lg n+1 \\
\end{eqnarray}
```

となります。
以上で証明完了です。

これよりマージソートの計算の次元は $\Theta(n \lg n)$ であることが分かります。

## A2.3-1
以下を実行しログを取得します。

```rust
fn main() {
    let mut v: Vec<i32> = vec![3,41,52,26,38,57,9,49];
    println!("v={:?}", &v);
    
    let len = v.len();
    let r = len-1;
    merge_sort(&mut v, 0, r);
    
    println!("sorted v={:?}", &v);
}
```

vが4つの組、[3, 41], [52, 26], [38, 57], [9, 49]にまで分割され、`merge_sort`関数によりソートされてから、`merge`関数により2組ずつ結合されて[3, 41, 26, 52]と[38, 57, 9, 49]になり、再ソートされてから再結合されて[3, 26, 41, 52, 9, 38, 49, 57]になり、最終的なソートをされるのが分かります。

```console
v=[3, 41, 52, 26, 38, 57, 9, 49]
v=[3, 41], p=0, q=0, r=1
vl=[3, 65535]
vr=[41, 65535]
vl[0]=3->v[0], v=[3, 41]
vr[0]=41->v[1], v=[3, 41]
v=[3, 41, 52, 26], p=2, q=2, r=3
vl=[52, 65535]
vr=[26, 65535]
vr[0]=26->v[2], v=[3, 41, 26, 26]
vl[0]=52->v[3], v=[3, 41, 26, 52]
v=[3, 41, 26, 52], p=0, q=1, r=3
vl=[3, 41, 65535]
vr=[26, 52, 65535]
vl[0]=3->v[0], v=[3, 41, 26, 52]
vr[0]=26->v[1], v=[3, 26, 26, 52]
vl[1]=41->v[2], v=[3, 26, 41, 52]
vr[1]=52->v[3], v=[3, 26, 41, 52]
v=[3, 26, 41, 52, 38, 57], p=4, q=4, r=5
vl=[38, 65535]
vr=[57, 65535]
vl[0]=38->v[4], v=[3, 26, 41, 52, 38, 57]
vr[0]=57->v[5], v=[3, 26, 41, 52, 38, 57]
v=[3, 26, 41, 52, 38, 57, 9, 49], p=6, q=6, r=7
vl=[9, 65535]
vr=[49, 65535]
vl[0]=9->v[6], v=[3, 26, 41, 52, 38, 57, 9, 49]
vr[0]=49->v[7], v=[3, 26, 41, 52, 38, 57, 9, 49]
v=[3, 26, 41, 52, 38, 57, 9, 49], p=4, q=5, r=7
vl=[38, 57, 65535]
vr=[9, 49, 65535]
vr[0]=9->v[4], v=[3, 26, 41, 52, 9, 57, 9, 49]
vl[0]=38->v[5], v=[3, 26, 41, 52, 9, 38, 9, 49]
vr[1]=49->v[6], v=[3, 26, 41, 52, 9, 38, 49, 49]
vl[1]=57->v[7], v=[3, 26, 41, 52, 9, 38, 49, 57]
v=[3, 26, 41, 52, 9, 38, 49, 57], p=0, q=3, r=7
vl=[3, 26, 41, 52, 65535]
vr=[9, 38, 49, 57, 65535]
vl[0]=3->v[0], v=[3, 26, 41, 52, 9, 38, 49, 57]
vr[0]=9->v[1], v=[3, 9, 41, 52, 9, 38, 49, 57]
vl[1]=26->v[2], v=[3, 9, 26, 52, 9, 38, 49, 57]
vr[1]=38->v[3], v=[3, 9, 26, 38, 9, 38, 49, 57]
vl[2]=41->v[4], v=[3, 9, 26, 38, 41, 38, 49, 57]
vr[2]=49->v[5], v=[3, 9, 26, 38, 41, 49, 49, 57]
vl[3]=52->v[6], v=[3, 9, 26, 38, 41, 49, 52, 57]
vr[3]=57->v[7], v=[3, 9, 26, 38, 41, 49, 52, 57]
sorted v=[3, 9, 26, 38, 41, 49, 52, 57]
```



## A2.3-2

```rust
fn merge(v: &mut Vec<i32>, p: usize, q: usize, r: usize) {//p=0,q=2,r=5
    println!("v={:?}, p={}, q={}, r={}", &v[p..r+1], p,q,r);
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
    
    println!("sorted v={:?}", &v);
}
```

## A2.3-3
$n=2$ の場合、$T(2)=2\lg 2=2$が成立します。

$n-1=2^{k-1}$ の場合、$T(n-1)=2^{k-1}\lg 2^{k-1}$が成立するとして、サイズが$n=2^k$の$T(n)$を求めると以下になります。

```math
\begin{eqnarray}
T(2^k) 
&=& 2T(2^{k-1})+2^k \\
&=& 2*2^{k-1}\lg2^{k-1}+2^k \\
&=& 2^k (k-1)+2^k \\
&=& 2^k k \\
&=& 2^k\lg 2^k \\
\end{eqnarray}
```

よって与式が成立します。
以上で証明完了です。

## A2.3-4
ある配列が、最後の要素を除いてソートされているとして、最後の要素についてだけ挿入ソートする場合を考えます。

```rust
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

fn main() {
    let mut v: Vec<i32> = vec![1, 5, 10, 11, 100, 6];
    println!("v={:?}", &v);
    
    insert_last(&mut v);

    println!("sorted v={:?}", &v);
}
```

次に再帰的にこのアルゴリズムを繰り返して、任意の配列をソートしてみます。

```rust
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
```

配列のサイズが1より大きい限り、最後の要素を除いた配列を挿入ソート関数に代入してから、最後の要素を戻します。
それから最後の要素について挿入ソートを適用します。

これにより、再帰的にサイズが2になるまで最後の要素を除き、サイズが2の配列に挿入ソートを適用して関数を抜けて、配列の最後尾に要素が追加されてから挿入ソートが適用されて関数を抜けて、配列の最後尾に要素が追加されてから挿入ソートが適用されて関数を抜けて、配列の最後尾に要素が追加されてから挿入ソートが適用されて関数を抜けて...を繰り返します。

ではこのアルゴリズムの計算時間 $T(n)$ を漸化式で求めます。
再帰アルゴリズムの計算は、分割、統治、結合の3つから成ります。

分割はpopで最後の要素を取り出す計算で、計算の次元は $\Theta(1)$ です。
統治は、要素が1つ減った $T(n-1)$ になります。
結合はpushとそれに続くソート関数の適用です。

今回のソート関数は、以前のソート関数と異なりforの繰り返しがないので、計算の次元は $\Theta(n)$ になります。

```math
\begin{eqnarray}
T_{insert_last}(n) &=& c_1 n + c_2 (n-1) + c_4(n-1) + c_5 n + c_6 (n-1) + c_7 (n-1) + c_8 (n-1)
\end{eqnarray}
```

よって漸化式は以下になります

```math
\begin{eqnarray}
T(n) &=& \Theta(1) + T(n-1) + \Theta(n) \\
&=& T(n-1) + \Theta(n)
\end{eqnarray}
```

## A2.3-5

```rust
fn search(x: i32, v: &mut Vec<i32>)-> usize {
    //println!("v={:?}", &v);
    let mut start_index;
    let mut end_index;
    let mut search_index;

    start_index = 0;
    end_index = v.len()-1;

    //println!("v={:?}, x={}, end_index={}", &v, x, end_index);

    loop {
        search_index = start_index + (end_index-start_index)/2;
        println!("search_index={}", search_index);

         if x == v[search_index] {
            return search_index+1;
         } else if x > v[search_index] {
            if search_index == end_index {
                return 0;
            } else {
                start_index = search_index+1;
            }
        } else {
            if search_index == start_index {
                return 0;
            } else {
                end_index = search_index-1;
            }
        }
    }
}

fn main()
{
    let mut v: Vec<i32> = vec![26, 31, 41, 41, 59, 58];
    
    let x = 26;
    println!("{}は{}番目の値", x, search(x, &mut v));
    
    let x = 31;
    println!("{}は{}番目の値", x, search(x, &mut v));
    
    let x = 41;
    println!("{}は{}番目の値", x, search(x, &mut v));
    
    let x = 59;
    println!("{}は{}番目の値", x, search(x, &mut v));

    let x = 100;
    println!("{}は{}番目の値", x, search(x, &mut v));
}
```


漸化式は、計算サイズは半分ずつになっていくので以下になります。

```math
\begin{eqnarray}
T(n) &=& T(n/2) + c
\end{eqnarray}
```

漸化式を展開し続けると、再帰レベルの数に比例することが分かります。

```math
\begin{eqnarray}
T(n)
&=& T(n/2)+c \\
&=& T(n/2^2) + 2c \\
&=& T(n/2^3) + 3c \\
&=& T(n/2^4) + 4n \\
&=& T(n/2^5) + 5n \\
&=& T(n/2^6) + 6n \\
&& ... \\
&=& T(n/2^k) + kc \\
&& ... \\
&=& (再帰レベルの数)*c 
\end{eqnarray}
```

再帰レベルの数はマージソートの場合と同じなので $\lg n$ です。

よって計算の次元は $\Theta(\lg n)$ になります。

## A.2.3-6

ある配列が、最後の要素を除いてソート済みとします。
ソート済みの配列の中央の要素と最後の要素を比較します。
中央の要素が大きい場合、もしくは値が等しい場合は、最後の要素により近い値がまだ左半分にあるかもしれません。

例えば、[1, 5, 10, 20（比較値）,30, 40, 50, 2] ならば [1, 5, 10]に2に近い要素があります 

最後の要素が大きい場合は、最後の要素により近い値がまだ右半分にあるかもしれません。

例えば、[1, 5, 10, 20（比較値）,30, 40, 50, 45]ならば [30, 40, 50]に45に近い要素があります 

そこで更に、その半分の中央の要素と大小を比較して、また右半分か左半分を選択し、その半分の中央の要素と大小を比較してということを繰り返します。

この繰り返しは、比較する要素のインデックスがソート済み配列の最初か最後になるまで続けます。


例えば、[1, 5（比較値）, 10, 2] ならば 、次の比較要素はソート済み配列の最初の要素である[1]になり、この比較で[1,2]の順にソートされ終了です。 
例えば、[1, 5（比較値）, 10, 6] ならば 、次の比較要素はソート済み配列の最後の要素である[10]になり、この比較で[6,10]の順にソートされ終了です。 

任意の配列に対して、最初の2要素について上記のソートを行い、次にソート済みの2要素と3番目の要素について、その次にソート済みの3要素と4番目の要素について...と同様のことをソート済みのn-1要素とn番目の要素になるまで繰り返します。

```rust
fn insert_sort(v: &mut Vec<i32>) {
    //println!("v={:?}", &v);
    let mut start_index;
    let mut end_index;
    let mut search_index;

    for j in 1..v.len() {
        let key = v[j];
        v.remove(j);
        
        start_index = 0;
        end_index = j-1;
        
        println!("v={:?}, key={}, end_index={}", &v, key, end_index);
        
        loop {
            search_index = start_index + (end_index-start_index)/2;
            println!("search_index={}", search_index);
            
            if key > v[search_index] {
                if search_index == end_index {
                    v.insert(search_index+1, key);
                    break;
                } else {
                    start_index = search_index+1;
                }
            } else {
                if search_index == start_index {
                    v.insert(search_index, key);
                    break;
                } else {
                    end_index = search_index-1;
                }
            }
        }
    }

    //println!("v={:?}", &v);
}

fn main() {
    let mut v: Vec<i32> = vec![4, 2, 6, 1, 3, 5];
    println!("v={:?}", &v);

    insert_sort(&mut v);

    println!("sorted v={:?}", &v);
}
```

配列を半分にしていくことを繰り返す場合の計算の次元は、 A2.3-5 の場合と同様に $\Theta(\lg n)$ です。
それをforループでn-1回繰り返すので、このアルゴリズムの計算の次元は、 $\Theta(n \lg n)$ になります。

## A2.3-7
まずは配列をソートします。この計算の次元は $\Theta(n \lg n)$ になります。

ソート済み配列から１つの要素 $v[i]$ を選んで、目標値 $x$ との差分 $x-v[i]$ を取ります。そして、残りの配列要素から $x-v[i]$ と一致する値を探します。

ソート済み配列から一致する値を探すのは、A2.3-5の探索アルゴリズムと同じです。その計算次元は $\Theta(\lg n)$ です。

一致する値がなければ、要素 $v[i]$ を削除した配列から、再度 $v[i+1]$ を選んで同じことを繰り返します。

一致する値を見つけるか、比較する要素がなくなれば（配列のサイズがnならば、n-1回の要素選択をするまで）、繰り返しは終了します。

この繰り返しの計算の次元は、 $\Theta(n)$ になります。

よって全体の計算の次元は $\Theta(n \lg n)$ になります。

```rust
fn search_sum(x: i32, v: &mut Vec<i32>) {
    //println!("v={:?}", &v);
    let mut start_index;
    let mut end_index;
    let mut search_index;

    for j in 0..v.len()-1 {
        let key = x-v[j];

        start_index = j;
        end_index = v.len()-1;
        //println!("v={:?}, x={}, end_index={}", &v, x, end_index);
    
        loop {
            search_index = start_index + (end_index-start_index)/2;
            //println!("search_index={}", search_index);
    
             if key == v[search_index] {
                println!("{} = v[{}]({}) + v[{}]({})", x, j, v[j], search_index, v[search_index]);
                return ;
             } else if key > v[search_index] {
                if search_index == end_index {
                    break;
                } else {
                    start_index = search_index+1;
                }
            } else {
                if search_index == start_index {
                    break;
                } else {
                    end_index = search_index-1;
                }
            }
        }
    }
    
    println!("notihng");
}

fn main()
{
    let mut v: Vec<i32> = vec![31, 41, 26, 59, 41, 58];
    println!("v={:?}", &v);
    
    insert_sort(&mut v);
    println!("v={:?}", &v);
    
    let x = 1000; //85; //100; // 
    search_sum(x, &mut v);
}
```

ここでA.2.3-6の `insert_sort`関数を使用しています。


## A2-1
`insert_merge_sort`関数は引数kで指定した回数だけ分割を行います。
0を代入すれば1回も分割をせずに、挿入ソートを実行します。
tは分割回数で、必ず最初は0を代入します。


```rust
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
```

### a

まず、サイズkの挿入ソートの最悪計算時間は  $\Theta(k^2)$ です。それがn/k個あるので $\Theta(k^2*n/k) = \Theta(nk)$ になります。

### b

マージソートの最悪計算時間は、再帰レベルの数に比例します。
サイズnの再帰レベルの数は $\lg n$  です。
サイズnをn/k個までに分割したのならば、そこに至るまでの再帰レベルの数は $\lg k$  です。

よってソート済みのn/k個の再帰レベルの数は、サイズnの再帰レベルの数からサイズkの再帰レベルの数を引いたもので以下になります

```math
\begin{eqnarray}
\lg n - \lg k &=& \lg n/k
\end{eqnarray}
```

そしてサイズkの配列の計算の次元は $k = cn$ なので  $\Theta(n)$ になります。よってマージソートの最悪計算時間は $\Theta(n \lg (n/k))$  になります。

### c

$\Theta(nk + n \lg (n/k)) = \Theta(n \lg n)$ と仮定すれば


```math
\begin{eqnarray}
&& \Theta(nk + n \lg (n/k)) = \Theta(n \lg n) \\
&& \Theta(k + \lg (n/k)) = \Theta(\lg n) \\
&& \Theta(k) + \Theta(\lg n) - \Theta(\lg k) = \Theta(\lg n) \\
&& \Theta(k) + \Theta(\lg n) = \Theta(\lg n) \\
\end{eqnarray}
```

よって $\Theta(k)$ も  $\Theta(\lg n)$ に比例していないと、上記等式は成立しません。

```math
\begin{eqnarray}
k \propto \lg n
\end{eqnarray}
```

### d

kの極大値を求めればいいので、


```math
\begin{eqnarray}
0 &=& (c_1 nk + c_2 n \lg(n/k))' \\
&=& c_1 n + c_2 n \frac{k}{n} \frac{(-n)}{k^2} \\
&=& c_1 n - c_2  \frac{n}{k} \\
&=& c_1 -  \frac{c_2}{k} \\
\end{eqnarray}
```

を満たすようにkを選びます
