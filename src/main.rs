use std::ops::{Deref, DerefMut};

use macroquad::prelude::*;
use rand::gen_range;

#[macroquad::main("Sun")]
async fn main() {
    let planet = MassiveBody::new(vec2(1.0, 0.0), vec2(0.0, 3.0), 0.1, 1.0, BLUE);

    let sun = MassiveBody::new(vec2(0.0, 0.0), vec2(0.0, 0.0), 0.5, 5.0, YELLOW);

    let mut massive_bodies = vec![sun, planet];
    let mut massless_bodies: Vec<Body> = vec![];

    let scale = 100.0;

    let delta_t = 0.01;
    let mut t = 0.0;

    loop {
        if is_mouse_button_down(MouseButton::Left) {
            let (x, y) = mouse_position();

            let pos = vec2(x - screen_width() / 2.0, screen_height() / 2.0 - y) / scale;
            massless_bodies.push(Body::new(
                pos,
                vec2(gen_range(-1.0, 1.0), gen_range(-1.0, 1.0)),
                0.02,
                RED,
            ));
        }

        clear_background(BLACK);

        for body in &mut massive_bodies {
            body.body.pos += body.vel * delta_t / 2.0;
        }

        for body in &mut massless_bodies {
            body.pos += body.vel * delta_t / 2.0;
        }

        for body in &mut massive_bodies {
            body.force = vec2(0.0, 0.0);
        }
        for body in &mut massless_bodies {
            body.force = vec2(0.0, 0.0);
        }

        for i in 0..(massive_bodies.len() - 1) {
            for j in (i + 1)..massive_bodies.len() {
                let dist_sq =
                    (massive_bodies[i].body.pos - massive_bodies[j].body.pos).length_squared();
                let force_mag =
                    massive_bodies[i].mass * massive_bodies[j].mass / dist_sq.max(0.0001);

                let force_dir =
                    (massive_bodies[i].body.pos - massive_bodies[j].body.pos).normalize();

                let force = force_mag * force_dir;
                massive_bodies[j].body.force += force;
                massive_bodies[i].body.force -= force;
            }
        }

        for massless in &mut massless_bodies {
            for massive in &massive_bodies {
                let dist_sq = (massive.body.pos - massless.pos).length_squared();
                let force_mag = massive.mass / dist_sq.max(0.0001);

                let force_dir = (massive.body.pos - massless.pos).normalize();

                let force = force_mag * force_dir;
                massless.force += force;
            }
        }

        for body in &mut massive_bodies {
            body.body.vel += body.body.force / body.mass * delta_t;
        }
        for body in &mut massless_bodies {
            body.vel += body.force * delta_t;
        }

        for body in &mut massive_bodies {
            body.body.pos += body.body.vel * delta_t / 2.0;
        }

        for body in &mut massless_bodies {
            body.pos += body.vel * delta_t / 2.0;
        }

        let pos = massive_bodies[0].pos;

        for body in &mut massive_bodies {
            body.pos -= pos;
        }
        for body in &mut massless_bodies {
            body.pos -= pos;
        }

        massless_bodies = massless_bodies
            .into_iter()
            .filter(|i| i.pos.x.abs() <= 5.0 && i.pos.y.abs() <= 5.0)
            .collect();

        t += delta_t;

        for body in &massive_bodies {
            body.draw(scale);
        }
        for body in &massless_bodies {
            body.draw(scale);
        }

        draw_text(&format!("{}", t), 0.0, 20.0, 20.0, WHITE);
        draw_text(&format!("{}", get_fps()), 0.0, 40.0, 20.0, WHITE);

        next_frame().await
    }
}

struct Body {
    pos: Vec2,
    vel: Vec2,
    radius: f32,
    color: Color,
    force: Vec2,
}

impl Body {
    fn new(pos: Vec2, vel: Vec2, radius: f32, color: Color) -> Self {
        Self {
            pos,
            vel,
            radius,
            color,
            force: vec2(0.0, 0.0),
        }
    }

    fn draw(&self, scale: f32) {
        let r = self.radius * scale;
        let x = self.pos.x * scale + screen_width() / 2.0;
        let y = -self.pos.y * scale + screen_height() / 2.0;
        draw_circle(x, y, r, self.color);
        // draw_line(x, y, x + self.force.x * scale, y - self.force.y * scale, 4.0, WHITE);
    }
}

struct MassiveBody {
    body: Body,
    mass: f32,
}

impl Deref for MassiveBody {
    type Target = Body;

    fn deref(&self) -> &Self::Target {
        &self.body
    }
}

impl DerefMut for MassiveBody {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.body
    }
}

impl MassiveBody {
    fn new(pos: Vec2, vel: Vec2, radius: f32, mass: f32, color: Color) -> Self {
        Self {
            body: Body::new(pos, vel, radius, color),
            mass,
        }
    }
}

// struct Wold {
//     planet_pos: Vec2,
//     planet_vel: Vec2,
// }

// impl World {
//     fn new() ->
// }
