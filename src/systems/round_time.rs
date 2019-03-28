use amethyst::{
    ecs::{
        System, Write, Read
    },
    core::timing::Time
};

use crate::pong::{
    GameSession
};

pub struct RoundTimeSystem;

impl <'a> System<'a> for RoundTimeSystem {
    type SystemData = (
        Write<'a, GameSession>,
        Read<'a, Time>
    );

    fn run(&mut self, (mut game_session, time): Self::SystemData) {
        game_session.round_time += time.delta_seconds();
    }
}