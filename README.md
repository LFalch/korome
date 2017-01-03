# korome
[![Travis Build Status](https://img.shields.io/travis/LFalch/korome.svg?style=flat-square)](https://travis-ci.org/LFalch/korome)
[![AppVeyor Build Status](https://ci.appveyor.com/api/projects/status/github/LFalch/korome?branch=master&svg=true)](https://ci.appveyor.com/project/LFalch/korome)
[![Crates.io](https://img.shields.io/crates/v/korome.svg?style=flat-square)](https://crates.io/crates/korome)
![Licence](https://img.shields.io/crates/l/korome.svg?style=flat-square)
[![Docs.rs](https://docs.rs/korome/badge.svg)](https://docs.rs/korome)

A game engine in Rust (previously Java) using [glium](https://github.com/tomaka/glium)

To use korome, add this to your Cargo.toml:
```toml
[dependencies]
korome = "0.13"
```

Though right now, this crate is very unstable and breaks all the time.

## Documentation

[Read documentation](https://docs.rs/korome/) (it's lacking quite a bit at the crate level right now)

## Simple Example

```rust
#[macro_use]
extern crate korome;

use korome::*;

fn main() {
    // Create a Graphics object, which creates a window with the given title and dimensions
    let graphics = Graphics::new("Example!", 800, 600).unwrap();

    // Load a texture, whose bytes have been loaded at compile-time
    let texture = include_texture!(graphics, "assets/planet.png").unwrap();

    // You can also parse other things than just a closure
    // See the documentation for `run_until_closed` and the `Game` trait
    run_until_closed(graphics, |_: &FrameInfo, drawer: &mut Drawer| {
        drawer.clear(0.1, 0., 1.);
        texture.drawer().draw(drawer);
    })
}
```

For more examples look in the examples.
