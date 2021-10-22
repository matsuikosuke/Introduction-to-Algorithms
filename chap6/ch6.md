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
ある要素が根ならば、その子は全て根より小さな値になるので、最大要素は根になります。

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

$T(n) = T(2n/3)+\Theta(1)$について

$a=1$、$b=2/3$、$f(n)=\Theta(1)$なので

マスター定理より、$n^{\log_b a + \epsilon} = n^{\log_{2/3} 1 + \epsilon}$について$\epsilon = 0$に選べば$n^0=1=\Theta(1)$になるので、max_heapの計算量は$T(n)=\Theta(\lg n)$になります。


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
任意の配列をMaxヒープに並べ替えます。
今までのアルゴリズムに`build_max_heap`を追加しています。
このアルゴリズムは、子を持つ要素の全てについて子と親の関係を正しく並んでいるかを確認して並べ替えます。
具体的には、対象の要素の内の最後の要素から最初の要素の順番に`max_heap`を適用します。

ヒープの葉（子を持たない要素）は$\lfloor n/2 \rfloor +1$番目の要素、$\lfloor n/2 \rfloor+2$番目の要素 $\cdots$ $n$番目の要素であることを「A6.1-7」で証明しました。
よって子を持つ要素は$1$番目の要素、$2$番目の要素 $\cdots$ $\lfloor n/2 \rfloor$番目の要素です。
つまり$\lfloor n/2 \rfloor$番目の要素から$1$番目の要素について`max_heap`を適用すればいいわけです。

プログラムのコード上では$1$番目の要素の添え字は0であり、$\lfloor n/2 \rfloor$番目の要素の添え字は$\lfloor n/2 \rfloor -1$なので注意ください。
更にRUSTでは降順のfor文`for i in (0..5).rev()`は4,3,2,1,0のように指定した最大値から1を引いた値から開始するので、1を足すのを忘れないで下さい。
（プログラム上では開始の添え字について、1を引いてから1を足すという操作をすることになります。）

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
    let heap_size = v.len();
    let start_index = heap_size/2-1;

    for i in (0..start_index+1).rev() {
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

このアルゴリズムの計算時間を考えます。
まず高さ$h$のヒープが持つ全ての節点は$\lceil n/2^{h+1} \rceil$個です。（証明はA6.3-3を参照してください。）
そして要素数$n$の高さは$\lfloor \lg n \rfloor$です。
`build_max_heap`は高さ0から高さ$\lfloor \lg n \rfloor$までの各層の呼び出し時点における全ての節点について`max_heap`を適用します。
`max_heap`の計算時間は、高さ$h$の場合$O(h)$です。
よって総コストは以下になります。

```math
\begin{eqnarray}
\sum_{h=0}^{\lfloor \lg n \rfloor} \Biggl[ \lceil \frac{n}{2^{h+1}} \rceil \Biggl] O(h) = O \Biggl( n \sum_{h=0}^{\lfloor \lg n \rfloor} \frac{h}{2^h} \Biggl)
\end{eqnarray}
```

本書の公式(A.8)より

```math
\begin{eqnarray}
O \Biggl( n \sum_{h=0}^{\lfloor \lg n \rfloor} \frac{h}{2^h} \Biggl) 
&\leq& O \Biggl( n \sum_{h=0}^{\infty} \frac{h}{2^h} \Biggl) \\
&=& O(2n) = O(n)
\end{eqnarray}
```

なので、アルゴリズムの計算時間の上界は$O(n)$という線形になることが分かります。

ちなみに`min_heap`を用いれば、配列をminヒープに並べ替える関数が作成できます。

```rust
fn build_min_heap(v: &mut Vec<i32>) {
    let heap_size = v.len();
    let start_index = heap_size/2-1;

    for i in (0..start_index+1).rev() {
        println!("v={:?}, i={}", &v, i);
        min_heap(v, i);
    }
}
```

## A6.3-1

上記のコードを実行すると以下を得ます。

```Console
v=[5, 3, 17, 10, 84, 19, 6, 22, 9]
v=[5, 3, 17, 10, 84, 19, 6, 22, 9], i=3
v=[5, 3, 17, 22, 84, 19, 6, 10, 9], i=2
v=[5, 3, 19, 22, 84, 17, 6, 10, 9], i=1
v=[5, 84, 19, 22, 3, 17, 6, 10, 9], i=0
sorted v=[84, 22, 19, 10, 3, 17, 6, 5, 9]
```

## A6.3-2
最後から最初の順番に行わなかった場合、高さの浅い部分まで成立していたMaxヒープの順番が、どこかの高さの交換によりMAxヒープを満たさない順番になってしまいます。

## A6.3-3

1) $h=0$の場合$\lceil n/2 \rceil$が成立することの証明
最下層の要素が全て1つ上の層と親子関係にある場合に節点の数が最大になります。

