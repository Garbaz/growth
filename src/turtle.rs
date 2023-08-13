use std::f32::consts::PI;

use macroquad::prelude::*;

#[derive(Clone, Debug)]
pub struct Turtle {
    pos: Vec2,
    dir: Vec2,
    kind: Kind,
}

#[derive(Clone, Debug)]
enum Kind {
    Stalk,
    Leaf,
}

impl Kind {
    fn color(&self) -> Color {
        Color::from_hex(match self {
            Kind::Stalk => 0x1d3804,
            Kind::Leaf => 0x4da80d,
        })
    }
}

enum Decision {
    Step,
    TurnLeft,
    TurnRight,
    Fork,
    Die,
}

impl Decision {
    fn random(p_step: f32, p_turn_left: f32, p_turn_right: f32, p_fork: f32, p_die: f32) -> Self {
        let sum = p_step + p_turn_left + p_turn_right + p_fork + p_die;
        let mut r = rand::gen_range(0., sum);
        if {
            r -= p_step;
            r
        } < 0.
        {
            Decision::Step
        } else if {
            r -= p_turn_left;
            r
        } < 0.
        {
            Decision::TurnLeft
        } else if {
            r -= p_turn_right;
            r
        } < 0.
        {
            Decision::TurnRight
        } else if {
            r -= p_fork;
            r
        } < 0.
        {
            Decision::Fork
        } else {
            Decision::Die
        }
    }
}

const STEP_SPEED: f32 = 50.;
const TURN_SPEED: f32 = 0.25 * PI;

pub enum UpdateEffect {
    None,
    NewTurtle(Turtle),
    Death,
}

impl Turtle {
    pub fn new(pos: Vec2, dir: Vec2) -> Self {
        Self {
            pos,
            dir: dir.normalize(),
            kind: Kind::Stalk,
        }
    }

    pub fn persistent(&mut self, dt: f32) -> UpdateEffect {
        match match self.kind {
            Kind::Stalk => Decision::random(90., 5., 5., 1., 1.),
            Kind::Leaf => Decision::random(50., 10., 10., 5., 10.),
        } {
            Decision::Step => {
                let new_pos = self.pos + dt * STEP_SPEED * self.dir;
                draw_line(
                    self.pos.x,
                    self.pos.y,
                    new_pos.x,
                    new_pos.y,
                    if let Kind::Stalk = self.kind { 8. } else { 10. },
                    self.kind.color(),
                );
                self.pos = new_pos;
            }
            Decision::TurnLeft => self.dir = Vec2::from_angle(dt * TURN_SPEED).rotate(self.dir),
            Decision::TurnRight => self.dir = Vec2::from_angle(-dt * TURN_SPEED).rotate(self.dir),
            Decision::Fork => {
                let mut new_turtle = self.clone();
                // if rand::gen_range(0., 1.) < 0.05 {
                //     new_turtle.kind = Kind::Leaf;
                // }
                new_turtle.dir = Vec2::from_angle(0.1 * PI * if new_turtle.dir.x > 0. {1.} else {-1.}).rotate(new_turtle.dir);
                return UpdateEffect::NewTurtle(new_turtle);
            }
            Decision::Die => return UpdateEffect::Death,
        }
        UpdateEffect::None
    }

    pub fn transient(&self) {
        match self.kind {
            Kind::Stalk => draw_circle(self.pos.x, self.pos.y, 3., self.kind.color()),
            Kind::Leaf => {}
        }
    }
}
