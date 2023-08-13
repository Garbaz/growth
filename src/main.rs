mod grow;
mod turtle;

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
    // let rotation_speed = 0.33 * PI;
    // let mut a = 0f32;

    // let mut turtles = vec![Turtle::new(vec2(0., -screen_height()), vec2(0., 1.)); 10];

    let mut tree = Branch {
        // pos: vec2(0., -screen_height()),
        dir: Vec2::from_angle(0.),
        length: 0.,
        subbranches: vec![],
        color: BLACK,
        leaf: false,
        grow_rate: 100.,
        width: 1.,
    };

    let zoom = /* (2. / 3.) * */ vec2(1. / screen_width(), -1. / screen_height());

    let persistent = {
        let rt = render_target(screen_width() as u32 + 1, screen_height() as u32 + 1);
        Camera2D {
            zoom,
            render_target: Some(rt),
            ..Default::default()
        }
    };
    set_camera(&persistent);
    clear_background(Color::from_hex(0x7ac6dd));

    let transient = {
        Camera2D {
            zoom,
            ..Default::default()
        }
    };
    loop {
        set_camera(&persistent);

        let dt = get_frame_time();

        // let x = 1000. * a.sin();
        // let y = 1000. * a.cos();
        // a += dt * rotation_speed;

        // draw_line(
        //     // 0.5 * screen_width(),
        //     // 0.5 * screen_height(),
        //     // 0.5 * screen_width() + x,
        //     // 0.5 * screen_height() + y,
        //     0., 0., x, y, 4., BLUE,
        // );

        // println!("{:?}", turtles);

        // turtles = turtles
        //     .into_iter()
        //     .flat_map(|mut t| match t.persistent(dt) {
        //         UpdateEffect::Death => {
        //             vec![]
        //         }
        //         UpdateEffect::None => {
        //             vec![t]
        //         }
        //         UpdateEffect::NewTurtle(u) => {
        //             vec![t, u]
        //         }
        //     })
        //     .collect();

        tree.update(dt, &mut 50.);

        set_default_camera();

        clear_background(PINK);

        draw_texture(
            &persistent.render_target.as_ref().unwrap().texture,
            0.,
            0.,
            WHITE,
        );

        set_camera(&transient);

        tree.draw(vec2(0., -screen_height()));

        // for t in &turtles {
        //     t.transient();
        // }

        // clear_background(RED);

        // draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        // draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        // draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);
        // draw_text("HELLO", 20.0, 20.0, 20.0, DARKGRAY);

        next_frame().await
    }
}
