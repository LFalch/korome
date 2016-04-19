#[macro_use]
extern crate korome;

use korome::*;
use korome::StateAction::*;

use std::rc::Rc;

fn main() {
    let graphics = Graphics::new("Menu example!", 800, 600).unwrap();

    let planet = Rc::new(include_texture!(graphics, "assets/planet.png").unwrap());
    let start_game = include_texture!(graphics, "assets/start_game.png").unwrap();

    let mut gm = GameManager::new(graphics);

    gm.run(Box::new(move|closed, info: FrameInfo, mut drawer: Drawer|{
        if closed{
            Close
        }else{
            drawer.clear(0., 0., 0.);
            drawer.draw_texture_rigid(&start_game, 0., 0.).unwrap();

            for &e in info.get_mouse_events(){
                if let (true, MouseButton::Left) = e{
                    if info.mousepos.0 <= 100. && info.mousepos.0 >= -100. && info.mousepos.1 <= 25. && info.mousepos.1 >= -25.{
                        let mut pos = Vector2(0., 0.);
                        let pl = planet.clone();

                        return ChangeTo(Box::new(move|closed, info: FrameInfo, mut drawer: Drawer|{
                            if closed{
                                Close
                            }else{
                                drawer.clear(0., 0., 0.);
                                drawer.draw_texture_rigid(&pl, pos.0, pos.1).unwrap();

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

                                Continue
                            }
                        }))
                    }
                }
            }
            Continue
        }
    }));
}
