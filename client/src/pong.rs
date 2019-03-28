use amethyst::{
    prelude::*,
    renderer::{
        Camera, Projection, SpriteSheetFormat, SpriteSheetHandle, Texture, TextureMetadata, PngFormat, SpriteSheet
    },
    core::transform::{
        Transform
    },
    ecs::prelude::{
        Entity
    },
    assets::{
        AssetStorage, Loader
    },
    ui::{
        Anchor, TtfFormat, UiText, UiTransform
    }
};

use crate::components;

pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;

#[derive(Default)]
pub struct ScoreBoard {
    pub score_left: i32,
    pub score_right: i32
}

pub struct ScoreText {
    pub p1_score: Entity,
    pub p2_score: Entity
}

#[derive(Default)]
pub struct GameSession {
    pub round_num: i32,
    pub round_time: f32
}

impl GameSession {
    pub fn new() -> Self {
        GameSession {
            round_num: 0,
            round_time: 0.0
        }
    }
}

// #[derive(Default)]
// pub struct RoundTime {
//     pub time: f32
// }

pub struct Pong;

impl SimpleState for Pong {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let sprite_sheet_handle = load_sprite_sheet(world);

        components::initialize_ball(world, sprite_sheet_handle.clone());
        components::initialize_paddles(world, sprite_sheet_handle);
        initialize_score_board(world);
        initialize_camera(world);

        world.add_resource(GameSession::new())
    }
}

pub fn initialize_score_board(world: &mut World) {
    let font = world.read_resource::<Loader>().load(
        "font/square.ttf",
        TtfFormat,
        Default::default(),
        (),
        &world.read_resource()
    );

    let p1_transform = UiTransform::new(
        "P1".to_string(),
        Anchor::TopMiddle,
        -50., -50., 1., 200., 50., 1
    );

    let p2_transform = UiTransform::new(
        "P2".to_string(),
        Anchor::TopMiddle,
        50., -50., 1., 200., 50., 1
    );

    let p1_score = world
        .create_entity()
        .with(p1_transform)
        .with(UiText::new(
            font.clone(),
            "0".to_string(),
            [1., 1., 1., 1.],
            50.
        ))
        .build();

    let p2_score = world
        .create_entity()
        .with(p2_transform)
        .with(UiText::new(
            font.clone(),
            "0".to_string(),
            [1., 1., 1., 1.],
            50.
        ))
        .build();

    world.add_resource(ScoreText { p1_score, p2_score });
}

pub fn initialize_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_xyz(0.0, 0.0, 1.0);

    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0,
            ARENA_WIDTH,
            0.0,
            ARENA_HEIGHT
        )))
        .with(transform)
        .build();
}

pub fn load_sprite_sheet(world: &mut World) -> SpriteSheetHandle {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();

        loader.load(
            "texture/pong_spritesheet.png",
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();

    loader.load(
        "texture/pong_spritesheet.ron",
        SpriteSheetFormat,
        texture_handle,
        (),
        &sprite_sheet_storage
    )
}