要素数が偶数の場合、上の条件を満たすと最下層は$n/2$個の要素になるので、節点数は$n-n/2 = n/2 = \lceil n/2 \rceil$個です。

要素数が奇数の場合、上の条件を満たすと最下層の最後の要素だけは、親に対して子が１つしかない節点になります。
つまり要素数を$n=2m+1$個として、節点の数は要素数が$n+1=2m+2$の場合と等しくなります。
よって節点数は$n+1-(n+1)/2 = (n+1)/2 = \lceil n/2 \rceil$個です。

以上より、偶数、奇数の両方において、$h=0$の場合に$\lceil n/2 \rceil$が成立します。

2) $h-1$の場合に与式が成立するとした場合に$h$でも成立することの証明

証明方法が思いつかないので解答保留です。
（[https://ita.skanev.com/06/03/03.html](https://ita.skanev.com/06/03/03.html)に回答がありましたが、読んでも理解できませんでした。
葉の要素$\lceil n/2 \rceil$を除いて新しいツリー$n-\lceil n/2 \rceil = \lfloor n/2 \rfloor$を作るというのが、どういう理屈なのか分かりませんでした。）

# 6.4　ヒープソートアルゴリズム
任意の配列をまずはmaxヒープで並べ替えます。
すると最初の要素は根であり、配列要素の最大値になります。
その根の2つの子は、下に続く要素から成る部分木の根とみなせます。つまりその「子」は部分木の根になっているので、自分の部分木の中では最大値ですが、隣の部分木と比べて最大値になっているとは限りません。
そこで根を除いた配列について、その「子」のmaxヒープを修正すると、再び配列の最初の要素が最大値になります。

以降は配列の要素数が2個になるまで同じことを繰り返します。

上記の過程で取り除いた要素を順に並べれば、最大値から最小値へと並び替えられた配列が得られます。

```rust
fn heap_sort(v: &mut Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    let mut temp;
    build_max_heap(v);
    println!("v={:?}", &v);
        
    let length = v.len();
    
    for i in (1..length).rev() {
        temp = v[i];
        v[i] = v[0];
        v[0] = temp;
        println!("v={:?}, sorted_v={:?}, {}", &v, &result, i);
        
        temp = v.pop().unwrap();
        result.push(temp);
    
        max_heap(v, 0);
        println!("v={:?}, sorted_v={:?}, i={}", &v, &result, i);
    }    
    
    temp = v.pop().unwrap();
    result.push(temp);
    result
    
}

fn main() {
    let mut v: Vec<i32> = vec![5,3,17,10,84,19,6,22,9];
    println!("v={:?}", &v);

    println!("sorted v={:?}", heap_sort(&mut v));
}
```

このアルゴリズムの計算量は、build_max_heapが$O(n)$、max_heapが$\Theta(\lg k)$です。
$k$は呼び出す度にサイズが$k=n-1$から$k=1$まで減っていくので、

```math
\begin{eqnarray}
\sum_{i=1}^{n-1} \lg k
&=& \lg (n-1)! \\
&=& \lg (n-1)(n-2) \cdots (n-(n-1)) \\
&=& \lg (n^n + \cdots) \\
&=& \Theta(\lg n^n) = \Theta(n \lg n)
\end{eqnarray}
```

よって計算量は$\Theta(n \lg n)$になります。
build_max_heapの計算量$O(n)$は$\Theta(n \lg n)$に比べると小さいので無視します。
マスター定理から導かれるのは、漸近的な限界$\Theta$なのに、本では漸近的上界$O$になっている理由がよく分かりませんでした。
$\Theta$を満たしていれば$O$も自動に満たしているからと解釈しました。

## A6.4-1とA6.4-2
上記のコードを適用すると以下を得られます。
部分配列が$A[0 \cdots i-1]$まではmaxヒープになっており、$A[i \cdots n-1]$はソートされていることが分かります。

```
v=[25, 13, 20, 8, 7, 17, 2, 5, 4]
v=[20, 13, 17, 8, 7, 4, 2, 5], sorted_v=[25], i=8
v=[17, 13, 5, 8, 7, 4, 2], sorted_v=[25, 20], i=7
v=[13, 8, 5, 2, 7, 4], sorted_v=[25, 20, 17], i=6
v=[8, 7, 5, 2, 4], sorted_v=[25, 20, 17, 13], i=5
v=[7, 4, 5, 2], sorted_v=[25, 20, 17, 13, 8], i=4
v=[5, 4, 2], sorted_v=[25, 20, 17, 13, 8, 7], i=3
v=[4, 2], sorted_v=[25, 20, 17, 13, 8, 7, 5], i=2
v=[2], sorted_v=[25, 20, 17, 13, 8, 7, 5, 4], i=1
sorted v=[25, 20, 17, 13, 8, 7, 5, 4, 2]
```

## A6.4-3
配列が降順であろうと昇順であろうとヒープにはなっていないのでbuild_max_heapによる計算量は変わらないと思われます。
ヒープされた配列に対するmax_heap適用以降のアルゴリズムも、配列が降順であろうと昇順であろうと変わらないので、計算量は同じ$\Theta(n \lg n)$になります。

## A6.4-4

Max Heapの最悪実行時間がA6.2-4より$\Omega(\lg n)$です。
ヒープソートアルゴリズムは、サイズを減少させながらMax Heapを適用させていくので、同様の計算で計算量は$\Omega(n \lg n)$になります。

## A6.4-5
直観的には、ヒープソートアルゴリズムは、配列の要素が全て異なれば、最良でも最悪でもMax Heapの計算量は変わらず、最良実行時間も$\Omega(n \lg n)$になるのだと思われます。

これを正確に証明する方法が思いつかないので、解答保留です。

ちなみにネットには[解答](https://ita.skanev.com/06/04/05.html)が落ちていましたが、「最良のケースが$2^{k-2}-1$個のノードがバイナリーツリーに配置されている時」という意味が分かりませんでした。

# 6.5 優先度付きキュー

## A6.5-1

```rust
fn heap_maximun(v: &Vec<i32>) -> i32 {
    v[0]
}

fn heap_extract_max(v: &mut Vec<i32>) -> Result<i32, &str> {
    let heap_size = v.len();
    if heap_size < 1 {
        return Err("Heap underflow");
    }
    let max = v[0];
    v[0] = v[heap_size-1];
    v.pop();
    max_heap(v, 0);
    
    Ok(max)
}

fn main() {
    let mut v: Vec<i32> = vec![15,13,9,5,12,8,7,4,0,6,2,1];
    println!("v={:?}", &v);
    
    build_max_heap(&mut v);
    println!("max heap v={:?}", &v);
    
    println!("maximum element is ={}", heap_maximun(&v));
    println!("maximum element is ={}", heap_extract_max(&mut v).unwrap());
    println!("max heap with maximum element removed ={:?}", &v);
    
    let mut new_v: Vec<i32> = Vec::new();
    println!("maximum element is ={}", heap_extract_max(&mut new_v).unwrap_err());
}
```

```
v=[13, 1, 9, 5, 12, 8, 7, 4, 0, 6, 2], i=0
v=[13, 12, 9, 5, 1, 8, 7, 4, 0, 6, 2], i=1
v=[13, 12, 9, 5, 6, 8, 7, 4, 0, 1, 2], i=4
maximum element is =15
max heap with maximum element removed =[13, 12, 9, 5, 6, 8, 7, 4, 0, 1, 2]
maximum element is =Heap underflow
```

## A6.5-2

```rust
fn heap_increase_key(v: &mut Vec<i32>, i: usize, key: i32) -> Result<(), &str> {
    let mut tmp;
    let mut index = i-1;
    if key < v[index] {
        return Err("The new key is smaller than the current key");
    }
    v[index] = key;
    println!("v={:?}, i={}", &v, index);
    
    while index>0 && v[(index+1)/2-1] < v[index]
    {
        tmp = v[(index+1)/2-1];
        v[(index+1)/2-1] = v[index];
        v[index] = tmp;
        index = (index+1)/2-1;
        
        println!("v={:?}, i={}", &v, index);
    }
    
    Ok(())
}

fn max_heap_insert(v: &mut Vec<i32>, key: i32) {
    v.push(-1000);
    heap_increase_key(v, v.len(), key);
}

fn main() {
    let mut v: Vec<i32> = vec![15,13,9,5,12,8,7,4,0,6,2,1];
    println!("v={:?}", &v);
    
    build_max_heap(&mut v);
    println!("max heap v={:?}", &v);
    
    max_heap_insert(&mut v, 10);
    
    println!("inserted v={:?}", &v);

    println!("{}", heap_increase_key(&mut v, 1, 10).unwrap_err());
}
```

```
max heap v=[15, 13, 9, 5, 12, 8, 7, 4, 0, 6, 2, 1]
v=[15, 13, 9, 5, 12, 8, 7, 4, 0, 6, 2, 1, 10], i=12
v=[15, 13, 9, 5, 12, 10, 7, 4, 0, 6, 2, 1, 8], i=5
v=[15, 13, 10, 5, 12, 9, 7, 4, 0, 6, 2, 1, 8], i=2
inserted v=[15, 13, 10, 5, 12, 9, 7, 4, 0, 6, 2, 1, 8]
The new key is smaller than the current key
```

## A6.5-3

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
        println!("v={:?}, i={}", &v, i);
        min_heap(v, smallest);
    }
}

