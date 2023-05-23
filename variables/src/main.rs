fn main() {
  // mut 关键字可以变
    let mut x = 5;
    println!("x is {}", x);
    x = 6;
    println!("x is {}", x);
    test();
    shadow();
    panic();
    char_font();
    type_tup();
}

fn test() {
  const MAX_POINT:u32 = 100_000;
  println!("MAX_POINT is {}", MAX_POINT);
}

fn shadow () {
  // shadow 可以覆盖之前的变量
  let x = 1;
  let x = x + 1;
  println!("x is {}", x);
  let x = "ss";
  println!("x is {}", x);
}

fn panic () {
  let n:u8 = 255;
  println!("n is u8 {}", n);
}

fn char_font() {
  // 字符类型
  let c = 's';
  println!("c is u8 {}", c);
}

fn type_tup() {
  // 元组 及 解构
  let tup:(i32, f32, bool) = (12, -21.0, true);
  let (x, y, z) = tup;
  println!("x is {}, y is {}, z is {}", x, y, z);
  // 访问tup
  println!("tup.0 is {}", tup.0);
}

fn type_vector () {
  let dyas = ["星期一", "星期二","星期三"];
  // 数组长度为5，且里面的内容是3
  let five3 = [3;5];
}