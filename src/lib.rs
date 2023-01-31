use turtle::{Color, Turtle};

/// Meaning of:
///  - Primary is the straight walking part
///  - Secondary is the diagonal walking part
#[derive(Debug)]
pub struct Walk8 {
    primary_angle: f64,
    secondary_angle: f64,

    secondary_segment_distance: f64,
    primary_segment_distance: f64,

    times: u32,

    last_primary_distance: f64,
    last_secondary_distance: f64,
}

impl Walk8 {
    /// `angle` angle of direction to acheive in radian
    /// `displacement` is the displacement to achieve.
    /// `offset` is amount of distance possible tangent offset to main "line".
    pub fn from_angle(angle: f64, displacement: f64, offset: f64) -> Walk8 {
        fn nearest_multiple(n: f64, multiple: f64) -> f64 {
            let mut result = n.abs() + multiple / 2.0;
            result -= result % multiple;
            result *= if n > 0.0 { 1.0 } else { -1.0 };
            result
        }

        use std::f64::consts::{FRAC_PI_2, FRAC_PI_4, TAU};

        let angle = angle % TAU;
        let primary_angle = nearest_multiple(angle, FRAC_PI_2);
        let secondary_angle = nearest_multiple(angle - FRAC_PI_4, FRAC_PI_2) + FRAC_PI_4;

        let primary_angle_rel = (primary_angle - angle).abs();
        let secondary_angle_rel = (secondary_angle - angle).abs();

        let primary_segment_distance = offset / primary_angle_rel.sin();
        let secondary_segment_distance = offset / secondary_angle_rel.sin();
        let segment_displacment =
            (offset / primary_angle_rel.tan()) + (offset / secondary_angle_rel.tan());

        let times = (displacement / segment_displacment).floor();

        let displacement_left = displacement - (segment_displacment * times);

        let last_offset = (offset * displacement_left) / segment_displacment;
        let last_primary_distance = last_offset / primary_angle_rel.sin();
        let last_secondary_distance = last_offset / secondary_angle_rel.sin();

        Walk8 {
            primary_angle,
            secondary_angle,
            secondary_segment_distance,
            primary_segment_distance,
            times: times as u32,
            last_primary_distance,
            last_secondary_distance,
        }
    }

    pub fn total_distance(&self) -> f64 {
        let Walk8 {
            secondary_segment_distance,
            primary_segment_distance,
            times,
            last_primary_distance,
            last_secondary_distance,
            ..
        } = self;
        (primary_segment_distance + secondary_segment_distance) * (*times as f64)
            + last_primary_distance
            + last_secondary_distance
    }

    /// `start_primary` if you wanted to start algorithm at primary angle
    pub fn iter_full(&self, start_primary: bool) -> Walk8IterFull<'_> {
        Walk8IterFull::new(self, start_primary)
    }
}

enum WalkIterState {
    MainStart,
    MainMiddle,
    MainEnd,
    // LastStart,
    // LastStart is in MainEnd
    LastEnd,
    Stop,
}

pub struct Walk8IterFull<'a> {
    walk: &'a Walk8,
    state: WalkIterState,
    start_primary: bool,
    switch: bool,
    n_left: u32,
}

impl<'a> Walk8IterFull<'a> {
    /// `start_primary` if you wanted to start algorithm at primary angle
    pub fn new(walk: &'a Walk8, start_primary: bool) -> Walk8IterFull<'a> {
        Walk8IterFull {
            n_left: walk.times.saturating_sub(1),
            switch: start_primary,
            start_primary,
            walk,
            state: WalkIterState::MainStart,
        }
    }
}

impl<'a> Iterator for Walk8IterFull<'a> {
    type Item = WalkAct;

    fn next(&mut self) -> Option<Self::Item> {
        macro_rules! value {
            (secondary) => {
                (
                    self.walk.secondary_angle,
                    self.walk.secondary_segment_distance,
                )
            };
            (primary) => {
                (self.walk.primary_angle, self.walk.primary_segment_distance)
            };
            (last_secondary) => {
                (self.walk.secondary_angle, self.walk.last_secondary_distance)
            };
            (last_primary) => {
                (self.walk.primary_angle, self.walk.last_primary_distance)
            };
            (secondary_double) => {
                (
                    self.walk.secondary_angle,
                    self.walk.secondary_segment_distance * 2.0,
                )
            };
            (primary_double) => {
                (
                    self.walk.primary_angle,
                    self.walk.primary_segment_distance * 2.0,
                )
            };
            (primary_add_last) => {
                (
                    self.walk.primary_angle,
                    self.walk.primary_segment_distance + self.walk.last_primary_distance,
                )
            };
            (secondary_add_last) => {
                (
                    self.walk.secondary_angle,
                    self.walk.secondary_segment_distance + self.walk.last_secondary_distance,
                )
            };
        }
        let (angle, distance) = match self.state {
            WalkIterState::MainStart => {
                self.state = WalkIterState::MainMiddle;
                if self.start_primary {
                    value!(primary)
                } else {
                    value!(secondary)
                }
            }
            WalkIterState::MainMiddle => {
                self.n_left = match self.n_left.checked_sub(1) {
                    Some(n) => n,
                    None => {
                        self.state = WalkIterState::MainEnd;
                        return self.next();
                    }
                };
                let switch = self.switch;
                self.switch = !self.switch;
                if switch {
                    value!(secondary_double)
                } else {
                    value!(primary_double)
                }
            }
            WalkIterState::MainEnd => {
                self.state = WalkIterState::LastEnd;
                if self.switch {
                    value!(secondary_add_last)
                } else {
                    value!(primary_add_last)
                }
            }
            WalkIterState::LastEnd => {
                self.state = WalkIterState::Stop;
                if self.switch {
                    value!(last_primary)
                } else {
                    value!(last_secondary)
                }
            }
            WalkIterState::Stop => return None,
        };
        Some(WalkAct { angle, distance })
    }
}

pub struct WalkAct {
    pub angle: f64,
    pub distance: f64,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn visualize() {
        let mut turtle = Turtle::new();
        let displacement = 300.0;

        turtle.set_speed("instant");
        turtle.use_radians();
        for angle in (0..360).step_by(1) {
            let angle = (angle as f64).to_radians();
            turtle.set_heading(angle);
            turtle.pen_down();
            turtle.set_pen_color(Color::rgb(200., 200., 200.));
            turtle.forward(displacement);
            turtle.home();

            let walk = Walk8::from_angle(angle, displacement, 1.);
            render_walk(&mut turtle, walk);
            turtle.pen_up();
            turtle.home();
            // turtle.clear()
        }
    }

    fn render_walk(turtle: &mut Turtle, walk: Walk8) {
        let colors = ["red", "green", "blue"];
        for (i, WalkAct { angle, distance }) in Walk8IterFull::new(&walk, false).enumerate() {
            turtle.set_pen_color(colors[i % colors.len()]);
            turtle.set_heading(angle);
            turtle.forward(distance);
        }
    }
}