fn build_min_heap(v: &mut Vec<i32>) {
    let heap_size = v.len();
    let start_index = heap_size/2-1;

    for i in (0..start_index+1).rev() {
        println!("v={:?}, i={}", &v, i);
        min_heap(v, i);
    }
}

fn heap_minimun(v: &Vec<i32>) -> i32 {
    v[0]
}

fn heap_extract_min(v: &mut Vec<i32>) -> Result<i32, &str> {
    let heap_size = v.len();
    if heap_size < 1 {
        return Err("Heap underflow");
    }
    let min = v[0];
    v[0] = v[heap_size-1];
    v.pop();
    min_heap(v, 0);
    
    Ok(min)
}

fn heap_decrease_key(v: &mut Vec<i32>, i: usize, key: i32) -> Result<(), &str> {
    let mut tmp;
    let mut index = i-1;
    if key > v[index] {
        return Err("The new key is larger than the current key");
    }
    v[index] = key;
    println!("v={:?}, i={}", &v, index);
    
    while index>0 && v[(index+1)/2-1] > v[index]
    {
        tmp = v[(index+1)/2-1];
        v[(index+1)/2-1] = v[index];
        v[index] = tmp;
        index = (index+1)/2-1;
        
        println!("v={:?}, i={}", &v, index);
    }
    
    Ok(())
}

