fn fix_incorrect_order() {
    cook_order();
    super::serve_order();
}

fn cook_order() {}

pub struct Breakfast {
    pub toast: String,
    fruit: String,
}

impl Breakfast {
    pub fn summer(toast: &str) -> Self {
        Self {
            toast: String::from(toast),
            fruit: String::from("peach"),
        }
    }
}
