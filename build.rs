extern crate glium;

use glium::DisplayBuild;
use glium::program::Program;

use std::io::Write;
use std::fs::File;

fn main() {
    let window = glium::glutin::WindowBuilder::new().with_visibility(false).build_glium().unwrap();

    let bin = Program::from_source(&window, include_str!("src/shaders/texture.vs"), include_str!("src/shaders/texture.fs"), None).unwrap().get_binary().unwrap();

    let mut content = File::create(concat!(env!("OUT_DIR"), "/content.inc")).unwrap();
    content.write_all(&bin.content).unwrap();

    let mut file = File::create(concat!(env!("OUT_DIR"), "/binary.in.rs")).unwrap();

    file.write_all(format!("Binary {{\n\tformat: {:?},\n\tcontent: include_bytes!(concat!(env!(\"OUT_DIR\"), \"/content.inc\")).to_vec()\n}}", bin.format).as_bytes()).unwrap();
}
