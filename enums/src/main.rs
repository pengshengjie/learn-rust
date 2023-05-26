fn main() {
  opeara();
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
// 为枚举定义方法
impl Message {
    fn call(&self) {
      println!("this is a message call")
    }
}
fn opeara() {
  let q = Message::Quit;
  let m = Message::Move{x: 10, y: 10};
  let w = Message::Write(String::from("Hello"));
  let c = Message::ChangeColor(255,255,255);
  c.call();
}
enum IpAddrkind {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn ip_demo(ip: IpAddrkind) {
    let four = IpAddrkind::V4(127, 0, 0, 1);
    let six = IpAddrkind::V6(String::from(":1"));
}

// Option枚举
// 在标准库中Option枚举是这样设计的
// Option 是在预先导入中的
// enum Option<T> {
//     Some(T),
//     None,
// }
fn options() {
  let some_number = Some(5);
  let some_string = Some("String");
  let absent_numbe: Option<i32> = None;
}

fn add_tow_number() {
  let a = 5;
  let b = Some(5);
  // can't a + b
}