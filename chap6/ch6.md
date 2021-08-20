---
title: �l�I�׋������u�A���S���Y���E�C���g���_�N�V�����v��Rust�Ŏ�������F6��
tags: �A���S���Y���C���g���_�N�V����
author: Kosuke_Matsui
slide: false
---
# �L���̊T�v
�A���S���Y���C���g���_�N�V���� ��3�� �����ŁF���E�W��MIT���ȏ��̃A���S���Y����Rust�Ŏ������Ȃ���׋������̂ŁA���̌��ʂ𐮗����܂����B
�܂Ƃ߂����e�ɂ͊m���Ɍ�肪����Ǝv����̂ŁA�Q�Ƃɂ������ẮA���炩���߂��������������B

�l�I�ȕ׋������Ȃ̂ő��l�l�Ɍ��₷�����̂ɂȂ��Ă��܂���B
�܂��A�{�̒��쌠����邽�߂ɁA�{���Q�Ƃ��Ȃ���łȂ��Ɨ����ł��Ȃ��悤�ɏ����Ă��܂��B

�����6�͂ɂ��ĕ׋����܂����B

RUST�R�[�h�̓���͈ȉ��̃T�C�g�Ŋm�F�ł��܂��B
https://play.rust-lang.org/

# 6.1�@�q�[�v

�i2���؁j�q�[�v�f�[�^�\���́A�ȉ��̐����������܂��B

- �ߓ_����m�[�h��2�����򂵂Ă����؍\���ł���
    - max�q�[�v�́A�����ő�l�ɂȂ�
        - �e�m�[�h�͎q�m�[�h���l���傫��
    - min�q�[�v�́A�����ŏ��l�ɂȂ�
        - �e�m�[�h�͎q�m�[�h���l��������
- ����$h$�̖؂̍ŉ��i�́A�q�[�v�̗v�f����$2^{h+1}-1$�����̏ꍇ�A������m�[�h�𖄂߂Ă����A1��������0�̗t���������Ȃ��ߓ_���ł���


## A6.1-1

����$h$�̃q�[�v�����v�f����

����0�̏ꍇ�F$1$
����1�̏ꍇ�F$1+2$
����2�̏ꍇ�F$1+2+2^2$
����3�̏ꍇ�F$1+2+2^2+2^3$
����h-1�̏ꍇ�F$\sum_{i=0}^{h-1}2^i$

����$h-1$�܂ł̗v�f�����Ɍv�Z���Ă����܂��B
$x = \sum_{i=0}^{h-1}2^i$�Ƃ���

```math
\begin{eqnarray}
x &=& 2x -x \\
&=& \sum_{i=1}^{h}2^i - \sum_{i=0}^{h-1}2^i \\
&=& 2^h -1
\end{eqnarray}
```

�ł��B

�ő�̏ꍇ�͍���$h$�Ԗڂ̗v�f����$2^h$�Ȃ̂ŁA$2^h-1+2^h = 2^{h+1}-1$�ɂȂ�܂��B

�ŏ��̏ꍇ�͍���$h$�Ԗڂ̗v�f����$1$�Ȃ̂ŁA$2^h-1+1 = 2^h$�ɂȂ�܂��B


## A6.1-2
�v�f����$n=2^m+k$�Ƃ��܂��B$m$�͔C�ӂ̒l�ŁA$k$��$0 \leq k \leq 2^m-1$�̒l�Ƃ��܂��B
�v�f����$2^m \leq n \leq 2^{m+1}-1$�Ƃ��āA�ŏ��̃q�[�v�v�f��$2^m$�ł��A�K������$m$�������܂��B
�����$2^m \leq n$���A$m \leq \lg n$�ƂȂ�܂��B

�܂��A�ő�̃q�[�v�v�f��$2^{m+1}-1$�ł�����$m+1$�̗v�f���͖������܂���B
�����$n < 2^{m+1}$�Ȃ̂ŁA$\lg n -1 < m$�ƂȂ�܂��B

����č�����$\lg n-1 < m \leq \lg n$�𖞂����̂�$\lfloor \lg n \rfloor$�ƂȂ�܂��B

## A6.1-3
�ő�v�f�����ɂ���̂͒�`���玩�����Ǝv���̂ł����A�����ؖ�����΂����̂�������Ȃ��̂ŉ𓚕ۗ��ł��B

## A6.1-4
max�q�[�v�ɂ����čŏ��v�f�͗t�ɒu����܂�

## A6.1-5
���\�[�g�z��͍ŏ��v�f�����ɗ���̂�min�q�[�v�ł�

## A6.1-6
23 �� 17, 14
17 �� 6, 13
14 �� 10, 1
6 �� 5, 7�F�e�m�[�h���q�m�[�h���傫���Ȃ��Ă���
13 �� 12

�e�m�[�h���q�m�[�h���傫���Ȃ��Ă���̂�max�q�[�v�ł͂���܂���B

## A6.1-7
����0�F1
����1�Ffrom 2 to 3
����2�Ffrom 4 to 7
����3�Ffrom 8 to 15
����h-1�Ffrom $2^{h-1}$ to $2^h-1$
����h�Ffrom $2^h$ to $2^{h+1}-1$

�v�f����$n=2^h$�̏ꍇ�́A�t�͍���h-1��2�Ԗڂ̗v�f����Ō�̗v�f�܂łƍ���h�̍ŏ��̗v�f�ł��B
�܂�$2^{h-1}+1$ ���� $2^h-1$��$2^h$�ł��B�����$n/2+1, n/2+2, \cdots , n$�ł��B

�v�f����$n=2^h+1$�̏ꍇ�́A�t�͍���h-1��2�Ԗڂ̗v�f����Ō�̗v�f�܂łƍ���h�̍ŏ��̗v�f�ł��B
�܂�$2^{h-1}+1$ ���� $2^h-1$��$2^h$ ���� $2^h+1$�ł��B�����$\lfloor n/2 \rfloor +1, \lfloor n/2 \rfloor+2, \cdots , n$�ł��B

�v�f����$n=2^h+2$�̏ꍇ�́A�t�͍���h-1��3�Ԗڂ̗v�f����Ō�̗v�f�܂łƍ���h�̍ŏ��̗v�f�ł��B
�܂�$2^{h-1}+2$ ���� $2^h-1$��$2^h$ ���� $2^h+2$�ł��B�����$n/2+1, n/2+2, \cdots , n$�ł��B

�v�f����$n=2^h+3$�̏ꍇ�́A�t�͍���h-1��2�Ԗڂ̗v�f����Ō�̗v�f�܂łƍ���h�̍ŏ��̗v�f�ł��B
�܂�$2^{h-1}+2$ ���� $2^h-1$��$2^h$ ���� $2^h+3$�ł��B�����$\lfloor n/2 \rfloor +1, \lfloor n/2 \rfloor+2, \cdots , n$�ł��B

�ȍ~�����l�ɂȂ�A�v�f�����ő��$n=2^{h+1}-1$�̏ꍇ�́A�t�͍���h��1�Ԗڂ̗v�f����Ō�̗v�f�܂łł��B
�܂�$2^h$ ���� $2^{h+1}-1$�ł��B�����$\lfloor n/2 \rfloor +1, \lfloor n/2 \rfloor+2, \cdots , n$�ł��B

����đS�Ă̗v�f��$n$�ɑ΂��āA�t��$\lfloor n/2 \rfloor +1, \lfloor n/2 \rfloor+2, \cdots , n$�ɂȂ�܂��B

## 6.2 �q�[�v�����̈ێ�
max�q�[�v�����𖞂����悤�ɔz�����בւ���A���S���Y����Rust�Ŏ������܂��B

�{���ƈقȂ�z���index��0����J�n����̂�left�֐���right�֐��̌v�Z��1�����Z���Ă��܂��B
�܂�{���ł͐e�m�[�h����q�m�[�h�ւ�index�̑������A1->1*2��1*2+1�ɂȂ��Ă���̂ɑ΂��āA0->0*2+1��0*2+2�ɏC�����܂��B

�{���̗�ł�4��index��2�ł����Aindex��0����J�n����̂Ȃ��1�ɂȂ�܂��B

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

���̍ċA�A���S���Y���̌v�Z�ʂ�


```math
\begin{eqnarray}
T(n) \leq T(2n/3) + \Theta(1)
\end{eqnarray}
```

�ɂȂ�܂��B
$A[i]$���E�ƍ��̂ǂ���ɕ��򂷂邩�̌v�Z�ʂ�$\Theta(1)$�ł��B
�����čċA�Ăяo���́A�ň��̏ꍇ�A�ł����̃��x�������܂��Ă���̂�$2n/3$�̗v�f�ɂ��Ď��s����܂��B

�Ⴆ�Ηv�f$A[i]$����ŉ��i�܂ł̍�����$h$�Ƃ��āA����$h-1$�܂ł̗v�f����$2^h-1$�ł��B�v�f$A[i]$�������������̗v�f��$2^{h-1}-1$�ł��B
����$h$�Ԗڂ��������܂��Ă���ꍇ�̗v�f����$2^{h-1}$�ł��B
�S�v�f����$n=3*2^{h-1}-1$�ł���A�ċA�Ăяo���ɂ�$2*2^{h-1}-1$�̗v�f���g�p�����̂ŁA�����悻$2n/3$�ƂȂ�܂��B

## A6.2-1
�ȉ��̏��Ԃŗv�f3���ړ����܂��B

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

���s���Ԃ�max�q�[�v�Ɠ����ł��B

## A6.2-3

�v�f$A[i]$�����E�����̎q���傫����΁A�����̎q�v�f�ɂ��Ă̏�������ňȉ������s����

```rust
    } else {
        largest = i;
    }    
```


�E���̎q�v�f�ɂ��Ă̏������򂪓��Ă͂܂炸�A�ȉ��̍s�ɔ�т܂��B


```rust
    if largest != i {
```

$largest = i$�Ȃ̂ŁA�����Ŋ֐����I�����܂��B


## A6.2-4

$i$���q�[�v�T�C�Y�̔������傫���ꍇ�A�{���Ƃ͈قȂ�$i$��0����J�n�Ȃ��$i > A.heap-size/2 -1$�̏ꍇ�Aleft�֐���right�֐����q�[�v�T�C�Y�𒴂���l��Ԃ��܂��B
����ƑO��Ɠ��l��


```rust
    } else {
        largest = i;
    }    
```


�֕����A�ȉ��̍s�ɔ�т܂��B


```rust
    if largest != i {
```

$largest = i$�Ȃ̂ŁA�����Ŋ֐����I�����܂��B

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

�ň��̏ꍇ�A������t�܂ō���$h=\lg n$��̌v�Z�����s����̂�$\Omega(\lg n)$

# 6.3�@�q�[�v�̍\�z

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


# ���̋L��
- [�l�I�׋������u�A���S���Y���E�C���g���_�N�V�����v��Rust�Ŏ�������F2��](https://qiita.com/Kosuke_Matsui/items/eea26e88cd261173a292)
- [�l�I�׋������u�A���S���Y���E�C���g���_�N�V�����v��Rust�Ŏ�������F3��](https://qiita.com/Kosuke_Matsui/private/b424d8eb30dd16455b12)
- [�l�I�׋������u�A���S���Y���E�C���g���_�N�V�����v��Rust�Ŏ�������F4��](https://qiita.com/Kosuke_Matsui/private/3b93d6b8a7ee5e90a2b5)
- [�l�I�׋������u�A���S���Y���E�C���g���_�N�V�����v��Rust�Ŏ�������F6��](https://qiita.com/Kosuke_Matsui/private/8d1586a463a78081d533)
