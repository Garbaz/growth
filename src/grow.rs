use std::f32::consts::PI;

use ::rand::{random, seq::SliceRandom, thread_rng, Rng};
use macroquad::prelude::*;

#[derive(Debug, Clone)]
pub struct Branch {
    // pub pos: Vec2,
    pub dir: Vec2,
    pub length: f32,
    pub subbranches: Vec<Branch>,
    pub color: Color,
    pub leaf: bool,
    pub grow_rate: f32,
    pub width: f32,
}

const BRANCH_MIN_ANGLE: f32 = 0.00 * PI;
const BRANCH_MAX_ANGLE: f32 = 0.1 * PI;

const PROB_LEAF_DENSITY: f32 = 0.3;

const PROB_FORK_DENSITY: f32 = 0.8;

// const FORK_WIDTH_FACTOR: f32 = 0.8;

const WIDTH_LENGTH_DISTR: f32 = 0.1;

// TODO: These should both derive from constant, that being the cell volume!
const LENGTH_JUICE_FACTOR: f32 = 7.;
const WIDTH_JUICE_FACTOR: f32 = 5.;

// const FORK_MAX_JUICE_FACTOR: f32 = 100.;

const FORK_GROWTH_RATE_FACTOR: f32 = 1.1;

impl Branch {
    pub fn update(&mut self, dt: f32, juice: &mut f32) {
        if !self.leaf {
            let j = (dt * self.grow_rate).min(*juice);
            *juice -= j;
            let jl = (1. - WIDTH_LENGTH_DISTR) * j;
            let jw = WIDTH_LENGTH_DISTR * j;
            self.width += (WIDTH_JUICE_FACTOR * jw + self.width * self.width).sqrt() - self.width;
            if self.width > 0. {
                self.length += LENGTH_JUICE_FACTOR * jl / (self.width * self.width);
            }
        }

        // println!("{}", *juice);

        if *juice > 0. && random::<f32>() < self.prob_fork(dt, *juice) {
            let mut angle = PI * thread_rng().gen_range(BRANCH_MIN_ANGLE..=BRANCH_MAX_ANGLE);
            if random::<bool>() {
                angle *= -1.
            };
            let dir = Vec2::from_angle(angle);
            // if dir.y < 0. && random::<f32>() < 0.5 {
            //     dir.y *= -1.
            // };
            let leaf = random::<f32>() < dbg!(self.prob_leaf(dt, *juice));
            // let pos = self.pos + self.length * self.dir;
            self.subbranches.push(Branch {
                // pos,
                dir,
                length: 0.,
                subbranches: vec![],
                color: self.color,
                leaf,
                grow_rate: self.grow_rate * FORK_GROWTH_RATE_FACTOR,
                width: 1.,
            })
        }
        let mut subbranches = self.subbranches.iter_mut().collect::<Vec<_>>();
        subbranches.shuffle(&mut thread_rng());
        for b in subbranches {
            b.update(dt, juice);
        }
    }

    fn prob_fork(&self, dt: f32, juice: f32) -> f32 {
        PROB_FORK_DENSITY
            * dt
            * if juice < 10. * dt * self.grow_rate {
                // println!("!!");
                0.01
            } else {
                1.
            }
    }

    fn prob_leaf(&self, dt: f32, juice: f32) -> f32 {
        PROB_LEAF_DENSITY
            * dt
            * if juice < 10. * dt * self.grow_rate {
                100000.
            } else {
                1.
            }
    }

    pub fn draw(&self, pos: Vec2) {
        self.draw_(pos, vec2(0., 1.))
    }

    fn draw_(&self, offset: Vec2, dir: Vec2) {
        let from = /* self.pos + */ offset;
        let dir = self.dir.rotate(dir);
        let to = from + self.length * dir;

        // println!("from: {}\nto:   {}\ndir:  {}", from, to, dir);

        // draw_circle(dir.x, dir.y, 4., RED);

        if self.leaf {
            draw_circle(to.x, to.y, 6., GREEN);
        }

        draw_line(from.x, from.y, to.x, to.y, self.width, self.color);

        for b in &self.subbranches {
            b.draw_(to, dir);
        }
    }
}

// pub trait Grow {
//     const SPEED: f32;

//     fn color(&self) -> Color;

//     fn update(&mut self, dt: f32);

//     fn draw(&self);
// }

// pub struct Stem {
//     pos: Vec2,
//     dir: Vec2,
// }

// impl Grow for Stem {
//     const SPEED: f32 = 100.0;

//     fn color(&self) -> Color {
//         Color::from_hex(0x2b241a)
//     }

//     fn update(&mut self, dt: f32) {
//         self.pos += dt * Self::SPEED * self.dir;
//     }

//     fn draw(&self) {
//         todo!()
//     }
// }
