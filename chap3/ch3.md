---
title: 個人的勉強メモ「アルゴリズム・イントロダクション」をRustで実装する：3章
tags: アルゴリズム Rust
author: Kosuke_Matsui
slide: false
---
# 記事の概要

アルゴリズムイントロダクション 第3版 総合版：世界標準MIT教科書のアルゴリズムをRustで実装しながら勉強したので、その結果を整理しました。
まとめた内容には確実に誤りがあると思われるので、参照にあたっては、あらかじめご了承ください。

個人的な勉強メモなので他人様に見やすいものになっていません。
また、本の著作権を守るために、本を参照しながらでないと理解できないように書いています。

今回は3章について勉強しました。
計算ばかりなのでRustは使っていません。
未解答の問題もありますが、飽きたので4章に進みます。4章が終わったら取り組もうと思っています。

# 3.1 漸近記法

$\Theta(g(n))$ の集合に属する $f(n) \in \Theta(g(n))$ は正の定数 $c_1,c_2,n_0$ に対して $0 \leq c_1 g(n) \leq f(n) \leq c_2 g(n)$ を満たします。
$g(n)$ は $n$ の増加により負になってはいけません。


## O記法

### 定義
$O(g(n))$ の集合に属する $f(n) \in O(g(n))$ は正の定数 $c$ に対して $0 \leq f(n) \leq c g(n)$ を満たします。

### 例1
例えば$an^3+bn^2+cn+d$は$1 \leq n$ならば

```math
\begin{eqnarray}
an^3+bn^2+cn+d 
&\leq& a(n^3+\frac{b}{a}n^2+\frac{c}{a}n+\frac{d}{a}) \\
&\leq& a(n^3+\frac{b}{a}n^3+\frac{c}{a}n^3+\frac{d}{a}n^3) \\
&=& a(1+\frac{b}{a}+\frac{c}{a}+\frac{d}{a})n^3
\end{eqnarray}
```

なので$O(n^3)$に属します。

### 例2
$an+|b|$ は$O(n^2)$に属します。

$a>0, c=a+|b|$として、$n_0 \geq n$ならば

```math
\begin{eqnarray}
an+|b| \leq (a+|b|)n^2
\end{eqnarray}
```

を必ず満たす整数$n_0$が存在するならば成立します。

例えば$n_0$を2次方程式の解

```math
\begin{eqnarray}
\frac{a+\sqrt{a^2+4(a-b)^2}}{a-b}
\end{eqnarray}
```

に選べば、$n_0 \leq n$ は常に正になります。

もしくは

```math
\begin{eqnarray}
\lim_{n \to \infty} \frac{f(n)}{g(n)} = Const
\end{eqnarray}
```

を満たすという定義でもいいようです。

https://detail.chiebukuro.yahoo.co.jp/qa/question_detail/q11188678831


## o記法

$2n^2 = O(n^2)$については

```math
\begin{eqnarray}
\lim_{n \to \infty} \frac{2 n^2}{n^2} = 2 \neq 0
\end{eqnarray}
```

なので$2n^2 \neq o(n^2)$です。



$2n = O(n^2)$については

```math
\begin{eqnarray}
\lim_{n \to \infty} \frac{2 n}{n^2} = \frac{2}{n} = 0
\end{eqnarray}
```

なので$2n = o(n^2)$です。


## $\Omega$記法


## A3.1-1

 $\mathrm{max}(f(n), g(n)) \in \Theta(f(n) + g(n))$ になることを証明するには、$\mathrm{max}(f(n), g(n))$ が以下を満たすことを示します。


```math
\begin{eqnarray}
0 \leq c_1 (f(n)+g(n)) \leq \mathrm{max}(f(n), g(n)) \leq c_2 (f(n)+g(n))
\end{eqnarray}
```


$c_1 = 0.5$ とすれば

```math
\begin{eqnarray}
0.5f(n) + 0.5g(n) &\leq& 0.5\mathrm{max}(f(n), g(n)) + 0.5\mathrm{max}(f(n), g(n)) \\
&=& \mathrm{max}(f(n), g(n))
\end{eqnarray}
```

となり

```math
\begin{eqnarray}
c_1 (f(n)+g(n)) \leq \mathrm{max}(f(n), g(n))
\end{eqnarray}
```

を満たします。


