pub mod front_of_house;

pub use crate::front_of_house::hosting;

fn serve_order() {}

mod back_of_house {
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
}

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // Error: private access
    // meal.fruit = String::from("Peach");

    hosting::add_to_waitlist();

    hosting::add_to_waitlist();
}
