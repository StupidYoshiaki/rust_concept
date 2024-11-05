# 3章

- イミュータブルはメモリ上に書き換えできない変数として保持されるが、所有権は存在する。
- ミュータブルな変数はメモリ上に書き換え可能な変数として保持される。
  - だから、Pythonの配列のように、リストの要素を変更すると、メモリ上のアドレスが変わる。

つまり、イミュータブルとミュータブルの違いは値の書き換えができるかどうか。
イミュータブルをRead権限、ミュータブルをWrite権限と考えるとわかりやすい。
Write権限を同時に複数与えるとデッドロックが発生する可能性があるため、Rustではミュータブルな借用は一つしかできない。

関数の中で値を変更する場合には、ミュータブルなリファレンスを渡す必要がある。
ミュータブルなリファレンスとは、値を変更できるように所有権を借用できるということ。
```rust
fn myclear(x: &mut String) {
    x.clear();
}

fn main() {
    let mut s = "Hello".to_string();
    println!("s = {}", s);

    let s_ref = &mut s;
    myclear(s_ref);
    println!("s = {}", s);
}
```

ただし、同時にミュータブルな借用は複数できない。
- Cコードのポインタのように、ある変数を変更すると、他の変数も変更されてしまう可能性があるため。
- 競合が起こってしまう可能性があるため。

でも、これはセーフ（だって同時じゃないから）。
```rust
fn myclear(x: &mut String) {
    x.clear();
}

fn main() {
    let mut s = "Hello".to_string();
    println!("s = {}", s);

    let s_ref = &mut s;
    myclear(s_ref);
    println!("s = {}", s);

    let s_ref2 = &mut s;
    myclear(s_ref2);
    println!("s = {}", s);
}
```

リファレンスのライフタイムの中で値を変更することはできないので、以下のコードはエラーになる。
この場合のx_refのスコープは3~6行目（println!）まで。
それまではx_refが所有権を借用してるわけだから、変更できない。
```rust
fn main() {
    let mut x = 1;
    let x_ref = &x;

    x = 2;
    println!("{}", x_ref)
}
```

return_hello関数のローカル変数sは、関数のスコープを抜けるとメモリから解放される。
よって、存在しないリファレンスを参照しようとしてるので、エラーになる。
```rust
fn return_hello() -> &String {
    let s = "Hello".to_string();
    &s
}

fn main() {
    let s = return_hello();
    println!("{}", s);
}
```

main関数で変数を定義して、そのリファレンスを関数に渡してるので、以下のコードはエラーにならない。
先ほどは、関数内のローカル変数だったが、今回はmain関数内の変数なので、関数のスコープを抜けても解放されない。
```rust
fn pick1(x: &[i32], end: usize) -> &[i32] {
    &x[..end]
}

fn main() {
    let v1 = [1, 2, 3, 4, 5];
    let p = pick1(&v1, 2);
    for ss in p {
        println!("{}", ss)
    };
}
```
v1がのライフタイムが終了するケースは以下の二つである。
- main関数の終了
- v1を別の変数にムーブした場合

複数の引数がリファレンスとして渡される場合、ライフタイムが一致している必要がある。
pick2関数の返り値がどの引数のライフタイムに従えばいいのかわからないので、エラーになる。
```rust
fn pick2(x: &[i32], y: &[i32], end: usize) -> &[i32] {
    (&x[..end], &y[..end])
}

fn main() {
    let v1 = [1, 2, 3, 4, 5];
    let v2 = [6, 7, 8];

    let p = pick2(&v1, &v2, 2);
    for ss in p.0 {
        println!("{}", ss)
    };
    for ss in p.1 {
        println!("{}", ss)
    };
}
```

