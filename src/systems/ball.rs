extern crate amethyst;

use amethyst::{
    core::{
        transform::Transform,
        timing::Time
    },
    ecs::{
        System, WriteStorage, ReadStorage, Read, Join
    }
};

use crate::{
    components::{
        Ball
    },
    pong::RoundTime
};

pub struct MoveBallSystem;

impl<'a> System<'a> for MoveBallSystem {
    type SystemData = (
        WriteStorage<'a, Transform>,
        ReadStorage<'a, Ball>,
        Read<'a, Time>,
        Read<'a, RoundTime>
    );

    fn run(&mut self, (mut transforms, balls, time, round_time): Self::SystemData) {
        let game_time = round_time.time;
        let speed_inc = (game_time - 5.0).max(0.0); // 3.0;

        for (ball, transform) in (&balls, &mut transforms).join() {
            let ball_velocity = (ball.velocity + speed_inc) * time.delta_seconds();

            transform.translate_x(ball.direction.x * ball_velocity);
            transform.translate_y(ball.direction.y * ball_velocity);
        }
    }
}