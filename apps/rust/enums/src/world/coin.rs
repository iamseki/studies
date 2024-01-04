
#[derive(Debug)]
pub enum UsState {
    Alabama,
    Alaska
}

pub enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter(UsState)
}

pub fn value_in_cents(coin: Coin) -> u8 {
  match coin {
      Coin::Penny => { 
          println!("Lucky penny!");
          1
      },
      Coin::Nickel => 5,
      Coin::Dime => 10,
      Coin::Quarter(state) => {
          println!("state quarter from {:?}!", state);
          25
      }
  }
}
