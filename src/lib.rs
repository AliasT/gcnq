use std::env;

pub fn get_message() -> String {
  env::args()
    .skip(1)
    .fold(String::new(), |acc, cur| format!("{} {}", acc, cur))
}
