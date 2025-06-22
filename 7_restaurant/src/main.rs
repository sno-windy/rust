use restaurant::hosting;  // re-exporting
// 元々は use restaurant::front_of_house::hosting;

fn main() {
    hosting::add_to_waitlist();
    restaurant::eat_at_restaurant();
}