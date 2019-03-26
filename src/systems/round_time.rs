use amethyst::{
    ecs::{
        System, Write, Read
    },
    core::timing::Time
};

use crate::pong::{
    RoundTime
};

pub struct RoundTimeSystem;

impl <'a> System<'a> for RoundTimeSystem {
    type SystemData = (
        Write<'a, RoundTime>,
        Read<'a, Time>
    );

    fn run(&mut self, (mut round_time, time): Self::SystemData) {
        round_time.time += time.delta_seconds();
    }
}