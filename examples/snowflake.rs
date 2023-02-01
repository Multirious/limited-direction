//! Visualize multiple angle at once
//!
//! Key W: Extend length
//! Key S: Shrink length
//! Key A: Extend offset
//! Key D: Shrink offset
//! Key Q: Decrease amount of angle by 1
//! Key E: Increase amount of angle by 1
//! Key Z: Show/Hide expected angle
//! Key N: Cycle amount of directions

use std::f64::consts::TAU;

use macroquad::{
    input::{is_key_pressed, KeyCode},
    prelude::*,
};
use rigid_walk::{RigidWalk, WalkAct};

const BG: Color = color_u8!(30, 30, 40, 255);

#[macroquad::main("Rigid Walk Snowflake")]
async fn main() {
    enum Algo {
        Walk4,
        Walk8,
    }

    impl Algo {
        fn cycle(&self) -> Algo {
            match self {
                Algo::Walk4 => Algo::Walk8,
                Algo::Walk8 => Algo::Walk4,
            }
        }
    }

    let change_length_speed = 1.0f64;
    let change_offset_speed = 0.5f64;

    let mut angles = 360 / 10u32;
    let mut show_angle = true;
    let mut length = 250.0f64;
    let mut offset = 10.0f64;
    let mut algorithm = Algo::Walk8;

    loop {
        clear_background(BG);

        let key_w = is_key_down(KeyCode::W);
        let key_s = is_key_down(KeyCode::S);
        let key_a = is_key_down(KeyCode::A);
        let key_d = is_key_down(KeyCode::D);
        let key_q = is_key_pressed(KeyCode::Q);
        let key_e = is_key_pressed(KeyCode::E);
        let key_z = is_key_pressed(KeyCode::Z);
        let key_n = is_key_pressed(KeyCode::N);
        if key_w {
            length += change_length_speed;
        }
        if key_s {
            length -= change_length_speed;
        }
        if key_a {
            offset -= change_offset_speed;
            // This thing get laggy when small
            if offset < 0.5 {
                offset = 0.5;
            }
        }
        if key_d {
            offset += change_offset_speed;
        }
        if key_q {
            angles -= 1;
            if angles == 0 {
                angles = 1;
            }
        }
        if key_e {
            angles += 1;
        }
        if key_z {
            show_angle = !show_angle;
        }
        if key_n {
            algorithm = algorithm.cycle();
        }

        let mid_x = screen_width() / 2.0;
        let mid_y = screen_height() / 2.0;

        let diff = TAU / (angles as f64);

        if show_angle {
            let mut curr_angle = 0.0f64;
            for _ in 0..angles {
                draw_line_angle(mid_x, mid_y, curr_angle, length, 1.0, GRAY);
                curr_angle += diff;
            }
        }
        let mut curr_angle = 0.0f64;
        for _ in 0..angles {
            let walk = match algorithm {
                Algo::Walk4 => RigidWalk::walk4(curr_angle, length, offset),
                Algo::Walk8 => RigidWalk::walk8(curr_angle, length, offset),
            };
            draw_walk(mid_x, mid_y, &walk);
            curr_angle += diff;
        }

        next_frame().await
    }
}

fn draw_walk(x: f32, y: f32, walk: &RigidWalk) {
    let mut turtle_x = x as f64;
    let mut turtle_y = y as f64;
    let color = [RED, GREEN, BLUE];

    for (i, WalkAct { angle, distance }) in walk.iter_full(false).enumerate() {
        let x1 = turtle_x;
        let y1 = turtle_y;
        let x2 = x1 + angle.sin() * distance;
        let y2 = y1 + angle.cos() * distance;
        turtle_x += angle.sin() * distance;
        turtle_y += angle.cos() * distance;
        draw_line(
            x1 as f32,
            y1 as f32,
            x2 as f32,
            y2 as f32,
            1.0,
            color[i % color.len()],
        );
    }
}

fn draw_line_angle(x: f32, y: f32, angle: f64, length: f64, thickness: f32, color: Color) {
    draw_line(
        x,
        y,
        x + (angle.sin() * length) as f32,
        y + (angle.cos() * length) as f32,
        thickness,
        color,
    );
}
