#[macro_use]
extern crate korome;

use korome::*;
use std::rc::Rc;

fn main() {
    let graphics = Graphics::new("Menu example!", 800, 600);

    let menu_state = Menu {
        planet: Rc::new(include_texture!(graphics, "assets/planet.png").unwrap()),
        start_game: include_texture!(graphics, "assets/start_game.png").unwrap()
    };

    let mut gm = GameManager::new(graphics);
    gm.run(Box::new(menu_state));
}

use korome::StateAction::*;

impl State for Menu{
    fn frame(&mut self, closed: bool, info: FrameInfo, mut drawer: Drawer) -> StateAction{
        if closed{
            Close
        }else{
            drawer.clear(0., 0., 0.);
            drawer.draw_texture_rigid(&self.start_game, 0., 0.).unwrap();

            for &e in info.get_mouse_events(){
                if let (true, MouseButton::Left) = e{
                    if info.mousepos.0 <= 100. && info.mousepos.0 >= -100. && info.mousepos.1 <= 25. && info.mousepos.1 >= -25.{
                        return ChangeTo(Box::new(Game{
                            planet: self.planet.clone(),
                            pos: Vector2(0., 0.)
                        }))
                    }
                }
            }
            Continue
        }
    }
}

struct Menu{
    planet    : Rc<Texture>,
    start_game: Texture
}

struct Game{
    planet: Rc<Texture>,
    pos: Vector2<f32>,
}

impl State for Game{
    fn frame(&mut self, closed: bool, info: FrameInfo, mut drawer: Drawer) -> StateAction{
        if closed{
            Close
        }else{
            drawer.clear(0., 0., 0.);
            drawer.draw_texture_rigid(&self.planet, self.pos.0, self.pos.1).unwrap();

            let vel = 100. * info.delta as f32;

            is_down!{info;
                A, Left => {
                    self.pos.0 -= vel
                },
                D, Right => {
                    self.pos.0 += vel
                },
                S, Down => {
                    self.pos.1 -= vel
                },
                W, Up => {
                    self.pos.1 += vel
                }
            }

            Continue
        }
    }
}
