use _rust_lib::restaurant;
fn main() {
    let mut meal = restaurant::Breakefast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    let order1 = restaurant::Appetizer::Salada;
    let order2 = restaurant::Appetizer::Soup;

    dbg!(order1);
    dbg!(order2);

    println!("pub enums has all of its variants public as well differently from structs")
}

