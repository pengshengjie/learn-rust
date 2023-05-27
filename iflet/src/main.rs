fn main() {
    // 针对一种匹配模式 可以使用if let
    let v = Some(3);
    if let Some(3) = v {
      println!("three")
    }

    if let Some(4) = v {
      println!("three");
    } else {
      println!("not");
    }
}
