mod front_hose {
  pub mod hosting {
    pub fn test() {
      println!("Hello world")
    }
  }
}

pub mod page;

pub fn eat () {
  crate::front_hose::hosting::test()
}