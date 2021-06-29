# 記事の概要
アルゴリズムイントロダクション 第3版 総合版：世界標準MIT教科書のアルゴリズムをRustで実装しながら勉強したので、その結果を整理しました。
まとめた内容には確実に誤りがあると思われるので、参照にあたっては、あらかじめご了承ください。

個人的な勉強メモなので他人様に見やすいものになっていません。
また、本の著作権を守るために、本を参照しながらでないと理解できないように書いています。

今回は4章について勉強しました。

RUSTコードの動作は以下のサイトで確認できます。
https://play.rust-lang.org/

# 4.1　分割統治

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

計算量は、配列の要素数がnとして、差分配列の要素数はn-1になります。n-1個から2個の要素を選ぶ全ての組み合わせを計算するので理論上の計算数は ${}_{n-1}C_2$ に比例します。

実際のアルゴリズムにおいては、forループが $\sum_{i=1}^{n-2} i = (n-2)*(n-1)/2$ 回の計算を行うので$\Theta(n^2)$ になります。

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

次にfind_maximum_subarrayの計算量$T(n)$を考えます。
以下の部分は定数時間です。

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

以下の関数find_max_cross_subarrayは$\Theta(n)$です。

```rust
let (cross_low, cross_high, cross_sum) = find_max_cross_subarray(&v, low, mid, high);
```

以下の関数find_maximum_subarrayは再帰的な処理で、要素数は半分になり、2つあるので$2T(n/2)$です。

```rust
let (left_low, left_high, left_sum) = find_maximum_subarray(&v, low, mid);
let (right_low, right_high, right_sum) = find_maximum_subarray(&v, mid+1, high);
```

よって漸化式は、計算量に寄与しない定数時間の計算を無視すると

```math
\begin{eqnarray}
T(n) = 2T(n/2) + \Theta(n)
\end{eqnarray}
```

となります。マスター法を用いると、この計算の次元は$\Theta(n \lg n)$になります。

## A4.1-1

例えば以下の配列を用いると、差分配列の全ての要素が負になります。

```rust
let v: Vec<i32> = vec![900, 850, 800, 750, 700, 650, 600, 550, 500, 450, 400, 350, 300, 250, 200, 150, 100];
```

これによりfind_maximum_subarrayの返り値は、常にlow要素になります。開始要素と終了要素もlowを返します。

## A4.1-2

