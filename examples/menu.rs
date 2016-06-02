#[macro_use]
extern crate korome;

use korome::*;

use State::*;
enum State {
    MainMenu, MainGame
}

fn main() {
    let graphics = Graphics::new("Menu example!", 800, 600).unwrap();

    let planet = include_texture!(graphics, "assets/planet.png").unwrap();
    let start_game = include_texture!(graphics, "assets/start_game.png").unwrap();
    let quit_game = include_texture!(graphics, "assets/quit_game.png").unwrap();

    let mut state = MainMenu;
    let mut pos = Vector2(0., 0.);

    run_until_closed(graphics, |info: FrameInfo, mut drawer: Drawer|{
        drawer.clear(0., 0., 0.);

        match state{
            MainMenu => {
                drawer.texture(&start_game).pos((0., 35.)).draw();
                drawer.texture(&quit_game).pos((0., -35.)).draw();

                for &e in info.get_mouse_events(){
                    if let (true, MouseButton::Left) = e{
                        match info.mousepos{
                            (-100. ... 100., 10. ... 65.) => state = MainGame,
                            (-100. ... 100., -65. ... -10.) => return GameUpdate::nothing().set_close(true),
                            _ => ()
                        }
                    }
                }
            },
            MainGame => {
                drawer.texture(&planet).pos(pos.into()).draw();

                let vel = 100. * info.delta as f32;

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
        GameUpdate::nothing()
    });
}