fn min_heap_insert(v: &mut Vec<i32>, key: i32) {
    v.push(1000);
    heap_decrease_key(v, v.len(), key);
}

fn main() {
    let mut v: Vec<i32> = vec![15,13,9,5,12,8,7,4,0,6,2,1];
    println!("v={:?}", &v);
    
    build_min_heap(&mut v);
    println!("min heap v={:?}", &v);
    
    
    println!("minimum element is ={}", heap_minimun(&v));
    println!("minimum element is ={}", heap_extract_min(&mut v).unwrap());
    println!("min heap with minimum element removed ={:?}", &v);
    
    
    let mut new_v: Vec<i32> = Vec::new();
    println!("{}", heap_extract_min(&mut new_v).unwrap_err());

    min_heap_insert(&mut v, 3);
    
    println!("inserted v={:?}", &v);
    

    println!("{}", heap_decrease_key(&mut v, 7, 100).unwrap_err());
}
```


```
min heap v=[0, 2, 1, 4, 6, 8, 7, 13, 5, 15, 12, 9]
minimum element is =0
v=[1, 2, 9, 4, 6, 8, 7, 13, 5, 15, 12], i=0
v=[1, 2, 7, 4, 6, 8, 9, 13, 5, 15, 12], i=2
minimum element is =0
min heap with minimum element removed =[1, 2, 7, 4, 6, 8, 9, 13, 5, 15, 12]
Heap underflow
v=[1, 2, 7, 4, 6, 8, 9, 13, 5, 15, 12, 3], i=11
v=[1, 2, 7, 4, 6, 3, 9, 13, 5, 15, 12, 8], i=5
v=[1, 2, 3, 4, 6, 7, 9, 13, 5, 15, 12, 8], i=2
inserted v=[1, 2, 3, 4, 6, 7, 9, 13, 5, 15, 12, 8]
The new key is larger than the current key
```

## A6.5-4

初回の挿入でエラーになるのを回避する為です。

## A6.5-5




# 他の記事
- [個人的勉強メモ「アルゴリズム・イントロダクション」をRustで実装する：2章](https://qiita.com/Kosuke_Matsui/items/eea26e88cd261173a292)
- [個人的勉強メモ「アルゴリズム・イントロダクション」をRustで実装する：3章](https://qiita.com/Kosuke_Matsui/private/b424d8eb30dd16455b12)
- [個人的勉強メモ「アルゴリズム・イントロダクション」をRustで実装する：4章](https://qiita.com/Kosuke_Matsui/private/3b93d6b8a7ee5e90a2b5)
- [個人的勉強メモ「アルゴリズム・イントロダクション」をRustで実装する：6章](https://qiita.com/Kosuke_Matsui/private/8d1586a463a78081d533)
