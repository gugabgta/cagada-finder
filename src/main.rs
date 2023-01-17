use std::process::Command;

fn main() {
    let output = Command::new("git")
                    .arg("diff")
                    .arg("--staged")
                    .output()
                    .expect("failed to execute process");

    let stdout = String::from_utf8(output.stdout).unwrap();
    println!("{}", stdout);
}
