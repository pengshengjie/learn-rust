use::std::cmp::Ordering;
use std::io::stdin;
use::rand::Rng;

fn main() {
  let secret_number = rand::thread_rng().gen_range(1..100);
  loop {
    let mut guess = String::new();
    stdin().read_line(&mut guess).unwrap();
    let guess: u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => continue,
    };

    match guess.cmp(&secret_number) {
        Ordering::Greater => println!("Greater"),
        Ordering::Less => println!("Less"),
        Ordering::Equal => {
          println!("YOU WIN！,and the secret_number is {}", secret_number)
        }
    }
  }
}
