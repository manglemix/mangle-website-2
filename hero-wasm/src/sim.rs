use std::sync::atomic::Ordering;

use nalgebra::Vector2;
use wasm_timer::Instant;

use crate::{BODY_1_MASS, BODY_2_MASS, BODY_3_MASS, PAUSED, RESET_REQUESTED};

pub struct Sim {
    body_1_pos: Vector2<f32>,
    body_2_pos: Vector2<f32>,
    body_3_pos: Vector2<f32>,
    body_1_vel: Vector2<f32>,
    body_2_vel: Vector2<f32>,
    body_3_vel: Vector2<f32>,
    last_instant: Instant,
}

impl Default for Sim {
    fn default() -> Self {
        Self { body_1_pos: Default::default(), body_2_pos: Default::default(), body_3_pos: Default::default(), body_1_vel: Default::default(), body_2_vel: Default::default(), body_3_vel: Default::default(), last_instant: Instant::now() }
    }
}

pub struct SimOutput {
    pub origin: Vector2<f32>,
    pub scale: f32,
    pub body_1_pos: Vector2<f32>,
    pub body_2_pos: Vector2<f32>,
    pub body_3_pos: Vector2<f32>,
}


impl Sim {
    fn calculate_scale(&self, origin: Vector2<f32>) -> f32 {
        let mut max_distance = (self.body_1_pos - origin).magnitude().max((self.body_2_pos - origin).magnitude().max((self.body_3_pos - origin).magnitude()));
        // add padding
        // max_distance /= 0.9;
        max_distance * 2.0
    }

    fn update_body(pos: &mut Vector2<f32>, vel: &mut Vector2<f32>, other_pos: [Vector2<f32>; 2], other_mass: [f32; 2], delta: f32) {
        let mut acceleration = Vector2::default();

        for (other_pos, other_mass) in other_pos.into_iter().zip(other_mass.into_iter()) {
            let mut travel = other_pos - *pos;
            let distance = travel.magnitude();
            travel.unscale_mut(distance);
            acceleration += travel * other_mass / distance.powi(2);
        }

        *vel += acceleration * delta;
        *pos += *vel * delta;
    }

    pub fn update(&mut self) -> Option<SimOutput> {
        let delta = self.last_instant.elapsed();
        self.last_instant = self.last_instant + delta;

        if RESET_REQUESTED.swap(false, Ordering::Relaxed) {
            PAUSED.store(true, Ordering::Relaxed);
            self.body_1_pos = Vector2::new(0.0, 2.0);
            self.body_2_pos = Vector2::new(-2.0, -2.0);
            self.body_3_pos = Vector2::new(2.0, -2.0);
            self.body_1_vel = Vector2::new(1.0, 0.0);
            self.body_2_vel = Vector2::new(0.0, 1.0);
            self.body_3_vel = Vector2::new(-1.0, 0.0);

            return Some(SimOutput {
                origin: (self.body_1_pos + self.body_2_pos + self.body_3_pos) / 3.0,
                scale: self.calculate_scale(Vector2::default()),
                body_1_pos: self.body_1_pos,
                body_2_pos: self.body_2_pos,
                body_3_pos: self.body_3_pos,
            })
        }
        if PAUSED.load(Ordering::Relaxed) {
            return None;
        }

        let delta = delta.as_secs_f32();
        let body_1_mass = f32::from_bits(BODY_1_MASS.load(Ordering::Relaxed));
        let body_2_mass = f32::from_bits(BODY_2_MASS.load(Ordering::Relaxed));
        let body_3_mass = f32::from_bits(BODY_3_MASS.load(Ordering::Relaxed));
        Self::update_body(&mut self.body_1_pos, &mut self.body_1_vel, [self.body_2_pos, self.body_3_pos], [body_2_mass, body_3_mass], delta);
        Self::update_body(&mut self.body_2_pos, &mut self.body_2_vel, [self.body_1_pos, self.body_3_pos], [body_1_mass, body_3_mass], delta);
        Self::update_body(&mut self.body_3_pos, &mut self.body_3_vel, [self.body_1_pos, self.body_2_pos], [body_1_mass, body_2_mass], delta);
        let origin = (self.body_1_pos + self.body_2_pos + self.body_3_pos) / 3.0;

        Some(SimOutput {
            origin,
            scale: self.calculate_scale(origin),
            body_1_pos: self.body_1_pos,
            body_2_pos: self.body_2_pos,
            body_3_pos: self.body_3_pos,
        })
    }
}