このような場合は、ライフタイムパラメータによって明示的に指定する。
```rust
fn pick2<'a, 'b>(x: &'a [i32], y: &'b [i32], end: usize) -> (&'a [i32], &'b [i32]) {
    (&x[..end], &y[..end])
}

fn main() {
    let v1 = [1, 2, 3, 4, 5];
    let v2 = [6, 7, 8];

    let p = pick2(&v1, &v2, 2);
    for ss in p.0 {
        println!("{}", ss)
    };
    for ss in p.1 {
        println!("{}", ss)
    };
}
```

ライフタイムは同じでもいいので、以下のように書くこともできる。
大事なのは、返り値が参照元の値のライフタイムを超えていないということをコンパイラに伝えること。
```rust
fn pick2<'a>(x: &'a [i32], y: &'a [i32], end: usize) -> (&'a [i32], &'a [i32]) {
    (&x[..end], &y[..end])
}

fn main() {
    let v1 = [1, 2, 3, 4, 5];
    let v2 = [6, 7, 8];

    let p = pick2(&v1, &v2, 2);
    for ss in p.0 {
        println!("{}", ss)
    };
    for ss in p.1 {
        println!("{}", ss)
    };
}
```

参考になりそうなサイト
- [Rustのライフタイムについて知りたい](https://qiita.com/toreis/items/970bcfed6a930e3c84dc)

# 4章
構造体にリファレンスを持たせる場合、ライフタイムを指定する必要がある。
```rust
#[derive(Debug)]
struct Person {
    name: String, 
    age: u8, 
}

#[derive(Debug)]
struct Parents<'a, 'b> {
    father: &'a Person, 
    mother: &'b Person, 
}
impl<'a, 'b> Parents<'a, 'b> {
    fn new(father: &'a Person, mother: &'b Person) -> Parents<'a, 'b> {
        Parents{father, mother}
    }
}

fn main() {
    let taro = Person{
        name: String::from("taro"), 
        age: 50, 
    };
    let hanako = Person{
        name: String::from("hanako"), 
        age: 48, 
    };

    let sato = Parents::new(&taro, &hanako);
    println!("{:?}", sato);
}
```

列挙型は取りうる値が全て列挙されたもので、matchと相性が良い。
```rust
enum Sign {
    Positive, 
    Zero, 
    Negative, 
}

use std::cmp::Ordering;
fn determine_sign(x: i32) -> Sign {
    match x.cmp(&0) {
        Ordering::Greater => Sign::Positive, 
        Ordering::Less => Sign::Negative, 
        Ordering::Equal => Sign::Zero, 
    }
}

fn print_sign(s: Sign) {
    match s {
        Sign::Positive => println!("+"), 
        Sign::Zero => println!("0"), 
        Sign::Negative => println!("-"), 
    }
}

fn main() {
    print_sign(determine_sign(1));
    print_sign(determine_sign(-2));
    print_sign(determine_sign(0));
}
```

タプルや構造体などの複数の値を持つ型を列挙型にすることもできる。
```rust
enum EnumExample {
    TupleTypeExample1(String),
    TupleTypeExample2(i32, bool),
    StructTypeExample{name: String, age: u8},
}

fn main() {
    let mut v: Vec<EnumExample> = Vec::new();

    v.push(EnumExample::TupleTypeExample1(String::from("Hello")));
    v.push(EnumExample::TupleTypeExample2(10, true));
    v.push(
        EnumExample::StructTypeExample{
            name: String::from("taro"),
            age: 10,
        }
    );

    for e in &v {
        if let EnumExample::StructTypeExample{name: n, age: a} = e {
            println!("StructTypeExample_iflet: name = {}, age = {}", n, a);
        }
    }

    for e in &v {
        match e {
            EnumExample::TupleTypeExample1(s) => {
                println!("TupleTypeExample1: s = {}", s);
            }
            EnumExample::TupleTypeExample2(n, b) => {
                println!("TupleTypeExample1: n = {}, b = {}", n, b);
            }
            EnumExample::StructTypeExample{name: n, ..} => {
                println!("StructTypeExample: name = {}", n);
            }
        }
    }
}
```