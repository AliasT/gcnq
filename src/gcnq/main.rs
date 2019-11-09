use std::error;
use std::process::Command;

/// git commit no quotes
fn main() -> Result<(), Box<dyn error::Error>> {
  Command::new("git")
    .arg("add")
    .arg(".")
    .spawn()?
    .wait_with_output()?;

  Command::new("git")
    .arg("commit")
    .arg("-m")
    .arg(nq::get_message())
    .spawn()?
    .wait_with_output()?;

  Command::new("git")
    .arg("pull")
    .arg("--rebase")
    .spawn()?
    .wait_with_output()?;

  Command::new("git")
    .arg("push")
    .spawn()?
    .wait_with_output()?;

  Ok(())
}
