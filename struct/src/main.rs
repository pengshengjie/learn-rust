#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn get_area(&self) -> u32 {
        self.width * self.length
    }
    fn can_hold(&self, other_react: Rectangle) -> bool {
        self.width > other_react.width && self.length > other_react.length
    }

    fn square(width: u32) -> Rectangle {
        Rectangle {
            length: width,
            width,
        }
    }
}

fn main() {
    let rect = Rectangle {
        length: 30,
        width: 50,
    };
    let other_react = Rectangle {
        length: 40,
        width: 60,
    };
    let square = Rectangle::square(20);
    let area = rect.get_area();
    println!("{}", area);
    println!("can hoold {}", rect.can_hold(other_react));
    println!("can hoold {}", rect.can_hold(square));
}
