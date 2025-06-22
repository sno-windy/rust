# rectangle で学んだこと

## derive Debug
#[derive(Debug)]で 文字列中の `{:?}` もしくは `{:#?}` でDebug表示ができるようなる
`derive` annotation で trait を使用できる。 PHPのtraitは自分たちで作るもののイメージなので、そこは新しく感じる

## method
static fn（factory） と method は特に記載分ける必要なし。

!具体例欲しい

## tuples

以下の場合はmutいらない。tupは別に変わっていないから。所有権もいらないんですね。
```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
}
```

## array
arrayは固定長、vectorは自由。
```rust
fn main() {
let a: [i32; 5] = [3; 5];
}
```
ホーン


## blocks
x+1; 4をreturnする式でも受け取れるよ
```rust
fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}
```


(1..4)とか, shellと一緒だね

## Managing heap data is why do ownership exists 
ヒープデータを管理することが所有権の存在する理由らしい
https://doc.rust-jp.rs/book-ja/ch04-01-what-is-ownership.html


## variables; *moved*
Rustは変数がスタックに積まれているか、ヒープに積まれているかを考えないといけない。
以下の例で示す。
```rust
fn main() {
    // 動く
    let x = 5;
    let y = x;
    println!("{}", x);
    println!("{}", y);

    // 動かない
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}", s1);  // borrow of moved value: `s1`
    println!("{}", s2);
}
```

スタックに積まれるかどうかは、その型がコンパイル時に完全にスタックに積めるかどうかで決まる。

### 一例
u32, bool, f64, char, **(i32, char) といったtuple** 


## 借用
借用とは、&s1などを関数の引数で渡すことである。
借りてるだけなので、所有権が元の関数から移動しない。
よって使用先ではimmutableである。


## slice
スライスはmutの必要がない。変更はしていないからである
`&s[..5], &s[1..], &s[..] など。`