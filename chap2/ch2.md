---
title: �l�I�׋������u�A���S���Y���E�C���g���_�N�V�����v��Rust�Ŏ�������F2��
tags: Rust �A���S���Y��
author: Kosuke_Matsui
slide: false
---
# �L���̊T�v

�A���S���Y���C���g���_�N�V���� ��3�� �����ŁF���E�W��MIT���ȏ��̃A���S���Y����Rust�Ŏ������Ȃ���׋����܂��B

�l�I�ȕ׋������Ȃ̂ő��l�l�Ɍ��₷�����̂ɂȂ��Ă��܂���B
�܂��A�{�̒��쌠����邽�߂ɁA�{���Q�Ƃ��Ȃ���łȂ��Ɨ����ł��Ȃ��悤�ɏ����Ă��܂��B

�����2�͂ɂ��ĕ׋����܂����B

# 2.1 �}���\�[�g

�}���\�[�g�͔z��̐��������Ԃɕ��ёւ��邽�߂̃A���S���Y���ł��B

�Ώۂ̔z��̒l���P�O�̔z��̒l�Ɣ�r���āA�Ώۂ̔z����l���傫����Δz��̏��Ԃ��������A�Ώۂ̔z����l����������΂����ŏI�����܂��B
���̑�����A�z���2�Ԗڂ���n�Ԗڂ܂ł����Ɏ��s���܂��B
�K��2�Ԗڂ�����s���܂��B3�Ԗڈȍ~������s����Ə��Ԃɕ��т܂���B

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

https://play.rust-lang.org/ �ɂ����Ď��s����ƁA�ȉ��̌��ʂ𓾂܂��B

```console
[4, 2, 6, 1, 3, 5]
[1, 2, 3, 4, 5, 6]
```

�܂��A�x�N�g����p����Έȉ��̂悤�Ɏ����ł��܂��B

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
�Ώۂ̔z��̒l���P��̔z��̒l�Ɣ�r���āA�Ώۂ̔z����l���傫����Δz��̏��Ԃ��������A�Ώۂ̔z����l����������΂����ŏI�����܂��B
���̑�����A�z���n-1�Ԗڂ���1�Ԗڂ܂ł����Ɏ��s���܂��B
�K��n-1�Ԗڂ�����s���܂��B

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

for���ŋt��`rev()`���g�p���Ĕz��̌�납��O�����Ɍv�Z���Ă��܂��B

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
    println!("{}��{}�Ԗڂ̒l", v, search(v));
    
    let v = 100;
    println!("{}��{}�Ԗڂ̒l", v, search(v));
}
```

## A2.1-4
���̖��́A�Ӗ���������ɂ����ł����A�v��2�i���̉��Z��ɂ��Ă̖��ł��B

�Ⴆ�΁A0b10011+0b10110�Ƃ���2�i���̘a���A�Q�̐����̔z��AA[5] = [1, 1, 0, 0, 1]��B[5]=[0, 1, 1, 0, 1]�ŕ\��������ŁA�a�����߂�������̂ł��B

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
�v�Z���ʂ��ȉ��Ɏ����܂��B
A��B��n-1�Ԗړ��m�̘a�����A�J��肪����΁An�Ԗړ��m�̘a�ɌJ�����Z���Ă��邱�Ƃ�������܂��B
0b10011+0b10110=0b101001��\���Ă��܂��B

```console
[1, 1, 0, 0, 1]
[0, 1, 1, 0, 1]
[1, 0, 0, 1, 0, 1]
```

�{�̒ʂ�ɂ���΁A�C�ӂ�n�v�f�z������ł���悤�ɂ��Ȃ��Ƃ����܂��񂪁A�A���S���Y���̖{���Ƃ͊֌W�Ȃ��̂ŁA������ŗp�ӂ����z��������܂����B

# 2.2 �A���S���Y���̉��

## �}���\�[�g�̉��

for���́A���������s���������v�Z���܂��B
`for j in 1..a.len()`��1����n-1�܂�n-1��̔���ɉ����A���[�v���甲����ۂ�1��̔�����s���̂ō��vn��B

for������1�񂵂����s���Ȃ��v�Z�̌v�Z�񐔂�n-1��B
for�����ł̔��莮�͊e���[�v���ɔ���񐔂��قȂ�A�e���[�v��$t_j$��ɂȂ�ꍇ�́A���v��$\sum_{j=1}^{n-1} t_j$��B
for�����ł̔��莮�ɂ����Ď��s�����v�Z�́A���[�v���甲����1��̔���񐔂������āA�v�Z�񐔁~$(t_j-1)$��ɂȂ�܂��B�}���\�[�g�ł͌v�Z�񐔂�1��Ȃ̂ō��v��$\sum_{j=1}^{n-1} t_j$��B

�ŗǂ̏����́A���߂���\�[�g����Ă���ꍇ�ł���Awhile�ȉ��̌v�Z��0�ɂȂ�܂��B

�ň��̏����́A���߂���t���Ƀ\�[�g����Ă���ꍇ�ł���Awhile�ȉ��̌v�Z��key�̓Y������菬���������̓Y�����̑S�Ăɂ��Čv�Z���Ȃ��Ƃ����Ȃ��̂ŁA$t_j=j$�ƂȂ�܂��B

## A2.2-1
�v�Z�ʂ̎����͍ő原���̍��݂̂��e������̂ŁA

```math
\begin{eqnarray}
n^3/1000 \in \Theta(n^3)
\end{eqnarray}
```

�ƂȂ�܂��B

## A2.2-2
�z���1�Ԗڂ̗v�f��2�Ԗڂ���n�Ԗڂ̗v�f�Ɣ�r���A�ŏ��̗v�f��1�Ԗڂ̗v�f���������܂��B
�z���2�Ԗڂ̗v�f��3�Ԗڂ���n�Ԗڂ̗v�f�Ɣ�r���A�ŏ��̗v�f��2�Ԗڂ̗v�f���������܂��B
...
�z���n-2�Ԗڂ̗v�f��n-1�Ԗڂ���n�Ԗڂ̗v�f�Ɣ�r���A�ŏ��̗v�f��n-2�Ԗڂ̗v�f���������܂��B
�z���n-1�Ԗڂ̗v�f��n�Ԗڂ̗v�f�Ɣ�r���A�ŏ��̗v�f��n-1�Ԗڂ̗v�f���������܂��B

for loop��n-1�Ԗڂ܂Ŏ��s����΁A�c���n�Ԗڂ̗v�f�͕K���ő�l�Ȃ̂ŁAn�S�Ă̗v�f�ɂ��Ď��s����K�v�͂���܂���B

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

�v�Z�ʂ́A

`for j in 0..a.len()-1`���A�v�f��n-1���loop�𔲂��锻���1���$n-1+1=n$��B
`let mut key = j;`��n-1��B
`let mut min = a[key];`��n-1��B
`for i in j+1..a.len()`���A�e���[�v���ɁA�v�f��n-j-1���loop�𔲂��锻���1���$n-j-1+1=n-j$ ��B���̑��a��$\sum_{j=0}^{n-1}(n-j)$
`if min >  a[i]`���e���[�v���ɁAn-j-1��B���̑��a��$\sum_{j=0}^{n-1}(n-j-1)$��B
`key = i;`���e���[�v���ɁAn-j-1��B���̑��a��$\sum_{j=0}^{n-1}(n-j-1)$��B
`min = a[i];`���e���[�v���ɁAn-j-1��B���̑��a��$\sum_{j=0}^{n-1}(n-j-1)$��B
`a[key] = a[j];`��n-1��B
`a[j] = min;`��n-1��B


�ŗǂ̏����́A���߂���\�[�g����Ă���ꍇ�ł���A`key = i;`��`min = a[i];`�̌v�Z��0�ɂȂ�܂��B

�ň��̏����́A���߂���t���Ƀ\�[�g����Ă���ꍇ�ł���A`key = i;`��`min = a[i];`�̌v�Z�𖈉񂵂Ȃ��Ƃ����܂���B

�ł����A�ŗǂ̏ꍇ���A�ň��̏ꍇ���A�K��`for i in j+1..a.len()`��`if min >  a[i]`�̌v�Z�����s����̂ŁA�ǂ���̏ꍇ�ł��v�Z�̎�����$\Theta(n^2)$�ɂȂ�܂��B

```math
\begin{eqnarray}
\sum_{j=0}^{n-1}(n-j-1) &=& \frac{((n-0-1)+(n-n+1-1))*n}{2} \\
&=& \frac{(n-1)*n}{2} 
\end{eqnarray}
```

���ϓI�ɍl����ƁA�}�����͔z��̕��ѕ��̉^���ǂ��Ɠr���Ń��[�v���甲���o�邱�Ƃ��ł��A�I�����͂ǂ�Ȕz��ł��S����r���Ȃ��Ƃ����Ȃ��̂Ń��[�v���Ō�܂Ŕ����o���Ȃ��Ƃ����Ⴂ�ł��B


## A2.2-3

�}���\�[�g�̃A���S���Y���ɂ��āA�z��v�f�ɓ�������������ꍇ�̌v�Z�ʂ̈Ⴂ���r������ł��B
�z����ɓ�������������΁Akey�̐��������������Ɣ�r�������_�Ń��[�v���甲���āA`while i>0 && a[i-1]>key`�ȉ������s���Ȃ��ōςނ̂ŁA���ꂾ���v�Z�ʂ�����܂��B

�ŗǂ̏ꍇ�͔z��̗v�f���S�ē��������̏ꍇ�ŁA�ŏ�����\�[�g����Ă���ꍇ�Ɠ����v�Z�ʂɂȂ�܂��B
�ň��̏ꍇ�͔z��̗v�f���S�ĈقȂ�ꍇ�ł��B

�Ⴆ�΁A�z��[3,1,2,1,1,2,4,1,5,2]�������ꍇ���l���܂��B
�ŏ���1��key�ɂ��Ĕ�r���Ă��鎞�A`while i>0 && a[i-1]>key`�ȉ��̌v�Z��1����ȗ��ł��܂���B
����2��key�ɂ��Ĕ�r���Ă��鎞�A`while i>0 && a[i-1]>key`�ȉ��̌v�Z��1����ȗ��ł��܂���B
����1��key�ɂ��Ĕ�r���Ă��鎞�A3��ڂ̔�r�Ώۂ�1���������_�ŁA`while i>0 && a[i-1]>key`�ȉ��̌v�Z���ȗ��ł��܂��B
����1��key�ɂ��Ĕ�r���Ă��鎞�A3��ڂ̔�r�Ώۂ�1���������_�ŁA`while i>0 && a[i-1]>key`�ȉ��̌v�Z���ȗ��ł��܂��B

�܂�v�Z�ʂ́A�z����ɓ����v�f������邩�A�����Ă��̗v�f����r�ΏۂɂȂ邩�ǂ����ɉe������܂��B
���̂Q�̏����𖞂����m���́A��L�̗�̂悤�ɁA�Z���z�񂾂Ɗm���̍����傫���ł��傤���A�c��Ȓ����̃����_���Ȕz��Ȃ�Έ��̊m���i1/10�j�Ɏ�������Ǝv���܂��B
���ꂪ��蕶���Łu�T�����ׂ��v�f���z�񒆂̔C�ӂ̗v�f�Ɠ��m���ň�v����v�ƕ\������Ă��邱�Ƃł��B
���̊m����$p$�Ƃ��܂��B


key�̗v�f�Ƃ̔�r���A1��ڂň�v���ďI������m���A2��ڂň�v���ďI������m���A3��ڂň�v���ďI������m���A...�A(n-1)��ڂň�v���ďI������m���A�����čŌ�܂ň�v���Ȃ��ꍇ�́A�v�Z�񐔂̊��Ғl�̑��a�����߂܂��B

�܂�key�̗v�f��$j$�̗v�f�Ɣ�r���邱�ƂɂȂ�m�����l���܂��B�����Ȃ�m����$j-1$��͈�v�����A1���v�����Ƃ������ƂȂ̂ŁA��v���Ȃ��m����(j-1)��ƈ�v����m���̐ρA$(1-p)^{j-1}*p$ �ł��B

���̎��̌v�Z�񐔂̊��Ғl�́Aj��̌v�Z�����Ă���̂ŁA$j*(1-p)^{j-1}*p$�ł��B

�܂�key�̗v�f��n-1��̌v�Z��1�����v���Ȃ��m����$(1-p)^{n-1}$�ł��B�v�Z�񐔂̊��Ғl��$n-1$��̌v�Z�����Ă���̂ŁA$(n-1)*(1-p)^{n-1}$�ł��B

����Ċ��Ғl�̑��a�́A

```math
\begin{eqnarray}
�v�Z�񐔂̊��Ғl 
&=& (n-1)*(1-p)^{n-1} + \sum_{j=1}^{n-1} j*(1-p)^{j-1}*p \\
&=& (n-1)*(1-p)^{n-1} + \sum_{i=1}^{n-1} \sum_{j=i}^{n-1} (1-p)^{j-1}*p \\
&=& (n-1)*(1-p)^{n-1} + \sum_{i=1}^{n-1}  \frac{(1-p)^{i-1} - (1-p)^{n-1}}{1-(1-p)} *p \\
&=& (n-1)* (1-p)^{n-1} + \sum_{i=1}^{n-1}  \Bigl((1-p)^{i-1} - (1-p)^{n-1} \Bigl) \\
&=& (n-1)* (1-p)^{n-1} + \sum_{i=1}^{n-1}  (1-p)^{i-1} - (n-1)* (1-p)^{n-1} \\
&=& \frac{1 - (1-p)^{n-1}}{1-(1-p)} \\
&=& \frac{1}{p} - \frac{(1-p)^{n-1}}{p} \\
\end{eqnarray}
```

�ł��B�܂�A���ꂪ��蕶���́u���͗�̒��Œ��ׂ���v�f���̕��ρv�ł��B
���̊��Ғl�̌v�Z�ʂ̎�����$\Theta(1)$�ł��B

## A2.2-4

�}���\�[�g�̎��������΁A�\�Ȍ����������𑁂߂ɏI��点�A���[�v�𔲂��o���ďI���ł���A���S���Y�����ŗǎ��s���Ԃ����A���S���Y���ƌ��������ł��B

# 2.3 �A���S���Y���̐݌v

## �}�[�W�\�[�g

���Ƀ\�[�g�ς݂̔z��2����Ɖ��肵�܂��B
1�͒���$n_1 = q-p+1$�̔z��A[p],A[p+1]...,A[q]�Ƃ��܂��B
����1�͒���$n_2 = r-q$�̔z��A[q+1],A[q+2]...,A[r]�Ƃ��܂��B
����2�̔z����Ȃ��āA�ă\�[�g����A���S���Y�����l���܂��B

�܂��A�z��̃R�s�[���쐬���܂��B
�z��A[p],A[p+1]...,A[q]��z��L[0]...L[q-p]�ɃR�s�[���܂��B
�z��A[q+1],A[q+2]...,A[r]��z��R[0]...R[r-q-1]�ɃR�s�[���܂��B

���̎��_��3�̔z�񂪂ł��Ă��܂��BL��R�ƒ���$n_1+n_2$�̔z��A[p],A[p+1]...,A[r]�ł��B

���ɁA�z��L�Ɣz��R�̍Ō�ɗv�f��ǉ����AL[q-p+1]��L[r-q]�ɖ�����̒l�����ԕ���u���܂��B
���ۂ̃v���O�����ł́A������͂Ȃ��̂ŁA�ő�l���i�[���܂��B

�܂�A[p]�ɒ��ڂ��܂��BA[p]��L[0]�̒l�ɓ������ł��B
L[0]��R[0]�Ɣ�r���āA����L[0]��R[0]�ȉ��Ȃ�΁AA[p]��L[0]�������āA���̃T�C�N���ł�L[1]��R[0]���r���܂��B
����L[0]��R[0]���傫���Ȃ�΁AA[p]��R[0]�������āA���̃T�C�N���ł�L[0]��R[1]���r���܂��B

�l����������A���̃T�C�N���Ɉڂ�܂��BA[p+1]��L[1]�̒l�ɓ������ł��B
�O�̃T�C�N���̌��ʂɈˑ�����L[1]��R[0]��������L[0]��R[1]�Ɣ�r���āA����L[x]��R[y]�ȉ��Ȃ�΁AA[p+1]��L[x]�������āA���̃T�C�N���ł�L[x+1]��R[y]���r���܂��B�i������x��y��0 or 1�j
����L[x]��R[y]���傫���Ȃ�΁AA[p+1]��R[y]�������āA���̃T�C�N���ł�L[x]��R[y+1]���r���܂��B


�T�C�N���̓r���ŁA�K��L[x]��R[y]�̂ǂ��炩����ɔԕ��̖�����ɓ��B���܂��B
�Ⴆ�΁A���L[x]��������ɓ��B����ƁA�ȍ~�̃T�C�N���ł̔�r�͏��L[y]���������Ȃ�̂ŁAL[y+1],L[y+2]...��L�̔z��ԍ��������Z����āA�ԕ��ɓ��B����܂ő����܂��B

�ȉ��ɁA�\�[�g�ς݂̔z��[1, 10, 100]��[5, 11, 35]���\�[�g����ꍇ��Rust�Ŏ������Ă݂܂��B

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

�����܂ł̘b�ł́A�\�[�g�ς݂�2�̔z�񂪗^����ꂽ�ꍇ�ɁA����2�̔z����������ă\�[�g���܂����B
���́A�\�[�g�ς݂ł͂Ȃ��C�ӂ̔z����\�[�g���܂��B

�܂��z���2�ɕ������A���ꂼ��̔z��Ƀ}�[�W�\�[�g��K�p���ă\�[�g�ς݂ɂ����Ă���A����2�̔z����}�[�W���܂��B
���̏������}�[�W�\�[�g�ƌĂт܂��B

���̐����ŕ�����悤�ɁA�}�[�W�\�[�g�����̓����Ń}�[�W�\�[�g���s���Ă��܂��B

�}�[�W�\�[�g���ċA�I�ɌJ��Ԃ����ƂŁA�ŏ��v�f�ɕ��������܂ŕ������s���A�ŉ��w���珇�Ƀ}�[�W�\�[�g��K�p���ă}�[�W���āA�K�w������Ȃ���\�[�g���Ă����܂��B

Rust�ŏ����ƈȉ��̂悤�ɂȂ�܂��B
������`merge`�֐��͐�q�̂��̂��g�p���܂��B

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

## �}�[�W�\�[�g�̉��

�}�[�W�\�[�g�A���S���Y���́A���߂�ꂽ�������ċA�I�ɌJ��Ԃ��܂��B
���̏����̌v�Z���Ԃ̓T�C�Y�Ɉˑ�����Ƃ���$T(n)$�ƕ\�����܂��B
�T�C�Y$n$�̏������ł́A�T�C�Y$n/2$�̌��߂�ꂽ������2��J��Ԃ���܂��B�����Ė��𕪊�����v�Z���Ԃ�$D(n)$�A�������������Č�������}�[�W�����̌v�Z���Ԃ�$C(n)$�Ƃ���ƁA�{�ɋL�ڂ���Ă��鑍�v�Z���Ԃ̑Q���������܂�܂��B

�}�[�W�\�[�g�̕��������́A�a�Ɗ���Z��1�񂸂����Ȃ̂ŁA�萔�̃I�[�_�[$\Theta(1)$�ɂȂ�܂��B

��������`merge`�̓T�C�Y��1���̃I�[�_�[$\Theta(n)$�ɂȂ�܂��B

�}�X�^�[�藝��p����ƁA�����$T(n)=\Theta(n\log n)$�ɂȂ�$n^2$�̃I�[�_�[���v�Z���Ԃ��Z���Ȃ�܂��B

�}�[�W�\�[�g�̏ꍇ��$\Theta(n)=cn$�Ƃ��Čv�Z����ƈȉ��ɂȂ�܂��B

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
&=& (�ċA���x���̐�)*cn 
\end{eqnarray}
```

�ċA���x���̐���$\lg n+1$�ł��邱�Ƃ͑Q�����ŏؖ��ł��܂��B

$n=1$ �̎��́A�ċA���x���̐���1�ł���A$\lg 1=0$�Ȃ̂�$\lg 1 + 1=1$�ƂȂ萳�����ł��B

�T�C�Y��$n-1=2^{k-1}$�Ƃ����ꍇ�A�ċA���x���̐���$\lg2^{k-1}+1$�ł���Ƃ��܂��B
�����čċA���x���̐���1���₷�ƁA���̃��x���ł̃T�C�Y��$n=2^k$�ƂȂ�܂��B

```math
\begin{eqnarray}
&& �T�C�Yn�̍ċA���x���̐� \\
&=& �T�C�Y(n-1)�̍ċA���x���̐�+1 \\
&=& \lg 2^{k-1}+1+1 \\
&=& k-1+2 \\
&=& k+1 \\
&=& \lg 2^k+1 \\
&=& \lg n+1 \\
\end{eqnarray}
```

�ƂȂ�܂��B
�ȏ�ŏؖ������ł��B

������}�[�W�\�[�g�̌v�Z�̎����� $\Theta(n \lg n)$ �ł��邱�Ƃ�������܂��B

## A2.3-1
�ȉ������s�����O���擾���܂��B

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

v��4�̑g�A[3, 41], [52, 26], [38, 57], [9, 49]�ɂ܂ŕ�������A`merge_sort`�֐��ɂ��\�[�g����Ă���A`merge`�֐��ɂ��2�g�����������[3, 41, 26, 52]��[38, 57, 9, 49]�ɂȂ�A�ă\�[�g����Ă���Č��������[3, 26, 41, 52, 9, 38, 49, 57]�ɂȂ�A�ŏI�I�ȃ\�[�g�������̂�������܂��B

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
    println!("v={:?}, p={}, q={}, r={}", &v[0..r+1], p,q,r);
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
$n=2$ �̏ꍇ�A$T(2)=2\lg 2=2$���������܂��B

$n-1=2^{k-1}$ �̏ꍇ�A$T(n-1)=2^{k-1}\lg 2^{k-1}$����������Ƃ��āA�T�C�Y��$n=2^k$��$T(n)$�����߂�ƈȉ��ɂȂ�܂��B

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

����ė^�����������܂��B
�ȏ�ŏؖ������ł��B

## A2.3-4
����z�񂪁A�Ō�̗v�f�������ă\�[�g����Ă���Ƃ��āA�Ō�̗v�f�ɂ��Ă����}���\�[�g����ꍇ���l���܂��B

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

���ɍċA�I�ɂ��̃A���S���Y�����J��Ԃ��āA�C�ӂ̔z����\�[�g���Ă݂܂��B

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

�z��̃T�C�Y��1���傫������A�Ō�̗v�f���������z���}���\�[�g�֐��ɑ�����Ă���A�Ō�̗v�f��߂��܂��B
���ꂩ��Ō�̗v�f�ɂ��đ}���\�[�g��K�p���܂��B

����ɂ��A�ċA�I�ɃT�C�Y��2�ɂȂ�܂ōŌ�̗v�f�������A�T�C�Y��2�̔z��ɑ}���\�[�g��K�p���Ċ֐��𔲂��āA�z��̍Ō���ɗv�f���ǉ�����Ă���}���\�[�g���K�p����Ċ֐��𔲂��āA�z��̍Ō���ɗv�f���ǉ�����Ă���}���\�[�g���K�p����Ċ֐��𔲂��āA�z��̍Ō���ɗv�f���ǉ�����Ă���}���\�[�g���K�p����Ċ֐��𔲂���...���J��Ԃ��܂��B

�ł͂��̃A���S���Y���̌v�Z���� $T(n)$ ��Q�����ŋ��߂܂��B
�ċA�A���S���Y���̌v�Z�́A�����A�����A������3���琬��܂��B

������pop�ōŌ�̗v�f�����o���v�Z�ŁA�v�Z�̎����� $\Theta(1)$ �ł��B
�����́A�v�f��1������ $T(n-1)$ �ɂȂ�܂��B
������push�Ƃ���ɑ����\�[�g�֐��̓K�p�ł��B

����̃\�[�g�֐��́A�ȑO�̃\�[�g�֐��ƈقȂ�for�̌J��Ԃ����Ȃ��̂ŁA�v�Z�̎����� $\Theta(n)$ �ɂȂ�܂��B

```math
\begin{eqnarray}
T_{insert_last}(n) &=& c_1 n + c_2 (n-1) + c_4(n-1) + c_5 n + c_6 (n-1) + c_7 (n-1) + c_8 (n-1)
\end{eqnarray}
```

����đQ�����͈ȉ��ɂȂ�܂�

```math
\begin{eqnarray}
T(n) &=& \Theta(1) + T(n-1) + \Theta(n) \\
&=& T(n-1) + \Theta(n)
\end{eqnarray}
```

## A2.3-5

```rust
fn search(v: i8)-> usize {
    let a: [i8; 6] = [26, 31, 41, 41, 59, 58];
    println!("{:?}", &a[0..6]);
    
    let mut start_index;
    let mut end_index;
    let mut search_index;
    
    start_index = 0;
    end_index = a.len()-1;
    
    loop {
        search_index = start_index + (end_index+1-start_index)/2;
        println!("start_index = {}, end_index = {}, search_index = {}", start_index, end_index, search_index);
        
        if search_index == 0 || search_index == a.len()-1 {
            break;
        }
        
        if v == a[search_index] {
            return search_index+1;
        } else if v > a[search_index] {
            start_index = search_index+1;
        } else {
            end_index = search_index-1;
        }
        
    }
    
    for i in start_index..end_index+1 {
        //println!("start_index = {}, end_index = {}, search_index = {}", start_index, end_index, search_index);
        //println!("a[{}] = {}", i, a[i]);
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
    println!("{}��{}�Ԗڂ̒l", v, search(v));
    
    let v = 31;
    println!("{}��{}�Ԗڂ̒l", v, search(v));
    
    let v = 41;
    println!("{}��{}�Ԗڂ̒l", v, search(v));
    
    let v = 59;
    println!("{}��{}�Ԗڂ̒l", v, search(v));

    let v = 100;
    println!("{}��{}�Ԗڂ̒l", v, search(v));
}
```


�Q�����́A�v�Z�T�C�Y�͔������ɂȂ��Ă����̂ňȉ��ɂȂ�܂��B

```math
\begin{eqnarray}
T(n) &=& T(n/2) + c
\end{eqnarray}
```

�Q������W�J��������ƁA�ċA���x���̐��ɔ�Ⴗ�邱�Ƃ�������܂��B

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
&=& (�ċA���x���̐�)*c 
\end{eqnarray}
```

�ċA���x���̐��̓}�[�W�\�[�g�̏ꍇ�Ɠ����Ȃ̂� $\lg n$ �ł��B

����Čv�Z�̎����� $\Theta(\lg n)$ �ɂȂ�܂��B

## A.2.3-6

����z�񂪁A�Ō�̗v�f�������ă\�[�g�ς݂Ƃ��܂��B
�\�[�g�ς݂̔z��̒����̗v�f�ƍŌ�̗v�f���r���܂��B
�����̗v�f���傫���ꍇ�A�������͒l���������ꍇ�́A�Ō�̗v�f�ɂ��߂��l���܂��������ɂ��邩������܂���B

�Ⴆ�΁A[1, 5, 10, 20�i��r�l�j,30, 40, 50, 2] �Ȃ�� [1, 5, 10]��2�ɋ߂��v�f������܂� 

�Ō�̗v�f���傫���ꍇ�́A�Ō�̗v�f�ɂ��߂��l���܂��E�����ɂ��邩������܂���B

�Ⴆ�΁A[1, 5, 10, 20�i��r�l�j,30, 40, 50, 45]�Ȃ�� [30, 40, 50]��45�ɋ߂��v�f������܂� 

�����ōX�ɁA���̔����̒����̗v�f�Ƒ召���r���āA�܂��E��������������I�����A���̔����̒����̗v�f�Ƒ召���r���ĂƂ������Ƃ��J��Ԃ��܂��B

���̌J��Ԃ��́A��r����v�f�̃C���f�b�N�X���\�[�g�ςݔz��̍ŏ����Ō�ɂȂ�܂ő����܂��B


�Ⴆ�΁A[1, 5�i��r�l�j, 10, 2] �Ȃ�� �A���̔�r�v�f�̓\�[�g�ςݔz��̍ŏ��̗v�f�ł���[1]�ɂȂ�A���̔�r��[1,2]�̏��Ƀ\�[�g����I���ł��B 
�Ⴆ�΁A[1, 5�i��r�l�j, 10, 6] �Ȃ�� �A���̔�r�v�f�̓\�[�g�ςݔz��̍Ō�̗v�f�ł���[10]�ɂȂ�A���̔�r��[6,10]�̏��Ƀ\�[�g����I���ł��B 

�C�ӂ̔z��ɑ΂��āA�ŏ���2�v�f�ɂ��ď�L�̃\�[�g���s���A���Ƀ\�[�g�ς݂�2�v�f��3�Ԗڂ̗v�f�ɂ��āA���̎��Ƀ\�[�g�ς݂�3�v�f��4�Ԗڂ̗v�f�ɂ���...�Ɠ��l�̂��Ƃ��\�[�g�ς݂�n-1�v�f��n�Ԗڂ̗v�f�ɂȂ�܂ŌJ��Ԃ��܂��B

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

�z��𔼕��ɂ��Ă������Ƃ��J��Ԃ��ꍇ�̌v�Z�̎����́A A2.3-5 �̏ꍇ�Ɠ��l�� $\Theta(\lg n)$ �ł��B
�����for���[�v��n-1��J��Ԃ��̂ŁA���̃A���S���Y���̌v�Z�̎����́A $\Theta(n \lg n)$ �ɂȂ�܂��B
