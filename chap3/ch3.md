---
title: �l�I�׋������u�A���S���Y���E�C���g���_�N�V�����v��Rust�Ŏ�������F3��
tags: �A���S���Y�� Rust
author: Kosuke_Matsui
slide: false
---
# �L���̊T�v

�A���S���Y���C���g���_�N�V���� ��3�� �����ŁF���E�W��MIT���ȏ��̃A���S���Y����Rust�Ŏ������Ȃ���׋������̂ŁA���̌��ʂ𐮗����܂����B
�܂Ƃ߂����e�ɂ͊m���Ɍ�肪����Ǝv����̂ŁA�Q�Ƃɂ������ẮA���炩���߂��������������B

�l�I�ȕ׋������Ȃ̂ő��l�l�Ɍ��₷�����̂ɂȂ��Ă��܂���B
�܂��A�{�̒��쌠����邽�߂ɁA�{���Q�Ƃ��Ȃ���łȂ��Ɨ����ł��Ȃ��悤�ɏ����Ă��܂��B

�����3�͂ɂ��ĕ׋����܂����B
�v�Z�΂���Ȃ̂�Rust�͎g���Ă��܂���B
���𓚂̖�������܂����A�O�����̂�4�͂ɐi�݂܂��B4�͂��I���������g�����Ǝv���Ă��܂��B

# 3.1 �Q�ߋL�@

$\Theta(g(n))$ �̏W���ɑ����� $f(n) \in \Theta(g(n))$ �͐��̒萔 $c_1,c_2,n_0$ �ɑ΂��� $0 \leq c_1 g(n) \leq f(n) \leq c_2 g(n)$ �𖞂����܂��B
$g(n)$ �� $n$ �̑����ɂ�蕉�ɂȂ��Ă͂����܂���B


## O�L�@

### ��`
$O(g(n))$ �̏W���ɑ����� $f(n) \in O(g(n))$ �͐��̒萔 $c$ �ɑ΂��� $0 \leq f(n) \leq c g(n)$ �𖞂����܂��B

### ��1
�Ⴆ��$an^3+bn^2+cn+d$��$1 \leq n$�Ȃ��

```math
\begin{eqnarray}
an^3+bn^2+cn+d 
&\leq& a(n^3+\frac{b}{a}n^2+\frac{c}{a}n+\frac{d}{a}) \\
&\leq& a(n^3+\frac{b}{a}n^3+\frac{c}{a}n^3+\frac{d}{a}n^3) \\
&=& a(1+\frac{b}{a}+\frac{c}{a}+\frac{d}{a})n^3
\end{eqnarray}
```

�Ȃ̂�$O(n^3)$�ɑ����܂��B

### ��2
$an+|b|$ ��$O(n^2)$�ɑ����܂��B

$a>0, c=a+|b|$�Ƃ��āA$n_0 \geq n$�Ȃ��

```math
\begin{eqnarray}
an+|b| \leq (a+|b|)n^2
\end{eqnarray}
```

��K������������$n_0$�����݂���Ȃ�ΐ������܂��B

�Ⴆ��$n_0$��2���������̉�

```math
\begin{eqnarray}
\frac{a+\sqrt{a^2+4(a-b)^2}}{a-b}
\end{eqnarray}
```

�ɑI�ׂ΁A$n_0 \leq n$ �͏�ɐ��ɂȂ�܂��B

��������

```math
\begin{eqnarray}
\lim_{n \to \infty} \frac{f(n)}{g(n)} = Const
\end{eqnarray}
```

�𖞂����Ƃ�����`�ł������悤�ł��B

https://detail.chiebukuro.yahoo.co.jp/qa/question_detail/q11188678831


## o�L�@

$2n^2 = O(n^2)$�ɂ��Ă�

```math
\begin{eqnarray}
\lim_{n \to \infty} \frac{2 n^2}{n^2} = 2 \neq 0
\end{eqnarray}
```

�Ȃ̂�$2n^2 \neq o(n^2)$�ł��B



$2n = O(n^2)$�ɂ��Ă�

```math
\begin{eqnarray}
\lim_{n \to \infty} \frac{2 n}{n^2} = \frac{2}{n} = 0
\end{eqnarray}
```

�Ȃ̂�$2n = o(n^2)$�ł��B


## $\Omega$�L�@


## A3.1-1

 $\mathrm{max}(f(n), g(n)) \in \Theta(f(n) + g(n))$ �ɂȂ邱�Ƃ��ؖ�����ɂ́A$\mathrm{max}(f(n), g(n))$ ���ȉ��𖞂������Ƃ������܂��B


```math
\begin{eqnarray}
0 \leq c_1 (f(n)+g(n)) \leq \mathrm{max}(f(n), g(n)) \leq c_2 (f(n)+g(n))
\end{eqnarray}
```


$c_1 = 0.5$ �Ƃ����

```math
\begin{eqnarray}
0.5f(n) + 0.5g(n) &\leq& 0.5\mathrm{max}(f(n), g(n)) + 0.5\mathrm{max}(f(n), g(n)) \\
&=& \mathrm{max}(f(n), g(n))
\end{eqnarray}
```

�ƂȂ�

```math
\begin{eqnarray}
c_1 (f(n)+g(n)) \leq \mathrm{max}(f(n), g(n))
\end{eqnarray}
```

�𖞂����܂��B


$c_2 = 1$ �Ƃ����

```math
\begin{eqnarray}
\mathrm{max}(f(n), g(n)) &\leq& f(n) + g(n) \\
&=& \mathrm{max}(f(n), g(n)) + \mathrm{min}(f(n), g(n))
\end{eqnarray}
```

�ƂȂ�

```math
\begin{eqnarray}
\mathrm{max}(f(n), g(n)) \leq c_2 (f(n)+g(n)) 
\end{eqnarray}
```

�𖞂����܂��B
����� $\mathrm{max}(f(n), g(n)) \in \Theta(f(n) + g(n))$ ���������܂��B

## A3.1-2

$(n+a)^b$ ���e�[���W�J����΍ō������� $n^b$ �ɂȂ�̂Ōv�Z�̎����� $\Theta(n^b)$ �ɂȂ�͎̂����Ǝv�����̂ł����A���̖��͒藝3.1���g���ďؖ����������̂�������܂���B

�܂� $(n+a)^b = O(n^b)$ �������܂��B
$n \geq n_0$�ɑ΂��� $0 \leq (n+a)^b \leq c n^b$ �ɂȂ�悤�ɒ萔��I�т܂��B
$c=k^b$�ɑI�ׂ� $0 \leq (n+a)^b \leq (kn)^b$ �ł��B

```math
\begin{eqnarray}
n_0+a &\leq& kn_0 \\
a &\leq& (k-1)n_0 \\
\frac{a}{k-1} &\leq& n_0
\end{eqnarray}
```

�ɂȂ�悤��$n_0$��I�ׂΏ�ɐ�������̂ŁA $(n+a)^b = O(n^b)$ �ł��B

���� $(n+a)^b = \Omega(n^b)$ �������܂��B
$n \geq n_0$�ɑ΂��� $0 \leq c n^b \leq (n+a)^b$ �ɂȂ�悤�ɒ萔��I�т܂��B
$c=1$�ɑI�ׂ΁A$0 \leq n_0$�ɑ΂��ď�ɐ�������̂ŁA $(n+a)^b = \Omega(n^b)$ �ł��B

����Ē藝3.1���$(n+a)^b = \Theta(n^b)$ �ɂȂ�܂��B

## A3.1-3

$O(n^2)$ �̃A���S���Y���͌v�Z�񐔂�0�ł������̂ŁA�ŏ����Ԃ�_����ɂ͎����o���ׂ��ł͂Ȃ��ƍl���܂�

## A3.1-4
$2^{n+1}$��$2*2^n = c2^n$�̌`�ɕ����ł���̂�$2^{n+1}=O(2^n)$���������܂��B

$2^{2n}$��$2^n*2^n \neq c2^n$�Ȃ̂ŁA$2^{2n}=O(2^n)$���������܂���B

## A3.1-5
$\Theta(g(n))$�Ȃ�΁A�K��$O(g(n))$��$\Omega(g(n))$�𖞂����܂��B

$O(g(n))$��$\Omega(g(n))$�Ȃ�΁A $0 \leq c_1 g(n) \leq f(n) \leq c_2 g(n)$ �𖞂����萔�����݂���̂�$\Theta(g(n))$�𖞂����܂��B

## A3.1-6
�藝3.1����$\Theta(g(n))$�Ȃ�΁A�K��$O(g(n))$��$\Omega(g(n))$�𖞂����A$O(g(n))$��$\Omega(g(n))$�̒�`����A�ň��v�Z���s���Ԃ�$O(g(n))$�ɂȂ�A�ŗǌv�Z���s���Ԃ�$\Omega(g(n))$�ɂȂ�܂�

## A3.1-7

$o$�L�@��$\omega$�L�@�̒�`�A$\lim f(n)/g(n) = 0$��$\lim f(n)/g(n) = \infty$�𓯎��ɖ��������Ƃ͂ł��Ȃ��̂ŁA��W���ɂȂ�܂��B

## A3.1-8

$\Omega(g(n,m)) = f(n,m)$�F���萔$c,n_0,m_0$�����݂��āA$n \geq n_0$���邢��$m \geq m_0$�𖞂����S�Ă�$n,m$�ɑ΂���$0 \leq cg(n,m) \leq f(n,m)$�𖞂����B

$\Theta(g(n,m)) = f(n,m)$�F���萔$c_1,c_2,n_0,m_0$�����݂��āA$n \geq n_0$���邢��$m \geq m_0$�𖞂����S�Ă�$n,m$�ɑ΂���$0 \leq c_1g(n,m) \leq f(n,m) \leq c_2g(n,m)$�𖞂����B

# �W���I�ȋL�@

## A3.2-1

$n<m$�ɑ΂���$f(n)<f(m)$����$g(n)<g(m)$�Ȃ̂�$f(n)+g(n)<f(m)+g(m)$�ƂȂ�܂��B�����$f(n)+g(n)$�͒P�������֐��ł��B

$n<m$�ɑ΂���$0<f(n)<f(m)$����$0<g(n)<g(m)$�Ȃ̂�$f(n)*g(n)<f(m)*g(m)$�ƂȂ�܂��B�����$f(n)*g(n)$�͒P�������֐��ł��B

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

�����

```math
\begin{eqnarray}
\log_b a = \frac{1}{\log_a b} 
\end{eqnarray}
```

�ƂȂ�܂��B


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

�����

```math
\begin{eqnarray}
a^{\log_b c} = c^{\log_b a}
\end{eqnarray}
```

�ƂȂ�܂��B


## A3.2-3

```math
\begin{eqnarray}
\lg(n!) 
&=& \lg \biggl(  \sqrt{2\pi n} \Bigl( \frac{n}{e} \Bigl)^n \biggl( 1 + \Theta \Bigl( \frac{1}{n} \Bigl) \Bigl) \biggl) \\
&=& \frac{1}{2} \lg (2\pi n) + n \lg n - n\lg e + \lg \Bigl( 1 + \Theta \Bigl( \frac{1}{n} \Bigl) \Bigl) 
\end{eqnarray}
```

��Ɍv�Z�ʂɊ�^����̂� $n \lg n$ �Ȃ̂ŁA�v�Z�̎����� $\Theta(n \lg n)$ �ɂȂ�܂��B

### n!��$\omega$�L�@

```math
\begin{eqnarray}
\lim_{n \to \infty} \frac{2^n}{n!}
&=& \lim_{n \to \infty} \frac{1}{\sqrt{2\pi n} \biggl( 1 + \Theta \Bigl( \frac{1}{n} \Bigl) \biggl) } \Bigl( \frac{2e}{n} \Bigl)^n
\leq \lim_{n \to \infty} \Bigl( \frac{2e}{n} \Bigl)^n
\end{eqnarray}
```

�㎮�ł͍ŏ��̎��̕�����1�����ɂȂ邱�Ƃ�p���܂����B

$n>4e$�Ƃ���΁A$1/2>2e/n$�ɂȂ�̂ŁA�㎮��

```math
\begin{eqnarray}
\lim_{n \to \infty} \frac{2^n}{n!}
\leq \lim_{n \to \infty}  \frac{1}{2^n}
\end{eqnarray}
```

�ƂȂ�̂�$n! = \omega(2^n)$���������܂��B

### n!��o�L�@

```math
\begin{eqnarray}
\lim_{n \to \infty} \frac{n^n}{n!}
&=& \lim_{n \to \infty} \frac{1}{\sqrt{2\pi n} \biggl( 1 + \Theta \Bigl( \frac{1}{n} \Bigl) \biggl) } e^n
\geq \lim_{n \to \infty} \frac{e^n}{c \sqrt{n}} = \infty
\end{eqnarray}
```

�ƂȂ�̂�$n! = o(n^n)$���������܂��B

## A3.2-4
$\lg n!$ ���������Ɍ��肳���Ƃ� $\lg n! = O(n^k)$ �𖞂����Ƃ������Ƃł��B
O�̒�`�ɏ]���ΑS�Ă�$n \geq n_0$ �ɑ΂��� $\lg n! \leq cn^k$ �𖞂������ǂ����ł��B

$n = 2^m$ �Ƃ���� $m! \leq c2^{mk}$ �ƂȂ�܂����A$m$���\���傫���Ȃ�ƁA���̎��͐������܂���B
�����$\lg n!$ �͑������ɂ͌��肳��܂���B


$\lg\lg n!$ ���������Ɍ��肳���Ƃ� $\lg\lg n! = O(n^k)$ �𖞂����Ƃ������Ƃł��B
O�̒�`�ɏ]���ΑS�Ă�$n \geq n_0$ �ɑ΂��� $\lg\lg n! \leq cn^k$ �𖞂������ǂ����ł��B

$n = 2^m$ �Ƃ���� $\lg m! \leq c2^{mk}$ �ƂȂ�܂��B
$m = 2^t$ �Ƃ���� $t! \leq c2^{2^{t}k}$ �ƂȂ�܂��B
���̎���$k \geq 1$ �ɑ΂��Đ�������̂ŁA$\lg\lg n!$ �͑������Ɍ��肳��܂��B

## A3.2-5
$\lg^* n = x$�@�Ƃ��� $\lg^* 2^n = 1+x$ �Ȃ̂� $\lg^* 2^n = 1 + \lg^* n$ 


```math
\begin{eqnarray}
\lim_{n \to \infty} \frac{\lg \lg^* n}{\lg^* \lg n} 
&=& \lim_{m \to \infty} \frac{\lg \lg^* 2^m}{\lg^* \lg 2^m} 
= \lim_{m \to \infty} \frac{\lg (1+\lg^* m)}{\lg^* m} \\
&=& \lim_{x \to \infty} \frac{\lg (1+x)}{x} = 0
\end{eqnarray}
```

����� $\lg^* \lg n$ ���Q�ߓI�ɑ傫���B

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

A3.2-6���

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

�����

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
����


# 3-1

## a

$0 \leq p(n) \leq c n^k$�������܂��B

$c> \sum_{i=0}^d a_i$�ɂȂ�悤�ɑI�ׂ΁A$k \geq d$���㎮�𖞂����̂ŁA$p(n) = O(n^k)$���������܂�

## b

$0 \leq c p(n) \leq n^k$�������܂��B

$1/c < a_k$�ɂȂ�悤�ɑI�ׂ΁A$k \leq d$���㎮�𖞂����̂ŁA$p(n) = \Omega(n^k)$���������܂�

## c

$k = d$�Ȃ��a��b�̗����𖞂����̂�$p(n) = \Theta(n^k)$���������܂�

## d


```math
\begin{eqnarray}
\lim_{n \to \infty} \frac{p(n)}{n^k}
&=& \lim_{n \to \infty} \sum_{i=0}^d a_i n^{i-k} = 0
\end{eqnarray}
```

$k<d$�Ȃ̂�$i-k$�͏�ɕ��Ȃ̂ŋɌ������ƑS�Ă̍���0�Ɏ������܂��B
�����$p(n)= o(n^k)$���������܂��B

## e

```math
\begin{eqnarray}
\lim_{n \to \infty} \frac{n^k}{p(n)}
&=& \lim_{n \to \infty} \sum_{i=0}^d \frac{1}{a_i} n^{k-i} = 0
\end{eqnarray}
```


$k>d$�Ȃ̂�$k-i$�͏�ɕ��Ȃ̂ŋɌ������ƑS�Ă̍���0�Ɏ������܂��B
�����$p(n)= \omega(n^k)$���������܂��B

# 3-2

## a

```math
\begin{eqnarray}
\lim_{n \to \infty} \frac{\lg^k n}{n^{\epsilon}} = 0
\end{eqnarray}
```

����������̂ŁA$\lg^k n = O(n^{\epsilon})$ ���� $\lg^k n = o(n^{\epsilon})$�ł��B

$\lg^k n = o(n^{\epsilon})$����������̂ŁA$\lg^k n \neq \Omega(n^{\epsilon})$ ���� $\lg^k n \neq \omega(n^{\epsilon})$�ł���A$\lg^k n \neq \Theta(n^{\epsilon})$�ł��B

## b

```math
\begin{eqnarray}
\lim_{n \to \infty} \frac{n^k}{c^n} = 0
\end{eqnarray}
```

����������̂ŁA$n^k = O(c^n)$ ���� $n^k = o(c^n)$�ł��B

$n^k = o(c^n)$����������̂�$n^k \neq \Omega(c^n)$ ���� $n^k \neq \omega(c^n)$�ł���A$n^k \neq \Theta(c^n)$�ł��B

## c

$\sin n$�̒l���$-1 \leq \sin n \leq 1$�ł���A$n^{1/2}$�̏搔��1�����ł��B

```math
\begin{eqnarray}
\lim_{n \to \infty} \frac{n^{1/2}}{n^{\sin n}}
\end{eqnarray}
```

�͗L���l�Ɏ��������A0�ɂ�$\infty$�ɂ��������܂���B
����đS�Ă��������܂���B

## d

```math
\begin{eqnarray}
\lim_{n \to \infty} \frac{2^n}{2^{n/2}} = \lim_{n \to \infty} 2^{n/2} = \infty
\end{eqnarray}
```


����������̂ŁA$2^n = \Omega(2^{n/2})$ ���� $2^n = \omega(c2^{n/2})$�ł��B

$2^n = \omega(2^{n/2})$����������̂�$2^n \neq O(2^{n/2})$ ���� $2^n \neq o(2^{n/2})$�ł���A$2^n \neq \Theta(2^{n/2})$�ł��B

## e

$n^{\lg c} = c^{\lg n}$�Ȃ̂�

```math
\begin{eqnarray}
\lim_{n \to \infty} \frac{n^{\lg c}}{c^{\lg n}} = \lim_{n \to \infty} 1 = 1
\end{eqnarray}
```

���̎��͗L���l�ɂ͎������܂����A0��$\infty$�ɂ͎������Ȃ��̂ŁA$n^{\lg c} = O(c^{\lg n})$ ���� $n^{\lg c} = \Omega(c^{\lg n})$ ���� $n^{\lg c} = \Theta(c^{\lg n})$ �ł����A $n^{\lg c} \neq o(c^{\lg n})$ �� $n^{\lg c} \neq \omega(c^{\lg n})$ �ł��B

## f


```math
\begin{eqnarray}
\lim_{n \to \infty} \frac{\lg n!}{\lg n^n} 
&=& \lim_{n \to \infty} \frac{\lg \sqrt{2 \pi n} + \lg n^n - \lg e^n + \lg e^{\alpha}}{\lg n^n} \\
&=& \lim_{n \to \infty} \frac{\frac{1}{2} \lg (2 \pi n) + n \lg n - n \lg e + \lg e^{\alpha}}{n \lg n} \\
&=& \lim_{n \to \infty} \frac{\frac{1}{2} \lg n + \lg n}{\lg n} = \frac{3}{2}
\end{eqnarray}
```

�r���̎��W�J�ł́A���ꂪ�����ɔ��U����0�Ɏ������鍀�͏Ȃ��܂����B
���̎��͗L���l�ɂ͎������܂����A0��$\infty$�ɂ͎������Ȃ��̂ŁA$\lg n!= O(\lg n^n)$ ���� $\lg n! = \Omega(\lg n^n)$ ���� $\lg n! = \Theta(\lg n^n)$ �ł����A $\lg n! \neq o(\lg n^n)$ �� $\lg n! \neq \omega(\lg n^n)$ �ł��B

# 3-3
����

# 3-4
�ے肷��ꍇ�͔����1�����邾���ŏؖ��ɂȂ�܂��B

## a

$an+b = O(n^2)$ �ɂȂ邱�Ƃ��{�����ɉ������Ă��܂����B
�ł��� $n^2 = O(an+b) = O(n)$ �ɂ͂Ȃ�܂���B
����Ė�蕶�̊֌W�͐������܂���B 

## b

$f(n) = n^2$ , $g(n) = n$ �Ƃ��ā@$min(f(n),g(n)) = g(n) =n$ �ɂȂ�܂��B
$\Theta(n^2+n) \neq \Theta(n)$ �Ȃ̂Ŗ�蕶�̊֌W�͐������܂���B



# 3-5
����

# 3-6
����

# ���̋L��
- [�l�I�׋������u�A���S���Y���E�C���g���_�N�V�����v��Rust�Ŏ�������F2��](https://qiita.com/Kosuke_Matsui/items/eea26e88cd261173a292)
- [�l�I�׋������u�A���S���Y���E�C���g���_�N�V�����v��Rust�Ŏ�������F3��](https://qiita.com/Kosuke_Matsui/private/b424d8eb30dd16455b12)
- [�l�I�׋������u�A���S���Y���E�C���g���_�N�V�����v��Rust�Ŏ�������F4��](https://qiita.com/Kosuke_Matsui/private/3b93d6b8a7ee5e90a2b5)


