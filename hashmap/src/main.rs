use::std::collections::HashMap;

fn main() {
    // 创建hash map
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let k = String::from("some");
    let v = 30;
    scores.insert(k, v);
    // 下一行println k v 已经将所有权交给了scores所以k v无法使用
    // println!("{}, {}", k, v);
    // 使用collect创建
    let keys = vec!["Yellow", "Blue"];
    let values = vec![10, 50];
    let res: HashMap<_, _> = keys.iter().zip(values.iter()).collect();
    // HashMap获取值
    match  scores.get("Blue") {
        Some(k) => println!("{}", k),
        None => println!("not found"),
    }
    // 遍历hash Map
    for(k, v) in &scores {
      println!("{}-{}", k, v);
    }
    // 更新HashMap
    // 如果K存在
      // 替换现有值
      // 保留现有值,忽略新的
      // 合并现有值和新的
    // 添加新的

    // 替换
    scores.insert(String::from("Haha"), 25);
    scores.insert(String::from("Haha"), 26);
    println!("{:#?}", scores);
    // 保留现有，忽略新的
    scores.entry(String::from("Haha")).or_insert(30);
    // 现有没有,则直接插入
    scores.entry(String::from("New")).or_insert(30);
    println!("{:#?}", scores);

    count_words();

}

// count words
fn count_words() {
  let words = "hello world my world";
  let mut map = HashMap::new();
  for word in words.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1
  }
  print!("{:#?}", map)
}