use rapier2d::prelude::*;
use std::sync::atomic::Ordering;

use nalgebra::Vector2;
use wasm_timer::Instant;

use crate::{BODY_1_MASS, BODY_2_MASS, BODY_3_MASS, PAUSED, RESET_REQUESTED, TIME_SCALE};

struct PhysicsEngine {
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
    collider_set: ColliderSet,
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
            let collider = ColliderBuilder::ball(RADIUS)
                .restitution(RESTITUTION)
                .build();
            let ball_body_handle = rigid_body_set.insert(rigid_body);
            handles.push(ball_body_handle);
            collider_set.insert_with_parent(collider, ball_body_handle, &mut rigid_body_set);
        }

        Self {
            physics_engine: PhysicsEngine {
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
            collider_set,
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

fn calculate_scale(
    body_1_pos: Vector2<f32>,
    body_2_pos: Vector2<f32>,
    body_3_pos: Vector2<f32>,
    origin: Vector2<f32>,
) -> f32 {
    let mut max_distance = (body_1_pos - origin)
        .magnitude_squared()
        .max(
            (body_2_pos - origin).magnitude_squared()
        )
        .max(
            (body_3_pos - origin).magnitude_squared()
        )
        .max(
            origin.magnitude_squared()
        )
        .sqrt();

    // add padding
    max_distance /= 0.9;
    max_distance
}

impl Sim {
    fn update_body(
        &mut self,
        handle: RigidBodyHandle,
        other_handles: [RigidBodyHandle; 2],
        delta: f32,
    ) {
        let mut acceleration = Vector2::default();
        let this_pos = self.rigid_body_set[handle].translation();

        for other_handle in other_handles {
            let other_body = &self.rigid_body_set[other_handle];
            let mut travel = other_body.translation() - this_pos;
            let distance = travel.magnitude();
            travel.unscale_mut(distance);
            acceleration += travel * other_body.mass() / distance.powi(2);
        }

        let this_body = &mut self.rigid_body_set[handle];
        this_body.set_linvel(this_body.linvel() + acceleration * delta, true);
    }

    pub fn update(&mut self) -> Option<SimOutput> {
        let delta = self.last_instant.elapsed();
        self.last_instant = self.last_instant + delta;

        if RESET_REQUESTED.swap(false, Ordering::Relaxed) {
            PAUSED.store(true, Ordering::Relaxed);

            let body_1_pos = Vector2::new(0.0, 8.0);
            let body_2_pos = Vector2::new(6.9282 , -4.0);
            let body_3_pos = Vector2::new(-6.9282, -4.0);

            self.rigid_body_set[self.body_1_handle].set_translation(body_1_pos, true);
            self.rigid_body_set[self.body_2_handle].set_translation(body_2_pos, true);
            self.rigid_body_set[self.body_3_handle].set_translation(body_3_pos, true);
            self.rigid_body_set[self.body_1_handle].set_linvel((body_2_pos - body_1_pos).normalize(), true);
            self.rigid_body_set[self.body_2_handle].set_linvel((body_3_pos - body_2_pos).normalize(), true);
            self.rigid_body_set[self.body_3_handle].set_linvel((body_1_pos - body_3_pos).normalize(), true);
            let origin = (body_1_pos + body_2_pos + body_3_pos) / 4.0;

            return Some(SimOutput {
                origin,
                scale: calculate_scale(body_1_pos, body_2_pos, body_3_pos, origin),
                body_1_pos,
                body_2_pos,
                body_3_pos,
            });
        }
        if PAUSED.load(Ordering::Relaxed) {
            return None;
        }

        let delta = delta.as_secs_f32() * f32::from_bits(TIME_SCALE.load(Ordering::Relaxed));
        let body_1_mass = f32::from_bits(BODY_1_MASS.load(Ordering::Relaxed));
        let body_2_mass = f32::from_bits(BODY_2_MASS.load(Ordering::Relaxed));
        let body_3_mass = f32::from_bits(BODY_3_MASS.load(Ordering::Relaxed));

        self.collider_set[self.rigid_body_set[self.body_1_handle].colliders()[0]]
            .set_mass(body_1_mass);
        self.collider_set[self.rigid_body_set[self.body_2_handle].colliders()[0]]
            .set_mass(body_2_mass);
        self.collider_set[self.rigid_body_set[self.body_3_handle].colliders()[0]]
            .set_mass(body_3_mass);

        self.update_body(
            self.body_1_handle,
            [self.body_2_handle, self.body_3_handle],
            delta,
        );
        self.update_body(
            self.body_2_handle,
            [self.body_1_handle, self.body_3_handle],
            delta,
        );
        self.update_body(
            self.body_3_handle,
            [self.body_1_handle, self.body_2_handle],
            delta,
        );
        self.physics_engine.integration_params.dt = delta;

        self.physics_engine.physics_pipeline.step(
            &Vector2::default(),
            &self.physics_engine.integration_params,
            &mut self.physics_engine.island_manager,
            &mut self.physics_engine.broad_phase,
            &mut self.physics_engine.narrow_phase,
            &mut self.rigid_body_set,
            &mut self.collider_set,
            &mut self.physics_engine.impulse_joint_set,
            &mut self.physics_engine.multibody_joint_set,
            &mut self.physics_engine.ccd_solver,
            Some(&mut self.physics_engine.query_pipeline),
            &(),
            &(),
        );
        let body_1_pos = *self.rigid_body_set[self.body_1_handle].translation();
        let body_2_pos = *self.rigid_body_set[self.body_2_handle].translation();
        let body_3_pos = *self.rigid_body_set[self.body_3_handle].translation();
        let origin = (body_1_pos + body_2_pos + body_3_pos) / 4.0;

        Some(SimOutput {
            origin,
            scale: calculate_scale(body_1_pos, body_2_pos, body_3_pos, origin),
            body_1_pos,
            body_2_pos,
            body_3_pos,
        })
    }
}
