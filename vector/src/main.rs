fn main() {
    // 创建
    let mut v = Vec::new(); //可以不显示指明类型
    let mut v1 = vec![1,2,3];
    // 追加
    v.push(1);
    v1.push(4);
    // 获取索引
    let third = &v1[2];
    let third1 = match v.get(2) {
        Some(third) => third,
        None => &0,
    };
    forin();
}

// 遍历vec 并将每个数组内容*2
fn forin() {
  let mut v =  vec![1,2,3,4];
  for i in &mut v {
    *i *= 2;
  }
  for i in v {
    println!("{}", i);
  }
}

// 如果存放不同的数据类型可以用枚举存放
enum SpreadSheetCell {
  Int(i32),
  Float(f64),
  Text(String),
}

fn option_vec () {
  let row = vec![
    SpreadSheetCell::Int(3),
    SpreadSheetCell::Float(3.0),
    SpreadSheetCell::Text(String::from("3")),
  ];
}