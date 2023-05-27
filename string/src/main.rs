fn main() {
  // 创建
    let mut s = String::new();
    let mut s1 = String::from("some String");
    let s2 = "some String".to_string();

  // 更新str
  s.push_str("hello");
  s.push('!');
  // 使用+链接时,s1的所有权会交给s3
  let s3 = s1 + &s2;
  let s1 = String::from("a");
  let s2 = String::from("b");
  let s3 = String::from("c");
  let res = format!("{}-{}-{}", s1, s2, s3);
  println!("{}", res);

  // 访问unicode标量值
  for b in "你好".chars() {
    println!("{}", b);
  }

}