$c_2 = 1$ とすれば

```math
\begin{eqnarray}
\mathrm{max}(f(n), g(n)) &\leq& f(n) + g(n) \\
&=& \mathrm{max}(f(n), g(n)) + \mathrm{min}(f(n), g(n))
\end{eqnarray}
```

となり

```math
\begin{eqnarray}
\mathrm{max}(f(n), g(n)) \leq c_2 (f(n)+g(n)) 
\end{eqnarray}
```

を満たします。
よって $\mathrm{max}(f(n), g(n)) \in \Theta(f(n) + g(n))$ が成立します。

## A3.1-2

$(n+a)^b$ をテーラ展開すれば最高次数は $n^b$ になるので計算の次元が $\Theta(n^b)$ になるのは自明と思ったのですが、この問題は定理3.1を使って証明させたいのかもしれません。

まず $(n+a)^b = O(n^b)$ を示します。
$n \geq n_0$に対して $0 \leq (n+a)^b \leq c n^b$ になるように定数を選びます。
$c=k^b$に選べば $0 \leq (n+a)^b \leq (kn)^b$ です。

```math
\begin{eqnarray}
n_0+a &\leq& kn_0 \\
a &\leq& (k-1)n_0 \\
\frac{a}{k-1} &\leq& n_0
\end{eqnarray}
```

になるような$n_0$を選べば常に成立するので、 $(n+a)^b = O(n^b)$ です。

次に $(n+a)^b = \Omega(n^b)$ を示します。
$n \geq n_0$に対して $0 \leq c n^b \leq (n+a)^b$ になるように定数を選びます。
$c=1$に選べば、$0 \leq n_0$に対して常に成立するので、 $(n+a)^b = \Omega(n^b)$ です。

よって定理3.1より$(n+a)^b = \Theta(n^b)$ になります。

## A3.1-3

$O(n^2)$ のアルゴリズムは計算回数が0でもいいので、最小時間を論じるには持ち出すべきではないと考えます

## A3.1-4
$2^{n+1}$は$2*2^n = c2^n$の形に分解できるので$2^{n+1}=O(2^n)$が成立します。

$2^{2n}$は$2^n*2^n \neq c2^n$なので、$2^{2n}=O(2^n)$が成立しません。

## A3.1-5
$\Theta(g(n))$ならば、必ず$O(g(n))$と$\Omega(g(n))$を満たします。

$O(g(n))$と$\Omega(g(n))$ならば、 $0 \leq c_1 g(n) \leq f(n) \leq c_2 g(n)$ を満たす定数が存在するので$\Theta(g(n))$を満たします。

## A3.1-6
定理3.1から$\Theta(g(n))$ならば、必ず$O(g(n))$と$\Omega(g(n))$を満たし、$O(g(n))$と$\Omega(g(n))$の定義から、最悪計算実行時間が$O(g(n))$になり、最良計算実行時間が$\Omega(g(n))$になります

## A3.1-7

$o$記法と$\omega$記法の定義、$\lim f(n)/g(n) = 0$と$\lim f(n)/g(n) = \infty$を同時に満たすことはできないので、空集合になります。

## A3.1-8

$\Omega(g(n,m)) = f(n,m)$：正定数$c,n_0,m_0$が存在して、$n \geq n_0$あるいは$m \geq m_0$を満たす全ての$n,m$に対して$0 \leq cg(n,m) \leq f(n,m)$を満たす。

$\Theta(g(n,m)) = f(n,m)$：正定数$c_1,c_2,n_0,m_0$が存在して、$n \geq n_0$あるいは$m \geq m_0$を満たす全ての$n,m$に対して$0 \leq c_1g(n,m) \leq f(n,m) \leq c_2g(n,m)$を満たす。

# 標準的な記法

## A3.2-1

$n<m$に対して$f(n)<f(m)$かつ$g(n)<g(m)$なので$f(n)+g(n)<f(m)+g(m)$となります。よって$f(n)+g(n)$は単調増加関数です。

$n<m$に対して$0<f(n)<f(m)$かつ$0<g(n)<g(m)$なので$f(n)*g(n)<f(m)*g(m)$となります。よって$f(n)*g(n)$は単調増加関数です。

## A3.2-2

### (3.16)-1

```math
\begin{eqnarray}
\log_b a = x 
\end{eqnarray}
```

