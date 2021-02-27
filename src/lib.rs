pub const GREETING: &'static str = "Hallo, Rust library here!";

pub fn greet(val: u32) -> String {
  return format!("value is {}", val);
}