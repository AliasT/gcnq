use std::process::Command;

/// git commit no quotes
fn main() {
  let child = Command::new("git")
    .arg("commit")
    .arg("-m")
    .arg(nq::get_message())
    .spawn()
    .unwrap();

  child.wait_with_output().unwrap();
}