```math
\begin{eqnarray}
a = b^x 
\end{eqnarray}
```

```math
\begin{eqnarray}
a^{1/x} = b 
\end{eqnarray}
```

```math
\begin{eqnarray}
\frac{1}{x} = \log_a b 
\end{eqnarray}
```


```math
\begin{eqnarray}
x = \frac{1}{\log_a b} 
\end{eqnarray}
```

よって

```math
\begin{eqnarray}
\log_b a = \frac{1}{\log_a b} 
\end{eqnarray}
```

となります。


### (3.16)-2

```math
\begin{eqnarray}
a^{\log_b c} = x 
\end{eqnarray}
```

```math
\begin{eqnarray}
\log_a x &=& \log_b c\\
&=& \frac{\log_a b}{\log_a c}
\end{eqnarray}
```

```math
\begin{eqnarray}
\log_b c &=& \log_a x \\
&=& \frac{\log_b x}{\log_b a} 
\end{eqnarray}
```

```math
\begin{eqnarray}
\log_b x &=& \log_b a \log_b c \\
&=& \log_b c^{\log_b a}
\end{eqnarray}
```

```math
\begin{eqnarray}
x &=& c^{\log_b a}
\end{eqnarray}
```

よって

```math
\begin{eqnarray}
a^{\log_b c} = c^{\log_b a}
\end{eqnarray}
```

となります。


## A3.2-3

```math
\begin{eqnarray}
\lg(n!) 
&=& \lg \biggl(  \sqrt{2\pi n} \Bigl( \frac{n}{e} \Bigl)^n \biggl( 1 + \Theta \Bigl( \frac{1}{n} \Bigl) \Bigl) \biggl) \\
&=& \frac{1}{2} \lg (2\pi n) + n \lg n - n\lg e + \lg \Bigl( 1 + \Theta \Bigl( \frac{1}{n} \Bigl) \Bigl) 
\end{eqnarray}
```

主に計算量に寄与するのは $n \lg n$ なので、計算の次元は $\Theta(n \lg n)$ になります。

### n!の$\omega$記法

```math
\begin{eqnarray}
\lim_{n \to \infty} \frac{2^n}{n!}
&=& \lim_{n \to \infty} \frac{1}{\sqrt{2\pi n} \biggl( 1 + \Theta \Bigl( \frac{1}{n} \Bigl) \biggl) } \Bigl( \frac{2e}{n} \Bigl)^n
\leq \lim_{n \to \infty} \Bigl( \frac{2e}{n} \Bigl)^n
\end{eqnarray}
```

上式では最初の式の分数が1未満になることを用いました。

$n>4e$とすれば、$1/2>2e/n$になるので、上式は

```math
\begin{eqnarray}
\lim_{n \to \infty} \frac{2^n}{n!}
\leq \lim_{n \to \infty}  \frac{1}{2^n}
\end{eqnarray}
```

となるので$n! = \omega(2^n)$が成立します。

### n!のo記法

```math
\begin{eqnarray}
\lim_{n \to \infty} \frac{n^n}{n!}
&=& \lim_{n \to \infty} \frac{1}{\sqrt{2\pi n} \biggl( 1 + \Theta \Bigl( \frac{1}{n} \Bigl) \biggl) } e^n
\geq \lim_{n \to \infty} \frac{e^n}{c \sqrt{n}} = \infty
\end{eqnarray}
```

となるので$n! = o(n^n)$が成立します。

## A3.2-4
$\lg n!$ が多項式に限定されるとは $\lg n! = O(n^k)$ を満たすということです。
Oの定義に従えば全ての$n \geq n_0$ に対して $\lg n! \leq cn^k$ を満たすかどうかです。

$n = 2^m$ とすれば $m! \leq c2^{mk}$ となりますが、$m$が十分大きくなると、この式は成立しません。
よって$\lg n!$ は多項式には限定されません。


$\lg\lg n!$ が多項式に限定されるとは $\lg\lg n! = O(n^k)$ を満たすということです。
Oの定義に従えば全ての$n \geq n_0$ に対して $\lg\lg n! \leq cn^k$ を満たすかどうかです。

$n = 2^m$ とすれば $\lg m! \leq c2^{mk}$ となります。
$m = 2^t$ とすれば $t! \leq c2^{2^{t}k}$ となります。
この式は$k \geq 1$ に対して成立するので、$\lg\lg n!$ は多項式に限定されます。

