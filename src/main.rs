use std::env;
use std::process::Command;

/// git commit no quotes
fn main() {
    let message = env::args()
        .skip(1)
        .fold(String::new(), |acc, cur| format!("{} {}", acc, cur));

    Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(message)
        .spawn()
        .unwrap();
}
