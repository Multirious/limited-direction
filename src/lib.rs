//! My algorithm that find path to any direction and distance but with limited directions
//! Seems to be support multiple directions but I'm not sure what is not supported.
//!
//! Read the code of the example to see keybindings.
//! `cargo run --example visualize` to see visualization
//! `cargo run --example snowflake` to see visualize multiple angle at once (It's kinda beautiful)

/// Round `n` to the nearest multiple of `multiple`
fn nearest_multiple(n: f64, multiple: f64) -> f64 {
    let mut result = n.abs() + multiple / 2.0;
    result -= result % multiple;
    result *= if n > 0.0 { 1.0 } else { -1.0 };
    result
}

#[derive(Debug)]
pub struct RigidWalk {
    angle: f64,
    displacement: f64,

    primary_angle: f64,
    secondary_angle: f64,

    secondary_segment_distance: f64,
    primary_segment_distance: f64,

    times: u32,

    last_primary_distance: f64,
    last_secondary_distance: f64,
}

impl RigidWalk {
    /// `angle` angle of direction to acheive in radian
    /// `displacement` is the displacement to achieve.
    /// `offset` is maximum distance relative to main line.
    pub fn new(
        primary_angle: f64,
        secondary_angle: f64,
        angle: f64,
        displacement: f64,
        offset: f64,
    ) -> RigidWalk {
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

        RigidWalk {
            angle,
            displacement,
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
        let RigidWalk {
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
    pub fn iter_full(&self, start_primary: bool) -> RigidWalkIterFull<'_> {
        RigidWalkIterFull::new(self, start_primary)
    }

    /// Walk in 8 direction of NSWE
    pub fn walk8(angle: f64, displacement: f64, offset: f64) -> RigidWalk {
        use std::f64::consts::{FRAC_PI_2, FRAC_PI_4};

        let primary_angle = nearest_multiple(angle, FRAC_PI_2);
        let secondary_angle = nearest_multiple(angle - FRAC_PI_4, FRAC_PI_2) + FRAC_PI_4;
        RigidWalk::new(primary_angle, secondary_angle, angle, displacement, offset)
    }

    /// Walk in 4 direction of up, down, left, right
    pub fn walk4(angle: f64, displacement: f64, offset: f64) -> RigidWalk {
        use std::f64::consts::{FRAC_PI_2, PI};

        let primary_angle = nearest_multiple(angle, PI);
        let secondary_angle = nearest_multiple(angle - FRAC_PI_2, PI) + FRAC_PI_2;
        RigidWalk::new(primary_angle, secondary_angle, angle, displacement, offset)
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

pub struct RigidWalkIterFull<'a> {
    walk: &'a RigidWalk,
    state: WalkIterState,
    start_primary: bool,
    switch: bool,
    n_left: u32,
}

impl<'a> RigidWalkIterFull<'a> {
    /// `start_primary` if you wanted to start algorithm at primary angle
    pub fn new(walk: &'a RigidWalk, start_primary: bool) -> RigidWalkIterFull<'a> {
        RigidWalkIterFull {
            n_left: walk.times,
            switch: start_primary,
            start_primary,
            walk,
            state: WalkIterState::MainStart,
        }
    }
}

impl<'a> Iterator for RigidWalkIterFull<'a> {
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
                // valid angle where the angle is actually either primary or secondary angle itself. Which broken the math.
                if self.walk.angle == self.walk.primary_angle {
                    self.state = WalkIterState::Stop;
                    return Some(WalkAct {
                        angle: self.walk.primary_angle,
                        distance: self.walk.displacement,
                    });
                } else if self.walk.angle == self.walk.secondary_angle {
                    self.state = WalkIterState::Stop;
                    return Some(WalkAct {
                        angle: self.walk.secondary_angle,
                        distance: self.walk.displacement,
                    });
                };

                if self.n_left == 0 {
                    self.state = WalkIterState::LastEnd;
                    let (angle, distance) = if self.start_primary {
                        self.switch = false;
                        value!(last_primary)
                    } else {
                        self.switch = true;
                        value!(last_secondary)
                    };
                    return Some(WalkAct { angle, distance });
                }
                self.state = WalkIterState::MainMiddle;
                if self.start_primary {
                    value!(primary)
                } else {
                    value!(secondary)
                }
            }
            WalkIterState::MainMiddle => {
                self.n_left = self.n_left - 1;
                if self.n_left == 0 {
                    self.state = WalkIterState::MainEnd;
                    return self.next();
                }
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
