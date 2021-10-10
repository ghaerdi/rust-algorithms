use std::process::Command;

fn main() {
  Command::new("cp")
    .args(["-a", ".githooks/.", ".git/hooks/"])
    .output()
    .expect("failed to execute process");
}
