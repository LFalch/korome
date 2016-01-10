use std::process::Command;
use std::fs::File;
use std::io::Write;

fn main() {
    // Runs `git describe` to always get some useful version
    let output = Command::new("git")
        .arg("describe")
        .output().unwrap();

    let path = concat!(env!("OUT_DIR"), "/version");

    if output.status.success(){
        let mut file = File::create(path).unwrap();
        // Writes the output from the command into a file, omitting the newline at the end
        file.write_all(&output.stdout[..output.stdout.len()-1]).unwrap();
    } else {
        panic!("output status not success")
    }
}