## A3.2-5
$\lg^* n = x$　として $\lg^* 2^n = 1+x$ なので $\lg^* 2^n = 1 + \lg^* n$ 


```math
\begin{eqnarray}
\lim_{n \to \infty} \frac{\lg \lg^* n}{\lg^* \lg n} 
&=& \lim_{m \to \infty} \frac{\lg \lg^* 2^m}{\lg^* \lg 2^m} 
= \lim_{m \to \infty} \frac{\lg (1+\lg^* m)}{\lg^* m} \\
&=& \lim_{x \to \infty} \frac{\lg (1+x)}{x} = 0
\end{eqnarray}
```

よって $\lg^* \lg n$ が漸近的に大きい。

## A3.2-6

```math
\begin{eqnarray}
(\frac{1+\sqrt{5}}{2})^2-\frac{1+\sqrt{5}}{2}-1 
&=& \frac{1+5+2\sqrt{5} - 2-2\sqrt{5} -4}{4} = 0
\end{eqnarray}
```

```math
\begin{eqnarray}
(\frac{1-\sqrt{5}}{2})^2-\frac{1-\sqrt{5}}{2}-1 
&=& \frac{1+5-2\sqrt{5} - 2+2\sqrt{5} -4}{4} = 0
\end{eqnarray}
```

## A3.2-7

A3.2-6より

```math
\begin{eqnarray}
1+\phi = \phi^2
\end{eqnarray}
```

```math
\begin{eqnarray}
1+\hat{\phi} = \hat{\phi}^2
\end{eqnarray}
```

よって

```math
\begin{eqnarray}
F_{i-1}+F_{i-2} 
&=& \frac{\phi^{i-1} + \phi^{i-2} - (\hat{\phi}^{i-1} + \hat{\phi}^{i-2})}{\sqrt{5}} \\
&=& \frac{\phi^{i-2} (\phi+1) - \hat{\phi}^{i-2} (\hat{\phi}+1)}{\sqrt{5}} \\
&=& \frac{\phi^{i-2} \phi^2 - \hat{\phi}^{i-2} \hat{\phi}^2}{\sqrt{5}} \\
&=& \frac{\phi^i- \hat{\phi}^i}{\sqrt{5}} \\
&=& F_i
\end{eqnarray}
```


## A3.2-8
未解答


# 3-1

## a

$0 \leq p(n) \leq c n^k$を示します。

$c> \sum_{i=0}^d a_i$になるように選べば、$k \geq d$より上式を満たすので、$p(n) = O(n^k)$が成立します

## b

$0 \leq c p(n) \leq n^k$を示します。

$1/c < a_k$になるように選べば、$k \leq d$より上式を満たすので、$p(n) = \Omega(n^k)$が成立します

## c

$k = d$ならばaとbの両方を満たすので$p(n) = \Theta(n^k)$が成立します

## d


```math
\begin{eqnarray}
\lim_{n \to \infty} \frac{p(n)}{n^k}
&=& \lim_{n \to \infty} \sum_{i=0}^d a_i n^{i-k} = 0
\end{eqnarray}
```

$k<d$なので$i-k$は常に負なので極限を取ると全ての項が0に収束します。
よって$p(n)= o(n^k)$が成立します。

## e

```math
\begin{eqnarray}
\lim_{n \to \infty} \frac{n^k}{p(n)}
&=& \lim_{n \to \infty} \sum_{i=0}^d \frac{1}{a_i} n^{k-i} = 0
\end{eqnarray}
```


$k>d$なので$k-i$は常に負なので極限を取ると全ての項が0に収束します。
よって$p(n)= \omega(n^k)$が成立します。

# 3-2

## a

```math
\begin{eqnarray}
\lim_{n \to \infty} \frac{\lg^k n}{n^{\epsilon}} = 0
\end{eqnarray}
```

が成立するので、$\lg^k n = O(n^{\epsilon})$ かつ $\lg^k n = o(n^{\epsilon})$です。

$\lg^k n = o(n^{\epsilon})$が成立するので、$\lg^k n \neq \Omega(n^{\epsilon})$ かつ $\lg^k n \neq \omega(n^{\epsilon})$であり、$\lg^k n \neq \Theta(n^{\epsilon})$です。

