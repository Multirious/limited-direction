use std::f64::consts::{PI, TAU};

use macroquad::{
    input::{is_key_down, KeyCode},
    prelude::*,
};
use rigid_walk::{RigidWalk, WalkAct};

const BG: Color = color_u8!(30, 30, 40, 255);

#[macroquad::main("Rigid Walk Visualizer")]
async fn main() {
    let change_angle_speed = PI / 500.0;
    let change_distance_speed = 1.0f64;
    let change_offset_speed = 0.5f64;

    let mut angle = 0.0f64;
    let mut distance = 300.0f64;
    let mut offset = 10.0f64;
    loop {
        clear_background(BG);

        // if is_key_down(KeyCode::Q) {
        angle += change_angle_speed;
        angle = angle % TAU;
        // }
        // if is_key_down(KeyCode::E) {
        //     angle -= change_angle_speed;
        //     angle = angle % TAU;
        // }
        if is_key_down(KeyCode::W) {
            distance += change_distance_speed;
        }
        if is_key_down(KeyCode::S) {
            distance -= change_distance_speed;
        }
        if is_key_down(KeyCode::A) {
            offset -= change_offset_speed;
            // This thing get laggy when small
            if offset < 0.5 {
                offset = 0.5;
            }
        }
        if is_key_down(KeyCode::D) {
            offset += change_offset_speed;
        }

        let mid_x = screen_width() / 2.0;
        let mid_y = screen_height() / 2.0;

        draw_stump_line_angle(mid_x, mid_y, angle, distance, 2.0, GRAY);

        // offset
        draw_offset(
            mid_x,
            mid_y,
            angle,
            offset,
            distance,
            color_u8!(75, 75, 75, 255),
        );

        let walk = RigidWalk::walk8(angle, distance, offset);
        draw_walk(mid_x, mid_y, &walk);

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
        draw_stump_line(
            x1 as f32,
            y1 as f32,
            x2 as f32,
            y2 as f32,
            5.0,
            color[i % color.len()],
        );
    }
}

fn draw_offset(x: f32, y: f32, angle: f64, offset: f64, length: f64, color: Color) {
    draw_stump_dotted_line_angle(
        x + ((PI - angle).cos() * offset) as f32,
        y + ((PI - angle).sin() * offset) as f32,
        angle,
        length,
        2.0,
        5.0,
        color,
    );
    draw_stump_dotted_line_angle(
        x - ((PI - angle).cos() * offset) as f32,
        y - ((PI - angle).sin() * offset) as f32,
        angle,
        length,
        2.0,
        5.0,
        color,
    );
}

fn draw_stump_dotted_line(
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    dot_length: f32,
    thickness: f32,
    color: Color,
) {
    let mut switch = true;
    let height = y2 - y1;
    let base = x2 - x1;
    let length = (height * height + base * base).sqrt();
    let dot_height = height * dot_length / length;
    let dot_base = base * dot_length / length;
    let mut x = x1;
    let mut y = y1;
    for _ in 0..((length / dot_length) as u32) {
        let x1 = x;
        let y1 = y;
        let x2 = x + dot_height;
        let y2 = y + dot_base;
        if switch {
            draw_stump_line(x1, y1, x2, y2, thickness, color);
        }
        x += dot_height;
        y += dot_base;
        switch = !switch;
    }
}

fn draw_stump_dotted_line_angle(
    x: f32,
    y: f32,
    angle: f64,
    length: f64,
    thickness: f32,
    dot_length: f32,
    color: Color,
) {
    draw_stump_dotted_line(
        x,
        y,
        x + (angle.cos() * length) as f32,
        y + (angle.sin() * length) as f32,
        dot_length,
        thickness,
        color,
    )
}

fn draw_stump_line_angle(x: f32, y: f32, angle: f64, length: f64, thickness: f32, color: Color) {
    draw_stump_line(
        x,
        y,
        x + (angle.sin() * length) as f32,
        y + (angle.cos() * length) as f32,
        thickness,
        color,
    );
}

fn draw_stump_line(x1: f32, y1: f32, x2: f32, y2: f32, thickness: f32, color: Color) {
    draw_circle(x1, y1, thickness / 2.0, color);
    draw_circle(x2, y2, thickness / 2.0, color);
    draw_line(x1, y1, x2, y2, thickness, color);
}
