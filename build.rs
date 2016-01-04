use std::process::Command;
use std::fs::File;
use std::io::Write;

fn main() {
    let output = Command::new("git")
        .arg("describe")
        .output().unwrap();

    if output.status.success(){
        let mut file = File::create("version.txt").unwrap();
        file.write_all(&output.stdout[..output.stdout.len()-1]).unwrap();
    } else {
        panic!("output status not success")
    }
}
