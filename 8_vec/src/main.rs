fn main() {
    let mut v = Vec::new();
    v.push(String::from("hello"));
    v.push(String::from("world"));
    v.push(String::from("!"));
    match v.get(2) {
        Some(third) => println!("The third element is definitely {}!", third),
        None => println!("There is no third element."),
    }
    // 基本的にはだめ。Vec<String>など。何故なら、これを書くとVec内のメモリがthirdのメモリにmoveしてしまうから
    // let third = v[2];
    let third = &v[2];
    println!("The third element is {}", third);
    // 自動derefが働く
    let thirdref = &&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&v[2];
    println!("The third element is {}", thirdref);


    // 借用なのでs2は引き続き使えるよ
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // s1はムーブされ、もう使用できないことに注意。s1は&にできない...。
    // fn add(self, s: &str) -> String {} を呼び出してるらしい。うーんこの
    let s3 = s1 + &s2; 
}
