fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle::square(30);

    // areaが消えていいなら、参照渡ししなくていい
    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     area(&rect)
    // );
    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );
    println!(
        "Can rect hold another rect? {}",
        rect.can_hold(&rect2)
    );
    println!("{:#?}", rect);
}

// method じゃない方法もあるけど...
// fn area(rect: &Rectangle) -> u32 {
//     rect.width * rect.height
// }

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, another: &Rectangle) -> bool {
        self.width >= another.width && self.height >= another.height
    }

    // static function
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
