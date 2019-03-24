extern crate amethyst;
extern crate nalgebra;

use amethyst::{
    prelude::*,
    renderer::{
        SpriteSheetHandle, SpriteRender
    },
    core::transform::{
        Transform
    },
    ecs::prelude::{
        Component, DenseVecStorage
    }
};

use nalgebra::base::Vector2;

use crate::{
    pong::{
        ARENA_HEIGHT, ARENA_WIDTH
    }
};

pub const BALL_VELOCITY: f32 = 20.0;
pub const BALL_VELOCITY_X: f32 = 20.0;
pub const BALL_VELOCITY_Y: f32 = 0.0;
pub const BALL_RADIUS: f32 = 2.0;

pub struct Ball {
    pub velocity: Vector2<f32>,
    pub radius: f32
}

impl Ball {
    pub fn new(radius: f32, velocity_x: f32, velocity_y: f32) -> Self {
        Ball {
            radius,
            velocity: Vector2::new(velocity_x, velocity_y)
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
        .with(Ball::new(BALL_RADIUS, BALL_VELOCITY_X, BALL_VELOCITY_Y))
        .with(transform)
        .build();
}
