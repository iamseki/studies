pub struct Breakefast {
  pub toast: String,
  seasonal_fruit: String
}

impl Breakefast {
  pub fn summer(toast: &str) -> Breakefast {
    Breakefast { toast: String::from(toast), seasonal_fruit: String::from("peaches") }
  }
}

#[derive(Debug)]
pub enum Appetizer {
  Soup,
  Salada
}