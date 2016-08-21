#[macro_use]
extern crate korome;

use korome::*;

use State::*;
enum State {
    Menu, InGame
}

fn main() {
    let graphics = Graphics::new("Menu example!", 800, 600).unwrap();

    let planet = Texture::from_file(&graphics, "examples/assets/planet.png").unwrap();
    let start_game = Texture::from_file(&graphics, "examples/assets/start_game.png").unwrap();
    let quit_game = Texture::from_file(&graphics, "examples/assets/quit_game.png").unwrap();

    let mut state = Menu;
    let mut pos = (0., 0.);

    run_until_closed(graphics, |info: &FrameInfo, drawer: &mut Drawer|{
        drawer.clear(0., 0., 0.);

        match state{
            Menu => {
                start_game.drawer().pos((0., 35.)).draw(drawer);
                quit_game.drawer().pos((0., -35.)).draw(drawer);

                for &e in info.get_mouse_events(){
                    if let (true, MouseButton::Left) = e{
                        match info.mousepos{
                            (-100. ... 100., 10. ... 65.) => state = InGame,
                            (-100. ... 100., -65. ... -10.) => return GameUpdate::Close,
                            _ => ()
                        }
                    }
                }
            },
            InGame => {
                planet.drawer().pos(pos.into()).draw(drawer);

                let vel = 100. * info.delta;

                is_down!{info;
                    A, Left => {
                        pos.0 -= vel
                    },
                    D, Right => {
                        pos.0 += vel
                    },
                    S, Down => {
                        pos.1 -= vel
                    },
                    W, Up => {
                        pos.1 += vel
                    }
                }
            }
        }
        GameUpdate::Nothing
    });
}
