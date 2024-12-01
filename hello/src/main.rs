fn foo(count: i32) -> &'static str {
  if count < 5 {
    return "Small World";
  } else {
    return "Big World";
  }
}

fn main() {
  // let world: &'static str = "World";
  println!("Hello, {}", foo(4));
}
