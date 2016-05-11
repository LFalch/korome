#[macro_use]
extern crate korome;

use korome::*;

fn main() {
    let graphics = Graphics::new("Menu example!", 800, 600).unwrap();

    let planet = include_texture!(graphics, "assets/planet.png").unwrap();
    let start_game = include_texture!(graphics, "assets/start_game.png").unwrap();

    let gm = GameManager::new(graphics);
    let mut state = 0;
    let mut pos = Vector2(0., 0.);

    gm.run_until_closed(|info: FrameInfo, mut drawer: Drawer|{
        drawer.clear(0., 0., 0.);

        match state{
            0 => {
                drawer.draw_texture_rigid(&start_game, 0., 0.).unwrap();

                for &e in info.get_mouse_events(){
                    if let ((true, MouseButton::Left), (-100. ... 100., -25. ... 25.)) = (e, info.mousepos){
                        state = 1;
                    }
                }
            },
            1 => {
                drawer.draw_texture_rigid(&planet, pos.0, pos.1).unwrap();

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
            },
            _ => unimplemented!()
        }
        GameUpdate::nothing()
    });
}
