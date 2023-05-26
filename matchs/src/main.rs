enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter,
}

fn main() {
    println!("Hello, world!");
}

fn value_in_cents(coin: Coin) -> u32 {
  match coin {
    Coin::Penny => 1,
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin::Quarter => 25
  }
}

// match必须穷举所有的可能
// 可以使用_代替
fn match_number() {
  let n = 0u32;
  match n {
      1 => println!("1"),
      2 => println!("2"),
      _ => (),
  }
}