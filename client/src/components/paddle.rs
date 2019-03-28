use amethyst::{
    prelude::*,
    core::transform::Transform,
    ecs::{
        Component, DenseVecStorage, World
    },
    renderer::{
        SpriteSheetHandle, SpriteRender, Flipped
    },
};

use crate::{
    pong::{
        ARENA_HEIGHT, ARENA_WIDTH
    }
};

pub const PADDLE_HEIGHT: f32 = 16.0;
pub const PADDLE_WIDTH: f32 = 4.0;

#[derive(PartialEq, Eq)]
pub enum Side {
    Left,
    Right
}

pub struct Paddle {
    pub side: Side,
    pub height: f32,
    pub width: f32
}

impl Paddle {
    pub fn new(side: Side) -> Paddle {
        Paddle {
            side,
            height: PADDLE_HEIGHT,
            width: PADDLE_WIDTH
        }
    }
}

impl Component for Paddle {
    type Storage = DenseVecStorage<Self>;
}

pub fn initialize_paddles(world: &mut World, sprite_sheet: SpriteSheetHandle) {
    let mut left_transform = Transform::default();
    let mut right_transform = Transform::default();

    let y = ARENA_HEIGHT / 2.0;
    let left_x = PADDLE_WIDTH * 0.5;
    let right_x = ARENA_WIDTH - PADDLE_WIDTH * 0.5;

    left_transform.set_xyz(left_x, y, 0.0);
    right_transform.set_xyz(right_x, y, 0.0);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 0
    };

    world
        .create_entity()
        .with(sprite_render.clone())
        .with(Paddle::new(Side::Left))
        .with(left_transform)
        .build();

    world
        .create_entity()
        .with(sprite_render.clone())
        .with(Flipped::Horizontal)
        .with(Paddle::new(Side::Right))
        .with(right_transform)
        .build();
}