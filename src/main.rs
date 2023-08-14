mod grow;

use std::f32::consts::PI;

use grow::Branch;
use macroquad::prelude::*;

fn window_config() -> Conf {
    Conf {
        window_title: "forking_turtle".into(),
        window_width: 1000,
        window_height: 800,
        // sample_count: 2,
        ..Default::default()
    }
}

#[macroquad::main(window_config)]
async fn main() {
    let mut tree = Branch {
        dir: Vec2::from_angle(PI),
        length: 0.,
        subbranches: vec![],
        color: BLACK,
        leaf: false,
        grow_rate: 500.,
        width: 1.,
    };

    // let zoom = vec2(1. / screen_width(), -1. / screen_height());

    // let persistent = {
    //     let rt = render_target(screen_width() as u32 + 1, screen_height() as u32 + 1);
    //     Camera2D {
    //         zoom,
    //         render_target: Some(rt),
    //         ..Default::default()
    //     }
    // };
    // set_camera(&persistent);
    // clear_background(Color::from_hex(0x7ac6dd));

    // let transient = {
    //     Camera2D {
    //         zoom,
    //         ..Default::default()
    //     }
    // };

    loop {
        // set_camera(&persistent);

        let dt = get_frame_time();

        tree.update(dt, &mut 10000.);

        // set_default_camera();

        // clear_background(PINK);

        clear_background(Color::from_hex(0x7ac6dd));

        // draw_texture(
        //     &persistent.render_target.as_ref().unwrap().texture,
        //     0.,
        //     0.,
        //     WHITE,
        // );

        // set_camera(&transient);

        tree.draw(vec2(0.5 * screen_width(), screen_height()));

        next_frame().await
    }
}
