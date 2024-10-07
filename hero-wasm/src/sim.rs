use std::sync::atomic::Ordering;
use rapier2d::prelude::*;

use nalgebra::Vector2;
use wasm_timer::Instant;

use crate::{BODY_1_MASS, BODY_2_MASS, BODY_3_MASS, PAUSED, RESET_REQUESTED};

struct PhysicsEngine {
    collider_set: ColliderSet,
    integration_params: IntegrationParameters,
    physics_pipeline: PhysicsPipeline,
    island_manager: IslandManager,
    broad_phase: DefaultBroadPhase,
    narrow_phase: NarrowPhase,
    impulse_joint_set: ImpulseJointSet,
    multibody_joint_set: MultibodyJointSet,
    ccd_solver: CCDSolver,
    query_pipeline: QueryPipeline,
}
const RADIUS: f32 = 1.0;
const RESTITUTION: f32 = 0.7;

pub struct Sim {
    rigid_body_set: RigidBodySet,
    body_1_handle: RigidBodyHandle,
    body_2_handle: RigidBodyHandle,
    body_3_handle: RigidBodyHandle,
    physics_engine: PhysicsEngine,
    last_instant: Instant,
}

impl Default for Sim {
    fn default() -> Self {
        let mut rigid_body_set = RigidBodySet::new();
        let mut collider_set = ColliderSet::new();
        let mut handles = vec![];

        for _ in 0..3 {
            let rigid_body = RigidBodyBuilder::dynamic()
                .translation(Vector2::default())
                .build();
            let collider = ColliderBuilder::ball(RADIUS).restitution(RESTITUTION).build();
            let ball_body_handle = rigid_body_set.insert(rigid_body);
            handles.push(ball_body_handle);
            collider_set.insert_with_parent(collider, ball_body_handle, &mut rigid_body_set);
        }

        Self {
            physics_engine: PhysicsEngine {
                collider_set,
                integration_params: IntegrationParameters::default(),
                physics_pipeline: PhysicsPipeline::new(),
                island_manager: IslandManager::new(),
                broad_phase: DefaultBroadPhase::new(),
                narrow_phase: NarrowPhase::new(),
                impulse_joint_set: ImpulseJointSet::new(),
                multibody_joint_set: MultibodyJointSet::new(),
                ccd_solver: CCDSolver::new(),
                query_pipeline: QueryPipeline::new(),
            },
            rigid_body_set,
            body_1_handle: handles[0],
            body_2_handle: handles[1],
            body_3_handle: handles[2],
            last_instant: Instant::now(),
        }
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
        let body_1_pos = self.rigid_body_set[self.body_1_handle].position().translation.vector;
        let body_2_pos = self.rigid_body_set[self.body_2_handle].position().translation.vector;
        let body_3_pos = self.rigid_body_set[self.body_3_handle].position().translation.vector;

        let mut max_distance = (body_1_pos - origin).magnitude_squared().max(
            (body_2_pos - origin)
                .magnitude_squared()
                .max((body_3_pos - origin).magnitude_squared()),
        ).sqrt();

        // add padding
        max_distance /= 0.9;
        max_distance
    }

    fn update_body(
        handle: RigidBodyHandle,
        other_handles: [RigidBodyHandle; 2],
        mut delta: f32,
    ) {
        let mut acceleration = Vector2::default();

        for (&&mut other_pos, other_mass) in other_pos.iter().zip(other_mass) {
            let mut travel = other_pos - *pos;
            let distance = travel.magnitude();
            travel.unscale_mut(distance);
            acceleration += travel * other_mass / distance.powi(2);
        }

        *vel += acceleration * delta;

        for ((other_pos, other_vel), other_mass) in other_pos.into_iter().zip(other_vel).zip(other_mass) {
            let mut collision_delta = (other_pos.x - pos.x + other_pos.y - pos.y) / (vel.x + vel.y);

            if !collision_delta.is_finite() {
                continue;
            }
            
            let predicted_pos = *pos + *vel * collision_delta;
            let closest_distance_sqr = (predicted_pos - *other_pos).magnitude_squared();
            if closest_distance_sqr > (2.0 * RADIUS).powi(2) {
                continue;
            }
            let intersection_length = ((2.0 * RADIUS).powi(2) - closest_distance_sqr).sqrt();
            let intersection_delta = intersection_length / vel.magnitude();
            collision_delta -= intersection_delta;
            collision_delta = collision_delta.max(0.0);

            if collision_delta > delta {
                continue;
            }
            delta -= collision_delta;
            *pos += *vel * collision_delta;

            let travel = (*pos - *other_pos).normalize();
            let mut relative_momentum = (*other_vel - *vel) * other_mass;
            let projection = relative_momentum.dot(&travel);
            if projection <= 0.0 {
                continue;
            }
            relative_momentum = projection * travel;
            let total_mass = other_mass + mass;
            let other_impulse = -relative_momentum * mass / total_mass;
            let impulse = relative_momentum * other_mass / total_mass;

            *vel += impulse / mass;
            *other_vel += other_impulse / other_mass;
        }

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
            });
        }
        if PAUSED.load(Ordering::Relaxed) {
            return None;
        }

        let delta = delta.as_secs_f32();
        let body_1_mass = f32::from_bits(BODY_1_MASS.load(Ordering::Relaxed));
        let body_2_mass = f32::from_bits(BODY_2_MASS.load(Ordering::Relaxed));
        let body_3_mass = f32::from_bits(BODY_3_MASS.load(Ordering::Relaxed));
        Self::update_body(
            &mut self.body_1_pos,
            &mut self.body_1_vel,
            body_1_mass,
            [&mut self.body_2_pos, &mut self.body_3_pos],
            [&mut self.body_2_vel, &mut self.body_3_vel],
            [body_2_mass, body_3_mass],
            delta,
        );
        Self::update_body(
            &mut self.body_2_pos,
            &mut self.body_2_vel,
            body_2_mass,
            [&mut self.body_1_pos, &mut self.body_3_pos],
                [&mut self.body_1_vel, &mut self.body_3_vel],
            [body_1_mass, body_3_mass],
            delta,
        );
        Self::update_body(
            &mut self.body_3_pos,
            &mut self.body_3_vel,
            body_3_mass,
            [&mut self.body_1_pos, &mut self.body_2_pos],
            [&mut self.body_1_vel, &mut self.body_2_vel],
            [body_1_mass, body_2_mass],
            delta,
        );
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
