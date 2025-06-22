fn main() {
    let no: Maybe<i32> = Maybe::Nothing;
    let yep: Maybe<i32> = Maybe::Just(5);
    println!("{:?}", yep);
    println!("{}", no.is_some());
    println!("{}", yep.is_some());
    let yes = yep.expect();
    println!("{:?}", yes);
    println!("{:?}", yep);
}

#[derive(Debug)]
enum Maybe<T> {
    Nothing,
    Just(T),
}

impl<T> Maybe<T> {
    fn is_some(&self) -> bool {
        match self {
            Maybe::Nothing => false,
            Maybe::Just(_) => true
        }
    }
    
    // これだとmovedされてしまうので別の実装が必要
    fn expect(&self) -> &T {
        match self {
            Maybe::Just(x) => x,
            Maybe::Nothing => panic!("No value")
        }
    }
}