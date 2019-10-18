use std::process::Command;

/// git commit no quotes
fn main() {
  Command::new("git")
    .arg("commit")
    .arg("-m")
    .arg(nq::get_message())
    .spawn()
    .unwrap()
    .wait_with_output()
    .unwrap();

  Command::new("git")
    .arg("pull")
    .arg("--rebase")
    .spawn()
    .unwrap()
    .wait_with_output()
    .unwrap();

  Command::new("git")
    .arg("push")
    .spawn()
    .unwrap()
    .wait_with_output()
    .unwrap();
}