## b

```math
\begin{eqnarray}
\lim_{n \to \infty} \frac{n^k}{c^n} = 0
\end{eqnarray}
```

が成立するので、$n^k = O(c^n)$ かつ $n^k = o(c^n)$です。

$n^k = o(c^n)$が成立するので$n^k \neq \Omega(c^n)$ かつ $n^k \neq \omega(c^n)$であり、$n^k \neq \Theta(c^n)$です。

## c

$\sin n$の値域は$-1 \leq \sin n \leq 1$であり、$n^{1/2}$の乗数は1未満です。

```math
\begin{eqnarray}
\lim_{n \to \infty} \frac{n^{1/2}}{n^{\sin n}}
\end{eqnarray}
```

は有限値に収束せず、0にも$\infty$にも収束しません。
よって全てが成立しません。

## d

```math
\begin{eqnarray}
\lim_{n \to \infty} \frac{2^n}{2^{n/2}} = \lim_{n \to \infty} 2^{n/2} = \infty
\end{eqnarray}
```


が成立するので、$2^n = \Omega(2^{n/2})$ かつ $2^n = \omega(c2^{n/2})$です。

$2^n = \omega(2^{n/2})$が成立するので$2^n \neq O(2^{n/2})$ かつ $2^n \neq o(2^{n/2})$であり、$2^n \neq \Theta(2^{n/2})$です。

## e

$n^{\lg c} = c^{\lg n}$なので

```math
\begin{eqnarray}
\lim_{n \to \infty} \frac{n^{\lg c}}{c^{\lg n}} = \lim_{n \to \infty} 1 = 1
\end{eqnarray}
```

この式は有限値には収束しますが、0や$\infty$には収束しないので、$n^{\lg c} = O(c^{\lg n})$ かつ $n^{\lg c} = \Omega(c^{\lg n})$ かつ $n^{\lg c} = \Theta(c^{\lg n})$ ですが、 $n^{\lg c} \neq o(c^{\lg n})$ と $n^{\lg c} \neq \omega(c^{\lg n})$ です。

## f


```math
\begin{eqnarray}
\lim_{n \to \infty} \frac{\lg n!}{\lg n^n} 
&=& \lim_{n \to \infty} \frac{\lg \sqrt{2 \pi n} + \lg n^n - \lg e^n + \lg e^{\alpha}}{\lg n^n} \\
&=& \lim_{n \to \infty} \frac{\frac{1}{2} \lg (2 \pi n) + n \lg n - n \lg e + \lg e^{\alpha}}{n \lg n} \\
&=& \lim_{n \to \infty} \frac{\frac{1}{2} \lg n + \lg n}{\lg n} = \frac{3}{2}
\end{eqnarray}
```

途中の式展開では、分母が無限に発散して0に収束する項は省きました。
この式は有限値には収束しますが、0や$\infty$には収束しないので、$\lg n!= O(\lg n^n)$ かつ $\lg n! = \Omega(\lg n^n)$ かつ $\lg n! = \Theta(\lg n^n)$ ですが、 $\lg n! \neq o(\lg n^n)$ と $\lg n! \neq \omega(\lg n^n)$ です。

# 3-3
未解答

# 3-4
否定する場合は反例を1つ挙げるだけで証明になります。

## a

$an+b = O(n^2)$ になることが本文中に解説されていました。
ですが $n^2 = O(an+b) = O(n)$ にはなりません。
よって問題文の関係は成立しません。 

## b

$f(n) = n^2$ , $g(n) = n$ として　$min(f(n),g(n)) = g(n) =n$ になります。
$\Theta(n^2+n) \neq \Theta(n)$ なので問題文の関係は成立しません。



# 3-5
未解答

# 3-6
未解答

# 他の記事
- [個人的勉強メモ「アルゴリズム・イントロダクション」をRustで実装する：2章](https://qiita.com/Kosuke_Matsui/items/eea26e88cd261173a292)
- [個人的勉強メモ「アルゴリズム・イントロダクション」をRustで実装する：3章](https://qiita.com/Kosuke_Matsui/private/b424d8eb30dd16455b12)
- [個人的勉強メモ「アルゴリズム・イントロダクション」をRustで実装する：4章](https://qiita.com/Kosuke_Matsui/private/3b93d6b8a7ee5e90a2b5)


