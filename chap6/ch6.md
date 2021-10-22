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
����v�f�����Ȃ�΁A���̎q�͑S�č���菬���Ȓl�ɂȂ�̂ŁA�ő�v�f�͍��ɂȂ�܂��B

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

$T(n) = T(2n/3)+\Theta(1)$�ɂ���

$a=1$�A$b=2/3$�A$f(n)=\Theta(1)$�Ȃ̂�

�}�X�^�[�藝���A$n^{\log_b a + \epsilon} = n^{\log_{2/3} 1 + \epsilon}$�ɂ���$\epsilon = 0$�ɑI�ׂ�$n^0=1=\Theta(1)$�ɂȂ�̂ŁAmax_heap�̌v�Z�ʂ�$T(n)=\Theta(\lg n)$�ɂȂ�܂��B


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
�C�ӂ̔z���Max�q�[�v�ɕ��בւ��܂��B
���܂ł̃A���S���Y����`build_max_heap`��ǉ����Ă��܂��B
���̃A���S���Y���́A�q�����v�f�̑S�Ăɂ��Ďq�Ɛe�̊֌W�𐳂�������ł��邩���m�F���ĕ��בւ��܂��B
��̓I�ɂ́A�Ώۂ̗v�f�̓��̍Ō�̗v�f����ŏ��̗v�f�̏��Ԃ�`max_heap`��K�p���܂��B

�q�[�v�̗t�i�q�������Ȃ��v�f�j��$\lfloor n/2 \rfloor +1$�Ԗڂ̗v�f�A$\lfloor n/2 \rfloor+2$�Ԗڂ̗v�f $\cdots$ $n$�Ԗڂ̗v�f�ł��邱�Ƃ��uA6.1-7�v�ŏؖ����܂����B
����Ďq�����v�f��$1$�Ԗڂ̗v�f�A$2$�Ԗڂ̗v�f $\cdots$ $\lfloor n/2 \rfloor$�Ԗڂ̗v�f�ł��B
�܂�$\lfloor n/2 \rfloor$�Ԗڂ̗v�f����$1$�Ԗڂ̗v�f�ɂ���`max_heap`��K�p����΂����킯�ł��B

�v���O�����̃R�[�h��ł�$1$�Ԗڂ̗v�f�̓Y������0�ł���A$\lfloor n/2 \rfloor$�Ԗڂ̗v�f�̓Y������$\lfloor n/2 \rfloor -1$�Ȃ̂Œ��ӂ��������B
�X��RUST�ł͍~����for��`for i in (0..5).rev()`��4,3,2,1,0�̂悤�Ɏw�肵���ő�l����1���������l����J�n����̂ŁA1�𑫂��̂�Y��Ȃ��ŉ������B
�i�v���O������ł͊J�n�̓Y�����ɂ��āA1�������Ă���1�𑫂��Ƃ�����������邱�ƂɂȂ�܂��B�j

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

���̃A���S���Y���̌v�Z���Ԃ��l���܂��B
�܂�����$h$�̃q�[�v�����S�Ă̐ߓ_��$\lceil n/2^{h+1} \rceil$�ł��B�i�ؖ���A6.3-3���Q�Ƃ��Ă��������B�j
�����ėv�f��$n$�̍�����$\lfloor \lg n \rfloor$�ł��B
`build_max_heap`�͍���0���獂��$\lfloor \lg n \rfloor$�܂ł̊e�w�̌Ăяo�����_�ɂ�����S�Ă̐ߓ_�ɂ���`max_heap`��K�p���܂��B
`max_heap`�̌v�Z���Ԃ́A����$h$�̏ꍇ$O(h)$�ł��B
����đ��R�X�g�͈ȉ��ɂȂ�܂��B

```math
\begin{eqnarray}
\sum_{h=0}^{\lfloor \lg n \rfloor} \Biggl[ \lceil \frac{n}{2^{h+1}} \rceil \Biggl] O(h) = O \Biggl( n \sum_{h=0}^{\lfloor \lg n \rfloor} \frac{h}{2^h} \Biggl)
\end{eqnarray}
```

�{���̌���(A.8)���

```math
\begin{eqnarray}
O \Biggl( n \sum_{h=0}^{\lfloor \lg n \rfloor} \frac{h}{2^h} \Biggl) 
&\leq& O \Biggl( n \sum_{h=0}^{\infty} \frac{h}{2^h} \Biggl) \\
&=& O(2n) = O(n)
\end{eqnarray}
```

�Ȃ̂ŁA�A���S���Y���̌v�Z���Ԃ̏�E��$O(n)$�Ƃ������`�ɂȂ邱�Ƃ�������܂��B

���Ȃ݂�`min_heap`��p����΁A�z���min�q�[�v�ɕ��בւ���֐����쐬�ł��܂��B

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

��L�̃R�[�h�����s����ƈȉ��𓾂܂��B

```Console
v=[5, 3, 17, 10, 84, 19, 6, 22, 9]
v=[5, 3, 17, 10, 84, 19, 6, 22, 9], i=3
v=[5, 3, 17, 22, 84, 19, 6, 10, 9], i=2
v=[5, 3, 19, 22, 84, 17, 6, 10, 9], i=1
v=[5, 84, 19, 22, 3, 17, 6, 10, 9], i=0
sorted v=[84, 22, 19, 10, 3, 17, 6, 5, 9]
```

## A6.3-2
�Ōォ��ŏ��̏��Ԃɍs��Ȃ������ꍇ�A�����̐󂢕����܂Ő������Ă���Max�q�[�v�̏��Ԃ��A�ǂ����̍����̌����ɂ��MAx�q�[�v�𖞂����Ȃ����ԂɂȂ��Ă��܂��܂��B

## A6.3-3

1) $h=0$�̏ꍇ$\lceil n/2 \rceil$���������邱�Ƃ̏ؖ�
�ŉ��w�̗v�f���S��1��̑w�Ɛe�q�֌W�ɂ���ꍇ�ɐߓ_�̐����ő�ɂȂ�܂��B

�v�f���������̏ꍇ�A��̏����𖞂����ƍŉ��w��$n/2$�̗v�f�ɂȂ�̂ŁA�ߓ_����$n-n/2 = n/2 = \lceil n/2 \rceil$�ł��B

�v�f������̏ꍇ�A��̏����𖞂����ƍŉ��w�̍Ō�̗v�f�����́A�e�ɑ΂��Ďq���P�����Ȃ��ߓ_�ɂȂ�܂��B
�܂�v�f����$n=2m+1$�Ƃ��āA�ߓ_�̐��͗v�f����$n+1=2m+2$�̏ꍇ�Ɠ������Ȃ�܂��B
����Đߓ_����$n+1-(n+1)/2 = (n+1)/2 = \lceil n/2 \rceil$�ł��B

�ȏ���A�����A��̗����ɂ����āA$h=0$�̏ꍇ��$\lceil n/2 \rceil$���������܂��B

2) $h-1$�̏ꍇ�ɗ^������������Ƃ����ꍇ��$h$�ł��������邱�Ƃ̏ؖ�

�ؖ����@���v�����Ȃ��̂ŉ𓚕ۗ��ł��B
�i[https://ita.skanev.com/06/03/03.html](https://ita.skanev.com/06/03/03.html)�ɉ񓚂�����܂������A�ǂ�ł������ł��܂���ł����B
�t�̗v�f$\lceil n/2 \rceil$�������ĐV�����c���[$n-\lceil n/2 \rceil = \lfloor n/2 \rfloor$�����Ƃ����̂��A�ǂ����������Ȃ̂�������܂���ł����B�j

# 6.4�@�q�[�v�\�[�g�A���S���Y��
�C�ӂ̔z����܂���max�q�[�v�ŕ��בւ��܂��B
����ƍŏ��̗v�f�͍��ł���A�z��v�f�̍ő�l�ɂȂ�܂��B
���̍���2�̎q�́A���ɑ����v�f���琬�镔���؂̍��Ƃ݂Ȃ��܂��B�܂肻�́u�q�v�͕����؂̍��ɂȂ��Ă���̂ŁA�����̕����؂̒��ł͍ő�l�ł����A�ׂ̕����؂Ɣ�ׂčő�l�ɂȂ��Ă���Ƃ͌���܂���B
�����ō����������z��ɂ��āA���́u�q�v��max�q�[�v���C������ƁA�Ăєz��̍ŏ��̗v�f���ő�l�ɂȂ�܂��B

�ȍ~�͔z��̗v�f����2�ɂȂ�܂œ������Ƃ��J��Ԃ��܂��B

��L�̉ߒ��Ŏ�菜�����v�f�����ɕ��ׂ�΁A�ő�l����ŏ��l�ւƕ��ёւ���ꂽ�z�񂪓����܂��B

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

���̃A���S���Y���̌v�Z�ʂ́Abuild_max_heap��$O(n)$�Amax_heap��$\Theta(\lg k)$�ł��B
$k$�͌Ăяo���x�ɃT�C�Y��$k=n-1$����$k=1$�܂Ō����Ă����̂ŁA

```math
\begin{eqnarray}
\sum_{i=1}^{n-1} \lg k
&=& \lg (n-1)! \\
&=& \lg (n-1)(n-2) \cdots (n-(n-1)) \\
&=& \lg (n^n + \cdots) \\
&=& \Theta(\lg n^n) = \Theta(n \lg n)
\end{eqnarray}
```

����Čv�Z�ʂ�$\Theta(n \lg n)$�ɂȂ�܂��B
build_max_heap�̌v�Z��$O(n)$��$\Theta(n \lg n)$�ɔ�ׂ�Ə������̂Ŗ������܂��B
�}�X�^�[�藝���瓱�����̂́A�Q�ߓI�Ȍ��E$\Theta$�Ȃ̂ɁA�{�ł͑Q�ߓI��E$O$�ɂȂ��Ă��闝�R���悭������܂���ł����B
$\Theta$�𖞂����Ă����$O$�������ɖ������Ă��邩��Ɖ��߂��܂����B

## A6.4-1��A6.4-2
��L�̃R�[�h��K�p����ƈȉ��𓾂��܂��B
�����z��$A[0 \cdots i-1]$�܂ł�max�q�[�v�ɂȂ��Ă���A$A[i \cdots n-1]$�̓\�[�g����Ă��邱�Ƃ�������܂��B

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
�z�񂪍~���ł��낤�Ə����ł��낤�ƃq�[�v�ɂ͂Ȃ��Ă��Ȃ��̂�build_max_heap�ɂ��v�Z�ʂ͕ς��Ȃ��Ǝv���܂��B
�q�[�v���ꂽ�z��ɑ΂���max_heap�K�p�ȍ~�̃A���S���Y�����A�z�񂪍~���ł��낤�Ə����ł��낤�ƕς��Ȃ��̂ŁA�v�Z�ʂ͓���$\Theta(n \lg n)$�ɂȂ�܂��B

## A6.4-4

Max Heap�̍ň����s���Ԃ�A6.2-4���$\Omega(\lg n)$�ł��B
�q�[�v�\�[�g�A���S���Y���́A�T�C�Y�����������Ȃ���Max Heap��K�p�����Ă����̂ŁA���l�̌v�Z�Ōv�Z�ʂ�$\Omega(n \lg n)$�ɂȂ�܂��B

## A6.4-5
���ϓI�ɂ́A�q�[�v�\�[�g�A���S���Y���́A�z��̗v�f���S�ĈقȂ�΁A�ŗǂł��ň��ł�Max Heap�̌v�Z�ʂ͕ς�炸�A�ŗǎ��s���Ԃ�$\Omega(n \lg n)$�ɂȂ�̂��Ǝv���܂��B

����𐳊m�ɏؖ�������@���v�����Ȃ��̂ŁA�𓚕ۗ��ł��B

���Ȃ݂Ƀl�b�g�ɂ�[��](https://ita.skanev.com/06/04/05.html)�������Ă��܂������A�u�ŗǂ̃P�[�X��$2^{k-2}-1$�̃m�[�h���o�C�i���[�c���[�ɔz�u����Ă��鎞�v�Ƃ����Ӗ���������܂���ł����B

# 6.5 �D��x�t���L���[

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

����̑}���ŃG���[�ɂȂ�̂��������ׂł��B

## A6.5-5




# ���̋L��
- [�l�I�׋������u�A���S���Y���E�C���g���_�N�V�����v��Rust�Ŏ�������F2��](https://qiita.com/Kosuke_Matsui/items/eea26e88cd261173a292)
- [�l�I�׋������u�A���S���Y���E�C���g���_�N�V�����v��Rust�Ŏ�������F3��](https://qiita.com/Kosuke_Matsui/private/b424d8eb30dd16455b12)
- [�l�I�׋������u�A���S���Y���E�C���g���_�N�V�����v��Rust�Ŏ�������F4��](https://qiita.com/Kosuke_Matsui/private/3b93d6b8a7ee5e90a2b5)
- [�l�I�׋������u�A���S���Y���E�C���g���_�N�V�����v��Rust�Ŏ�������F6��](https://qiita.com/Kosuke_Matsui/private/8d1586a463a78081d533)
