pub mod front_of_house;
pub mod back_of_house;

pub use crate::front_of_house::hosting;

fn serve_order() {}

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // Error: private access
    // meal.fruit = String::from("Peach");

    hosting::add_to_waitlist();

    hosting::add_to_waitlist();
}