[最大部分配列の総当たり](## 最大部分配列の総当たり戦略)をご参照ください。

## A4.1-3

計算時間を比較するプログラムを作成しました。

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

    //println!("{}日目終了時に購入し、{}日目終了時に売却すると、最大の利益{}を得る", start, end, max);
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
        //println!("{}日目終了時に{}ドルで購入し、{}日目終了時に{}ドルで売却すると、最大の利益{}を得る", low, v[low], high+1, v[high+1], sum);
        find_stop = Instant::now();
        
        
        round_start = Instant::now();
        find_max_round_robin(&dv);
        round_stop = Instant::now();
        
        println!("要素数  :  総当たり戦略  :  分割統治 = {}  :  {:?}  :  {:?}", v.len(), round_stop.duration_since(round_start), find_stop.duration_since(find_start));
        
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


以下のような結果になりました。
要素数が15くらいまでは、ランダムな値によって大小が逆転しました。

```Console
要素数  :  総当たり戦略  :  分割統治 = 2  :  822ns  :  1.51?s
要素数  :  総当たり戦略  :  分割統治 = 3  :  1.446?s  :  2.849?s
要素数  :  総当たり戦略  :  分割統治 = 4  :  2.14?s  :  1.845?s
要素数  :  総当たり戦略  :  分割統治 = 5  :  2.088?s  :  2.333?s
要素数  :  総当たり戦略  :  分割統治 = 6  :  2.975?s  :  3.357?s
要素数  :  総当たり戦略  :  分割統治 = 7  :  3.31?s  :  3.885?s
要素数  :  総当たり戦略  :  分割統治 = 8  :  4.544?s  :  3.936?s
要素数  :  総当たり戦略  :  分割統治 = 9  :  5.102?s  :  4.886?s
要素数  :  総当たり戦略  :  分割統治 = 10  :  6.477?s  :  5.598?s
要素数  :  総当たり戦略  :  分割統治 = 11  :  8.212?s  :  6.14?s
要素数  :  総当たり戦略  :  分割統治 = 12  :  9.69?s  :  7.486?s
要素数  :  総当たり戦略  :  分割統治 = 13  :  11.617?s  :  8.533?s
要素数  :  総当たり戦略  :  分割統治 = 14  :  12.691?s  :  8.582?s
要素数  :  総当たり戦略  :  分割統治 = 15  :  12.828?s  :  8.71?s
要素数  :  総当たり戦略  :  分割統治 = 16  :  16.715?s  :  10.895?s
要素数  :  総当たり戦略  :  分割統治 = 17  :  17.261?s  :  10.473?s
要素数  :  総当たり戦略  :  分割統治 = 18  :  20.797?s  :  12.249?s
要素数  :  総当たり戦略  :  分割統治 = 19  :  22.781?s  :  13.097?s
要素数  :  総当たり戦略  :  分割統治 = 20  :  24.218?s  :  14.101?s
要素数  :  総当たり戦略  :  分割統治 = 21  :  26.412?s  :  14.086?s
要素数  :  総当たり戦略  :  分割統治 = 22  :  29.178?s  :  15.375?s
要素数  :  総当たり戦略  :  分割統治 = 23  :  32.537?s  :  16.605?s
要素数  :  総当たり戦略  :  分割統治 = 24  :  33.34?s  :  17.281?s
要素数  :  総当たり戦略  :  分割統治 = 25  :  35.704?s  :  17.682?s
要素数  :  総当たり戦略  :  分割統治 = 26  :  40.877?s  :  19.403?s
要素数  :  総当たり戦略  :  分割統治 = 27  :  40.812?s  :  20.598?s
要素数  :  総当たり戦略  :  分割統治 = 28  :  44.214?s  :  20.364?s
要素数  :  総当たり戦略  :  分割統治 = 29  :  42.266?s  :  21.559?s
要素数  :  総当たり戦略  :  分割統治 = 30  :  51.377?s  :  21.642?s
要素数  :  総当たり戦略  :  分割統治 = 31  :  53.039?s  :  22.858?s
```

## A4.1-4

今までのアルゴリズムが空の部分配列を解として認めない、と問題文には書いてありましたが、意味がよく分かりませんでした。
部分配列が空でも問題なく結果を出しているように見えるからです。

分からないので解答保留です。

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

# 4.2　Strassenのアルゴリズム

## 行列の要素の総当たり積算

行列は掛ける順番で結果が変わるのでご注意ください。
また、最初の行列の列数と次の行列の行数が一致していないと計算は成立しません。

```rust
type Matrix = Vec<Vec<i32>>;

fn matrix_print(a: &Matrix) {
    println!("{}X{}行列",a.len(), a[0].len());
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

ここでRustによる行列計算は以下を参照しました。

[行列の足し算で学ぶRustのエラーハンドリング](https://qiita.com/es__135/items/5a4121de7f5eaa3f4973)

ndarrayを使用するのが一般的だと思いますが、ndarrayを使うと行列の積が数式1行で実行できます。
便利で良いのですが、それではアルゴリズムの練習にならないので今回はvecの多重入れ子を用いました。

## 部分行列への分割

行列を部分行列に分割する時は、添え字の計算で範囲を定めることが推奨されていましたが、以下では行列の要素をコピーしています。
これにより$\Theta(n^2)$の定数倍、計算時間が増えます。


```rust
type Matrix = Vec<Vec<i32>>;

fn matrix_print(a: &Matrix) {
    println!("{}X{}行列",a.len(), a[0].len());
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

次にこのアルゴリズムの計算量$T(n)$を求めます。

$n=1$の時は積算を1回だけ実行するので$T(1) = \Theta(1)$です。

```rust
c[0][0] = a[0][0]*b[0][0];
```

$n>1$の時は$n/2$回の計算を行うforループを2回繰り返す箇所の計算量は全て$\Theta(n^2/4)$になります。
$\Theta$の引数を定数倍しても計算の次元は変わらないので、$\Theta(n^2) = \Theta(n^2/4)$であり、また$\Theta(n^2) = \Theta(cn^2)$でもあるので、まとめて$\Theta(n^2)$に集約できます。

再帰呼び出しは$T(n/2)$が8回実行されます。
再帰呼び出しを定数倍すれば計算の次元に影響を与えるので、定数も計算量に含めないといけません。

よって計算量は

```math
\begin{eqnarray}
T(n) &=& \left \{
\begin{array}{l}
\Theta(1)  \ \ \ \ \ \ \ \ \ \ \ \ \ \ \ \ \ \ \ \ \ \ \ \ \ \ (n=1の場合)\\
8T(n/2) + \Theta(n^2) \ \ \ \ \ (n>1の場合)
\end{array}
\right.
\end{eqnarray}
```

となります。

## Strassenの方法

これは再帰処理の回数を8回から7回に減らすことで、計算量を減らすアルゴリズムです。

まずは先ほど同様に2つの行列を8個の部分行列に分割します。
次に、この8個の部分行列から更に10個の行列を生成します。
この10個の行列を用いると再帰呼び出しは7回で済みます。

```rust

type Matrix = Vec<Vec<i32>>;

fn matrix_print(a: &Matrix) {
    println!("{}X{}行列",a.len(), a[0].len());
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

よって

```math
\begin{eqnarray}
\begin{pmatrix} 1 & 3 \\ 7 & 5 \end{pmatrix}
\begin{pmatrix} 6 & 8 \\ 4 & 2 \end{pmatrix}
=
\begin{pmatrix} 18 & 14 \\ 62 & 66 \end{pmatrix}
\end{eqnarray}
```

が求まります。

## A4.2-2

[Strassenの方法](## Strassenの方法)をご参照ください。

## A4.2-3

与えられた正行列の行数が偶数か奇数かを判別します。
偶数ならば、今までと同じように解けます。
奇数ならば４つの部分行列、$2n-1 \times 2n-1$行列、$1 \times 2n-1$行列、$2n-1 \times 1$行列、$1 \times 1$行列に分けます。
そして$2n-1 \times 2n-1$行列について今までと同じように求め、残りの行列を総当たり戦略で解きます。

```rust
type Matrix = Vec<Vec<i32>>;

fn matrix_print(a: &Matrix) {
    println!("{}X{}行列",a.len(), a[0].len());
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

よって$a(c+d)$と$d(a+b)$と$c(b-a)$の3回の乗算により計算を実行できます。


# 4.3 置き換え法

## A4.3-1

```math
\begin{eqnarray}
T(n) &\leq& c(n-1)^2 + n = cn^2 + (1-2c)n + 1 \\
&\leq& cn^2 + 1 - 2c + 1 \\
&\leq& cn^2 
\end{eqnarray}
```

$ c \geq 1$ になるように選ぶと、$1-2c < 0$で$n \geq 1$なので $(1-2c)n \leq 1-2c$ になります。
更に$1-c \leq 0$なので最後の式展開が成立します。

## A4.3-2
これは $T(n) \leq cn$ が成立しない場合なので、$T(n) \leq cn -d$を示します。

$\lceil a/b \rceil \leq (a+b-1)/b$より

```math
\begin{eqnarray}
T(n) &\leq& c\lg (\lceil n/2 \rceil) -d + 1 \\
&\leq& c \lg (n+1)/2 -d + 1 \\
&\leq& c \lg (1+k)n/2 -d + 1 
= c\lg (1+k) + c \lg n - c -d + 1
\end{eqnarray}
```

ここで$d=1$とします。
また、$c \lg (1+k) - c \leq c \lg(1+k)$より

```math
\begin{eqnarray}
T(n) &\leq& c\lg (1+k) + c \lg n
\end{eqnarray}
```

漸化式を満たすには、$c \lg (1+k) = -1$ になればいいので、$c=t,1+k=1/2^{1/t}$ とおけば

```math
\begin{eqnarray}
t \lg 1/2^{1/t} = - t \frac{1}{t} = -1  
\end{eqnarray}
```

となります。よって$c \geq 1$を満たす任意の正数が選べます。$c=1$でも$c=3$でも$c=645363728$でもです。
よって$T(n) \leq 645363728 \lg n -1$が成立します。


## A4.3-3

$\lfloor a/b \rfloor \geq (a-b+1)/b$より


```math
\begin{eqnarray}
T(n) &\geq& 2(c\lfloor n/2 \rfloor \lg (\lfloor n/2 \rfloor) + n \\
&\geq& 2(c \frac{n-1}{2} \lg\frac{n-1}{2}) + n \\
&=&c(n-1) \lg(n-1) - c(n-1) + n \\
&=& cn \lg n - c \lg n + c(n-1) \lg ((n-1)/n) -c(n-1) + n
\end{eqnarray}
```

ここで


```math
\begin{eqnarray}
0 \leq - c \lg n + c(n-1) \lg ((n-1)/n) -c(n-1) + n
\end{eqnarray}
```

を示せれば、$T(n) \geq cn \lg n$が成立して$T(n) = \Omega(n \lg n)$が証明できます。

$0 < c \leq 1/3$として$n \geq 2$において数値計算すると

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

より0以上です。
与式は$n$の増加に対して単調増加するので、与式は常に0以上になり、$T(n) = \Omega(n \lg n)$が成立します。

## A4.3-4

まず$T(n) \leq cn\lg n -d$と仮定します。
ここで$c \geq 1$とします。

また、$T(1)=1$を満たさないといけないので$d = -1$として$T(n) \leq cn\lg n +1$でないといけません。

$\lfloor a/b \rfloor \leq a/b$より

```math
\begin{eqnarray}
T(n) &\leq& 2(c \lfloor n/2 \rfloor \lg (\lfloor n/2 \rfloor)) +2 + n \\
&\leq& c n \lg (n/2) -2d + n = cn \lg n -cn +2 + n 
\end{eqnarray}
```

ここで$ -cn +2 + n  \leq 1$を示せれば、$T(n) \leq cn\lg n +1$が証明できます。
もし$c=1$ならば$2 \geq 1$となるので不成立です。
$c \geq 2$ならば$(1-c)n \leq -1$となるので$ -cn +2 + n  \leq 1$が成立します。

よって

```math
\begin{eqnarray}
T(n) &\leq& cn\lg n +1
\end{eqnarray}
```

となります。

## A4.3-5

マージソートは$T(n)$の引数が整数でないといけないので、$n$が偶数の場合と奇数の場合で異なります。

偶数の場合は$\lceil n/2 \rceil$も$\lfloor n/2 \rfloor$も整数なので

```math
\begin{eqnarray}
T(n) &=& T(\lceil n/2 \rceil)+T(\lfloor n/2 \rfloor)+\Theta(n) \\
&=& 2T(n/2)+\Theta(n) 
\end{eqnarray}
```
です。


$\Theta(n)$は$O(n)$と$\Omega(n)$も満たすので、

```math
\begin{eqnarray}
T(n) &\leq& 2c(n/2)\lg (n/2) + O(n) \\
&=& cn \lg n - cn + O(n) \\
&=& cn \lg n + O(n)
\end{eqnarray}
```

より$T(n) = O(n \lg n + O(n)) = O(n \lg n)$

```math
\begin{eqnarray}
T(n) &\geq& 2c(n/2)\lg (n/2) + \Omega(n) \\
&=& cn \lg n - cn + \Omega(n) \\
&=& cn \lg n + \Omega(n)
\end{eqnarray}
```

より$T(n) = \Omega(n \lg n + \Omega(n)) = \Omega(n \lg n)$

よって$T(n) = \Theta(n \lg n)$が成立します。


奇数の場合は$\lceil n/2 \rceil$も$\lfloor n/2 \rfloor$は整数ではないので、$n-1 < \lfloor n \rfloor \leq n \leq \lceil n \rceil < n+1$、$\lfloor n/2 \rfloor \geq (n-2+1)/2$、 $\lceil n/2 \rceil \leq (n+2-1)/2$ より

```math
\begin{eqnarray}
T(n) &=& T(\lceil n/2 \rceil)+T(\lfloor n/2 \rfloor)+\Theta(n) \\
&=& T((n+1)/2)+T((n-1)/2)+\Theta(n) \\
&=& c\frac{n+1}{2} \lg \frac{n+1}{2} + c \frac{n-1}{2} \lg \frac{n-1}{2} +\Theta(n) \\
&=& c\frac{n+1}{2} \lg (n+1) + c \frac{n-1}{2} \lg (n-1) +\Theta(n) \\
&=& c\frac{n}{2} \lg (n^2-1) + c \frac{1}{2} \lg \frac{n+1}{n-1} +\Theta(n) \\
\end{eqnarray}
```

です。

$cn \lg n$が上式より大きければいいのですが、それをどうやって証明すればいいのか検討中です。

解答保留とします。



## A4.3-6

まず$T(n) \leq cn\lg n -d$と仮定します。
ここで$c \geq 1$とします。

```math
\begin{eqnarray}
T(n) &\geq& 2c(n/2 + 17)\lg (n/2 + 17) - d) + n \\  
&=& 2cn\lg (n/2 + 17) + 34 c \lg (n/2 + 17) - 2d + n \\
\end{eqnarray}
```

以降はA4.3-2と同じだと思われます。
（この解答にはあまり自信がないので、再検討するかもしれません）


## A4.3-7

$T(n) \leq cn^{\log_3 4}$を満たすと仮定します。

```math
\begin{eqnarray}
T(n) &=& 4T(n/3) + n \\
&\leq& 4c(n/3)^{\log_3 4} + n \\
&=& cn^{\log_3 4} + n \\
\end{eqnarray}
```

ここで$3^{\log_3 4} = 4^{\log_3 3} = 4$を用いました。
$n \geq 1$なので、これが$T(n) \leq cn^{\log_3 4}$という形式になることはありません。


次に$T(n) \leq cn^{\log_3 4} - dn$を満たすと仮定します。

```math
\begin{eqnarray}
T(n) &=& 4T(n/3) + n \\
&\leq& 4c(n/3)^{\log_3 4} -4dn/3+ n \\
&=& cn^{\log_3 4} -4dn/3 + n \\
\end{eqnarray}
```

ここで$d=3$ならば

```math
\begin{eqnarray}
T(n) 
&\leq& cn^{\log_3 4} - 3n
\end{eqnarray}
```

となり仮定を満たします。


## A4.3-8

$T(n) \leq cn^2$を満たすと仮定します。

```math
\begin{eqnarray}
T(n) 
&\leq& c4n^2/4 + n^2 = (c+1)n^2
\end{eqnarray}
```

$c>0$なので、これが$T(n) \leq cn^2$を満たすことはありません。


次に$T(n) \leq cn^2 -dn$を満たすと仮定します。

```math
\begin{eqnarray}
T(n) 
&\leq& c4n^2/4 -dn + n^2 = (c+1)n^2 - dn
\end{eqnarray}
```

これも$T(n) \leq cn^2$を満たすことはありません。
以上のように、問題文は「低次の項を引いた置き換え法に基づいて証明できる」と書いてあるのですが、実際には証明できません。

ここで私は問題文の$T(n)=4T(n/2)+n^2$が$T(n)=4T(n/2)+n^2$が$T(n)=4T(n/2)+n$の誤植ではないかと疑いました。
前者は$n^{2 \log_2 4} = n^4$よりマスター法を適用できる条件を満たさず、$\Theta(n^2)$にもなりません。
後者ならば、マスター法を適用すると$n^{\log_2 4} = n^2 = \Theta(n^2)$となり問題文と一致します。

そこで後者が正しいとしてやり直してみます。


$T(n) \leq cn^2$を満たすと仮定します。

```math
\begin{eqnarray}
T(n) 
&\leq& c4n^2/4 + n = cn^2 + n
\end{eqnarray}
```

$c>0$なので、これが$T(n) \leq cn^2$を満たすことはありません。
次に$T(n) \leq cn^2 -dn$を満たすと仮定します。

```math
\begin{eqnarray}
T(n) 
&\leq& c4n^2/4 -4dn/2 + n = cn^2 + (1-2d)n
\end{eqnarray}
```

$d=1$とすれば、$T(n) \leq cn^2 -n$を満たします。


## A4.3-9

$m = \lg n$として$T(2^m)=3T(2^{m/2}) + m$と式変形します。

ここで$S(m) = T(2^m)$とすると

```math
\begin{eqnarray}
S(m) 
&=& 3S(m/2) + m
\end{eqnarray}
```

$S(m) \leq cm^{log_2 3} - dm$を満たすとします。

```math
\begin{eqnarray}
S(m) 
&\leq& 3c(m/2)^{log_2 3}  -3dm/2 + m \\
&=& 3cm^{log_2 3}  -3dm/2 + m
\end{eqnarray}
```

上式の展開では、$2^{log_2 3} = 3^{log_2 2}= 3$を用いました。
$d=2$とすれば、$S(m) \leq cm^{log_2 3} - 2m$ を満たします。

$S(1) = c -2$より$c = S(1)+2$です

よって解は 

```math
\begin{eqnarray}
T(n) = T(2^m) = S(m) = (S(1)+2)m^{lg 3} - 2m = (T(2)+2) \lg n^{lg 3} - 2\lg n
\end{eqnarray}
```

 となります。

# 4.4 漸化式を解くための再帰木法

## A4.4-1

再帰木の各レベルのサイズは$n/2^i$です。
再帰木のレベルの階層はサイズが1になるまで深くなるので、$n/2^i = 1$の場合、つまり$i = \log_2 n$層になります。
よって$i=0,1, \cdots , \log_2 n-1$です。
また各レベルの節点の数は$3^i$です。
レベル$\log_2 n$での節点数は$3^{\log_2 n} = n^{\log_2 3}$です。
レベル$\log_2 n$でのコストは$n^{\log_2 3}T(1) = \Theta(n^{\log_2 3})$になります。

木全体のコストは

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

になります。
よって漸近的上限は$O(n^{\log_2 3})$です。

次にこれを置き換え法を用いて検証します。

$T(n) \leq cn^{\log_2 3} -dn$を満たすと仮定します。

```math
\begin{eqnarray}
T(n) &=& 3T(\lfloor n/2 \rfloor) + n \\
&\leq& 3 \frac{cn^{\log_2 3}}{2^{\log_2 3}} - \frac{3dn}{2} + n \\
&=& cn^{\log_2 3} - \frac{3dn}{2} + n 
\end{eqnarray}
```

$d=2$とすれば、$T(n) \leq cn^{\log_2 3} - 2n$が成立します。

## A4.4-2

再帰の各レベルのサイズは$n/2^i$です。
再帰木のレベルの階層はサイズが1になるまで深くなるので、$n/2^i = 1$の場合、つまり$i = \log_2 n$層になります。
よって$i=0,1, \cdots , \log_2 n-1$です。
また各レベルの節点の数は$1$です。
レベル$\log_2 n$での節点数は$1$です。
レベル$\log_2 n$でのコストは$T(1) = \Theta(1)$になります。

木全体のコストは

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

になります。
よって漸近的上限は$O(n^2)$です。

次にこれを置き換え法を用いて検証します。

$T(n) \leq cn^2$を満たすと仮定します。

```math
\begin{eqnarray}
T(n) &=& T(n/2) + n^2 \\
&\leq& cn^2/2^2 + n^2 \\
&=& (c/4 + 1) n^2 \leq cn^2
\end{eqnarray}
```

が成立します。
ここで$c/4 + 1 \leq c$となるように$4/3 \leq c$とします。

## A4.4-3

再帰木の各レベルのサイズは$n/2^i+2$です。
再帰木のレベルの階層はサイズが3になるまで深くなるので、$n/2^i+2 = 3$の場合、つまり$i = \log_2 n$層になります。
よって$i=0,1, \cdots , \log_2 n-1$です。
また各レベルの節点の数は$4^i$です。
レベル$\log_2 n$での節点数は$4^{\log_2 n} = n^{\log_2 4}=n^2$です。
レベル$\log_2 n$でのコストは$n^2 T(3) = \Theta(n^2)$になります。

木全体のコストは

```math
\begin{eqnarray}
T(n) 
&=& n + \Bigl(\frac{4}{2} n + 8 \Bigl) + \Bigl(\frac{4^2}{2^2} n + 4^2*2 \Bigl) \cdots + \Bigl(\frac{4^{\log_2 n-1}}{2^{\log_2 n-1}} n + 4^{\log_2 n-1}*2 \Bigl) + n^2 T(3) \\  
&=& n + (2 n + 8 ) + (4 n + 32 ) \cdots + (2^{\log_2 n-1} n + 4^{\log_2 n-1}*2 ) + n^2 T(3) \\
&=& n + (2 n + 8 ) + (4 n + 32 ) \cdots + ((n-1)n + 2(n-1)^2) + n^2 T(3) \\
&=& O(n^2)
\end{eqnarray}
```

になります。
よって漸近的上限は$O(n^2)$です。

次にこれを置き換え法を用いて検証します。

$T(n) \leq cn^2 -dn$を満たすと仮定します。ここで$c \geq 0$とします。

```math
\begin{eqnarray}
T(n) &=& 4T(n/2+2) + n \\
&\leq& 4c(n/2+2)^2 -4d(n/2+2) + n \\
&=& c(n^2 + 8n + 16 - 2dn/c -8d/c)+n
\end{eqnarray}
```

$c(8-2d/c)+1 \leq -d$と$c(16-8d/c) \leq 0$を満たせば仮定が成立します。

$c(16-8d/c) \leq 0$より、$d \geq 2c$となります。

$d/c=2$の場合、$4c+1 \leq -2$ですが、$c \geq 0$なので成立しません。
$d/c=3$の場合、$2c+1 \leq -3$ですが、$c \geq 0$なので成立しません。
$d/c=4$の場合、$1 \leq -4$ですが、成立しません。
$d/c=5$の場合、$-2c+1 \leq -5$なので、$c \geq 3$の場合に成立します。
そして$d \geq 6$となります。$d/c=5$も満たさないといけないので、ここでは$d=5c$とします。


よって、$c \geq 3$を満たせば、$T(n) \leq cn^2 -5cn$が成立します。



## A4.4-4

再帰木の各レベルのサイズは$1^{n-i}$です。
再帰木のレベルの階層はサイズが$1^1$になるまで深くなるので、$n$層になります。
よって$i=0,1, \cdots , n-1$です。
また各レベルの節点の数は$2^i$です。
レベル$n$での節点数は$2^n$です。
レベル$n$でのコストは$2^n T(1) = \Theta(2^n)$になります。

木全体のコストは

```math
\begin{eqnarray}
T(n) 
&=& 1 + 2 +  \cdots + 2^{n-1} + 2^n T(1) 
&=& O(2^n)
\end{eqnarray}
```

になります。
よって漸近的上限は$O(2^n)$です。

次にこれを置き換え法を用いて検証します。

$T(n) \leq c2^n -d$を満たすと仮定します。ここで$c \geq 0$とします。

```math
\begin{eqnarray}
T(n) &=& 2T(n-1) + 1 \\
&\leq& 2c2^{n-1} -2d + 1 \\
&=& c2^n -2d+1
\end{eqnarray}
```

$d=1$とすれば、$T(n) \leq c2^n -1$が成立します。

## A4.4-5

$T(n/2)$について、再帰の各レベルのサイズは$n/2^i$です。
再帰木のレベルの階層はサイズが1になるまで深くなるので、$n/2^i = 1$の場合、つまり$i = \log_2 n$層になります。
よって$i=0,1, \cdots , \log_2 n-1$です。
また各レベルの節点の数は$2^i$です。（$T(n-1)$による節点も大雑把な推定として含めてしまいます。）
レベル$\log_2 n$での節点数は$2^{\log_2 n} = n$です。
レベル$\log_2 n$でのコストは$nT(1) = \Theta(n)$になります。

木全体のコストは

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

$T(n-1)$について、再帰木の各レベルのサイズは$1^{n-i}$です。
再帰木のレベルの階層はサイズが$1^1$になるまで深くなるので、$n$層になります。
よって$i=0,1, \cdots , n-1$です。
また各レベルの節点の数は$2$です。（$T(n/2)$による節点も大雑把な推定として含めてしまいます。）
レベル$n$での節点数は$2^n$です。
レベル$n$でのコストは$2^n T(1) = \Theta(2^n)$になります。

木全体のコストは

```math
\begin{eqnarray}
T(n) 
&=& n + n +  \cdots + n + 2^n T(1) 
&=& O(2^n)
\end{eqnarray}
```

になります。

全体的にはこちらの計算が寄与して、漸近的上限は$O(2^n)$になると予測します。

次にこれを置き換え法を用いて検証します。

$T(n) \leq 2^n$を満たすと仮定します。ここで$c \geq 0$とします。

```math
\begin{eqnarray}
T(n) 
&\leq& 2^{n-1} + \sqrt{2^n} + n
\end{eqnarray}
```

$sqrt{2^n} + n \leq 2^{n-1}$なので

```math
\begin{eqnarray}
T(n) 
&\leq& 2^n
\end{eqnarray}
```

が成立します。

## A4.4-6

本書においては、漸近的上界を経路の長い方の再帰木$T(2n/3)$からの推定で求めていました。

漸近的下界は経路が短い方の再帰木り$T(n/3)$からの推定で求まります。

再帰の各レベルのサイズは$n/3^i$です。
再帰木のレベルの階層はサイズが1になるまで深くなるので、$n/3^i = 1$の場合、つまり$i = \log_3 n$層になります。
よって$i=0,1, \cdots , \log_3 n-1$です。
また各レベルの節点の数は$2^i$です。
レベル$\log_3 n$での節点数は$2^{\log_3 n}= n^{\log_3 2}$です。
レベル$\log_3 n$でのコストは$n^{\log_3 2} T(1) = \Theta(n^{\log_3 2})$になります。

木全体のコストは

```math
\begin{eqnarray}
T(n) 
&=& cn + \frac{2}{3} cn + \frac{2^2}{3^2} cn + \cdots + \frac{2^{\log_3 n-1}}{3^{\log_3 n-1}} cn + n^{\log_3 2} T(1) \\
&=& \sum_{i=0}^{\log_3 n-1} \Bigl( \frac{2}{3} \Bigl)^i cn + n^{\log_3 2} T(1)\\
&=& \frac{(2/3)^{\log_3 n}-1}{(2/3)-1}cn  + n^{\log_3 2} T(1) \\
&=& \frac{1}{3} (1 - 2^{\log_3 n}/n)cn + n^{\log_3 2} T(1) \\
\end{eqnarray}
```

になります。
これが$T(n) \geq dn \lg n$を満たすことを示そうとしましたが、どう計算すればいいのか分かりませんでした。
よって、以上の計算は全てなしとしますが、正解に至らなかった考察の備忘録として残します。
そして最初からやり直します。

本書の議論とは逆に$d \leq c/(\lg 3 - 2/3)$を満たすように選べば$T(n) \geq dn \lg n$になるので$\Omega(n \lg n)$が成立します。


## A4.4-7
$T(n) \leq dn^2$と仮定します。

```math
\begin{eqnarray}
T(n) 
&\leq& 4T(\lfloor n/2 \rfloor) + cn \\
&=& dn^2 + cn
\end{eqnarray}
```

$dn^2 + cn \leq n^2$を満たせば$n^2$をタイトな上限とみなせます。よって$d+c/n \leq 1$を満たせるかを考えます。
十分に大きな$n$では2項目は0に近づきます。$d \leq 1$を満たすならば与式は成立します。

逆に$T(n) \geq dn^2$と仮定します。

```math
\begin{eqnarray}
T(n) 
&\geq& 4T(\lfloor n/2 \rfloor) + cn \\
&=& dn^2 + cn
\end{eqnarray}
```

$dn^2 + cn \geq n^2$を満たせば$n^2$をタイトな下限とみなせます。よって$d+c/n \geq 1$を満たせるかを考えます。
十分に大きな$n$では2項目は0に近づきます。$d \geq 1$を満たすならば与式は成立します。

よってタイトな限界は$n^2$となります。


## A4.4-8

再帰木の高さは$n$が$a$で割り切れるならば$n/a$です。
$n$が$a$で割り切れずに余りが出るならば$(n/a)-(余り)$です。
よって再帰木の高さは$\lceil n/a \rceil$となります。

木全体のコストは

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

となります。

## A4.4-9

この再帰式は、サイズ$n$を$\alpha n$と$(1-\alpha)n$に分割します。
次の階層では、$\alpha n$を$\alpha^2 n$と$(1-\alpha)\alpha n$に分割し、$(1-\alpha) n$を$(1-\alpha)\alpha n$と$(1-\alpha)^2 n$に分割します。
このように2分割を続ける再帰式は$\alpha = 1/2$ならば2章でのマージソートそのものです。

ここでマージソートとの類推から、計算のサイズを$n \lg n$と仮定します。

$T(n) \leq k n \lg n -tn$を満たすと仮定します。ここで$k \geq 0$とします。

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

ここで$t=c$になるように選びます。
また$0 < \alpha < 1$より$\lg n$と$\lg (1-\alpha)$は必ず分数の対数$\log(a/b)=\log a - \log b$になり、かつ分母が分子よりも大きいので$\log a - \log b \leq 0$になります。
そして$a$を適当にえらぶことで$k \alpha n \lg \alpha + k (1-\alpha)n \lg (1-\alpha) \leq -c$にできます。

よって$T(n) \leq  k n \lg n -cn$が成立します。

# 4.5 漸化式を解くためのマスター法

## マスター定理

$a \geq 1$、$b > 1$とします。

```math
\begin{eqnarray}
T(n) 
&=& aT(n/b)+f(n)
\end{eqnarray}
```

について、$f(n)$がどのような漸近的限界を持つかにより、$T(n)$には以下の３つの漸近的限界が成立します。

### 漸近的上界

$\epsilon > 0$に対して漸近的上界$f(n) = O(n^{\log_b a - \epsilon})$を満たすならば、$T(n) = \Theta(n^{\log_b a})$になります。

一般的には$f(n) = O(f(n)の最大次数)$とみなしていいと思います。

ですが、もし$f(n)/n^{\log_b a}$が$1/n^{\epsilon}$より漸近的に大きい場合は$f(n)$が$O(n^{\log_b a - \epsilon})$の上界を超えることになり、$f(n) = O(n^{\log_b a - \epsilon})$を満たさないので注意しないといけません。

これは$n^{\log_b a}/f(n)$が$n^{\epsilon}$より漸近的に小さい場合と言い換えることもできます。

### 漸近的限界

漸近的限界$f(n) = \Theta(n^{\log_b a})$を満たすならば、$T(n) = \Theta(n^{\log_b a} \lg n)$になります。


### 漸近的下界

$\epsilon > 0$に対して漸近的下界$f(n) = \Omega(n^{\log_b a + \epsilon})$を満たし、かつ$c<1$と十分大きな$n$に対して$af(n/b) \leq cf(n)$ならば、$T(n) = \Theta(f(n))$になります。

もし$f(n)/n^{\log_b a}$が$n^{\epsilon}$より漸近的に小さい場合は$f(n)$が$\Omega(n^{\log_b a + \epsilon})$の下界を下回ることになり、$f(n) = \Omega(n^{\log_b a + \epsilon})$を満たさないので注意しないといけません。

## A4.5-1

### a

$T(n) = 2T(n/4)+1$

$a=2$、$b=4$、$f(n)=1$

$f(n) = 1 = \Theta(1)$

$n^{\log_b a + \epsilon} = n^{\log_4 2 + \epsilon}$について$\epsilon = -\log_4 2$に選べば$n^0=1=O(1)$になるので、$T(n)=\Theta(n^{\log_4 2})=\Theta(n^{1/2})$が成立します。

### b

$T(n) = 2T(n/4)+\sqrt{n}$

$a=2$、$b=4$、$f(n)=n^{1/2}$

$f(n) = \sqrt{n} = \Theta(n^{1/2})$

$n^{\log_b a + \epsilon} = n^{\log_4 2 + \epsilon}$について$\epsilon = 0$に選べば$n^{1/2}=\Theta(n^{1/2})$になるので、$T(n)=\Theta(n^{1/2} \lg n)$が成立します。

### c

$T(n) = 2T(n/4)+n$

$a=2$、$b=4$、$f(n)=n$

$f(n) = n = \Theta(n)$

$n^{\log_b a + \epsilon} = n^{\log_4 2 + \epsilon}$について$\epsilon = +\log_4 2$に選べば$n=\Omega(n)$になります。

$an/b = n/2 \leq cn$が$1/2 \leq c < 1$について成立するので、$T(n)=\Theta(n)$が成立します。

### d

$T(n) = 2T(n/4)+n^2$

$a=2$、$b=4$、$f(n)=n^2$

$f(n) = n^2 = \Theta(n^2)$

$n^{\log_b a + \epsilon} = n^{\log_4 2 + \epsilon}$について$\epsilon = 1+\log_4 2$に選べば$n^2=\Omega(n^2)$になります。

$an/b = n^2/2 \leq cn^2$が$1/2 \leq c < 1$について成立するので、$T(n)=\Theta(n^2)$が成立します。


## A4.5-2

本文よりStrassenのアルゴリズムの漸近的解は$\Theta(n^{\lg 7})$です。（$2.80 < \lg 7 < 2.81$）

$T(n) = aT(n/4)+\Theta(n^2)$より、$n^{\log_b a + \epsilon} = n^{\log_4 a + \epsilon}$について$\epsilon = 0$ならば$T(n)=\Theta(\log_4 a \lg n)$になります。
ですがこれは$\Theta(n^{\lg 7})$より効率的にはならないので成立しません。

$T(n) = aT(n/4)+\Theta(n^2)$より、$n^{\log_b a + \epsilon} = n^{\log_4 a + \epsilon}$について$\epsilon < 0$ならば$T(n)=\Theta(\log_4 a)$になります。
よって$\log_4 a < \lg 7$を満たす最大の整数$a$は48です。
なお、$n^{\log_b a + \epsilon} = n^{\log_4 48 + \epsilon}$について$\log_4 48 + \epsilon = 2$になるように$\epsilon$を選ぶことができます。


$T(n) = aT(n/4)+\Theta(n^2)$より、$n^{\log_b a + \epsilon} = n^{\log_4 a + \epsilon}$について$\epsilon > 0$かつ$\Theta(a n^2/4) \leq \Theta(c n^2)$ならば$T(n)=\Theta(n^2)$になります。
上記が成立するためには、$\log_4 a + \epsilon = 2$になるように選ばないといけません。ですが、$a$の最大値は$a=4^2$を超えることはないので、$a=48$が最大の整数になります。


## A4.5-3

$T(n) = T(n/2)+\Theta(1)$

$a=1$、$b=2$、$f(n)=\Theta(1)$

$n^{\log_b a + \epsilon} = n^{\log_2 1 + \epsilon}$について$\epsilon = 0$に選べば$n^0=1=\Theta(1)$になるので、$T(n)=\Theta(\lg n)$が成立します。

## A4.5-4

$T(n) = 4T(n/2)+ n^2 \lg n$

$a=4$、$b=2$、$f(n) = n^2 \lg n$


$n^{\log_b a + \epsilon} = n^{\log_2 4 + \epsilon} = n^{2 + \epsilon}$について考えます。
$\epsilon = 0$に選べば$n^2$について$\Theta(n^2 \lg n)$が成立しません。
$\epsilon < 0$に選べば$\lg n$が$1/n^{\epsilon}$より漸近的に大きいので$O(n^2 \lg n)$が成立しません。
$\epsilon > 0$に選べば$\lg n$が$n^{\epsilon}$より漸近的に小さいので$\Omega(n^2 \lg n)$が成立しません。

よってマスター定理は適用できません。
そして$T(n)$の漸近解は$\Theta(n^2)$、$\Theta(n^2 \lg n)$ではありません。


$T(n) \leq n^2 \lg^2 n$と仮定します。

```math
\begin{eqnarray}
T(n) 
&=& 4T(n/2)+n^2 \lg n \\
&\leq& 4 \frac{n^2}{4} (\lg n - \lg 2)^2 +n^2 \lg n　\\
&=& n^2 \lg^2 n +n^2 -2 n^2 \lg n +n^2 \lg n\\
&=& n^2 \lg^2 n -n^2(\lg n -1) \\
&\leq& n^2 \lg^2 n
\end{eqnarray}
```

が成立します。よって$T(n)=O(n^2 \lg^2 n)$になります。


## A4.5-5

正則性$af(n/b) \leq cf(n)$を満たさない$f(n)$を求める問題です。
$f(n) < f(n/b)$を満たすような関数を探します。
ただし$f(n)$は$\Omega(n^{\log_b a + \epsilon})$も満たさないといけません。

思いつかないので解答保留です。


# 4.6 マスター定理の証明

## 正確なベキ乗に対する証明

### 補題4.2

漸化式を再帰木法で解きます。

再帰木の各レベルのサイズは$n/b^i$です。
再帰木のレベルの階層はサイズが1になるまで深くなるので、$n/b^i = 1$の場合、つまり$i = \log_b n$層になります。
よって$i=0,1, \cdots , \log_b n-1$です。
また各レベルの節点の数は$a^i$です。
レベル$\log_b n$での節点数は$a^{\log_b n} = n^{\log_b a}$です。
レベル$\log_b n$でのコストは$n^{\log_b a} T(1) = \Theta(n^{\log_b a})$になります。

木全体のコストは

```math
\begin{eqnarray}
T(n) 
&=& f(n) + a f(n/b) + a^2 f(n/b^2) + \cdots + a^{\log_2 n-1} f(n/b^{\log_2 n-1}) + \Theta(n^{\log_b a}) \\
&=& \sum_{j=0}^{\log_b n-1} a^j f(n/b^j) + \Theta(n^{\log_b a})
\end{eqnarray}
```

になります。

マスター定の３つのパターンは、葉のコスト$\Theta(n^{\log_b a})$が主要コストになる場合、根のコスト$f(n)$が葉のコスト$\Theta(n^{\log_b a})$と等しい場合、根のコスト$f(n)$が主要コストになる場合を表していると解釈できます。



# 他の記事
- [個人的勉強メモ「アルゴリズム・イントロダクション」をRustで実装する：2章](https://qiita.com/Kosuke_Matsui/items/eea26e88cd261173a292)
- [個人的勉強メモ「アルゴリズム・イントロダクション」をRustで実装する：3章](https://qiita.com/Kosuke_Matsui/private/b424d8eb30dd16455b12)
- [個人的勉強メモ「アルゴリズム・イントロダクション」をRustで実装する：4章](https://qiita.com/Kosuke_Matsui/private/3b93d6b8a7ee5e90a2b5)

