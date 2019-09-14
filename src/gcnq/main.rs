use std::process::Command;

/// git commit no quotes
fn main() {
    Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(nq::get_message())
        .spawn()
        .unwrap();
}
