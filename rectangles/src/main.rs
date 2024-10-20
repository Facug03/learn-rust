#[derive(Debug)]
struct Rect {
  width: u32,
  height: u32,
}

impl Rect {
  fn area(&self) -> u32 {
    self.width * self.height
  }

  fn can_hold(&self, other: &Rect) -> bool {
    self.width > other.width && self.height > other.height
  }
}

fn main() {
  let rect1 = Rect {
    height: 50,
    width: 30,
  };
  let rect2 = Rect {
    height: 40,
    width: 35,
  };

  println!("rect1 is {rect1:#?}");

  println!(
    "The area of the rectangle is {} square pixels.",
    rect1.area()
  );

  println!("Can react1 hold react2? {}", rect1.can_hold(&rect2));
}
