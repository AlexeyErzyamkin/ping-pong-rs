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
    }
};

pub struct MoveBallSystem;

impl<'a> System<'a> for MoveBallSystem {
    type SystemData = (
        WriteStorage<'a, Transform>,
        ReadStorage<'a, Ball>,
        Read<'a, Time>
    );

    fn run(&mut self, (mut transforms, balls, time): Self::SystemData) {
        for (ball, transform) in (&balls, &mut transforms).join() {
            transform.translate_x(ball.velocity.x * time.delta_seconds());
            transform.translate_y(ball.velocity.y * time.delta_seconds());
        }
    }
}