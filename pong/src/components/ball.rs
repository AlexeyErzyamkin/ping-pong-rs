use amethyst::{
    prelude::*,
    renderer::{
        SpriteSheetHandle, SpriteRender
    },
    core::{
        transform::Transform,
        nalgebra::Vector2
    },
    ecs::prelude::{
        Component, DenseVecStorage
    }
};

use crate::{
    states::{
        ARENA_HEIGHT, ARENA_WIDTH
    }
};

pub const BALL_VELOCITY: f32 = 30.0;
pub const BALL_VELOCITY_X: f32 = 1.0;
pub const BALL_VELOCITY_Y: f32 = 0.0;
pub const BALL_RADIUS: BallRadius = BallRadius(2.0);

pub struct BallRadius(pub f32);

pub struct Ball {
    pub direction: Vector2<f32>,
    pub velocity: f32,
    pub radius: BallRadius
}

impl Ball {
    pub fn new(radius: BallRadius, velocity: f32, velocity_x: f32, velocity_y: f32) -> Self {
        Ball {
            radius,
            velocity,
            direction: Vector2::new(velocity_x, velocity_y).normalize()
        }
    }
}

impl Component for Ball {
    type Storage = DenseVecStorage<Self>;
}

pub fn initialize_ball(world: &mut World, sprite_sheet: SpriteSheetHandle) {
    let mut transform = Transform::default();
    transform.set_xyz(ARENA_HEIGHT / 2.0, ARENA_WIDTH / 2.0, 0.0);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 1
    };

    world
        .create_entity()
        .with(sprite_render)
        .with(Ball::new(BALL_RADIUS, BALL_VELOCITY, BALL_VELOCITY_X, BALL_VELOCITY_Y))
        .with(transform)
        .build();
}
