use std::process::Command;
use std::str;

fn main() {
    let pass = "123";
    let output = Command::new("sh")
        .arg("-c")
        .arg(format!("zip -e test.zip test.txt -P {pass}"))
        .output()
        .expect("failed to execute process");
    println!("{:?}", str::from_utf8(&output.stdout));
}
