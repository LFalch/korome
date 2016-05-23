extern crate korome;

#[test]
fn glium_reexport_works() {
    let _b = korome::glium::glutin::WindowBuilder::new();
}
