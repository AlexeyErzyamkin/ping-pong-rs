use amethyst::{
    core::{
        transform::Transform,
        timing::Time
    },
    ecs::{
        System, WriteStorage, ReadStorage, Join, Read
    },
    input::InputHandler
};

use crate::{
    states::ARENA_HEIGHT,
    components::{
        Paddle, Side, PADDLE_HEIGHT
    }
};

const MAX_Y: f32 = ARENA_HEIGHT - PADDLE_HEIGHT / 2.0;
const MIN_Y: f32 = PADDLE_HEIGHT / 2.0;
const PADDLE_SPEED: f32 = 30.0;

pub struct PaddleSystem;

impl<'a> System<'a> for PaddleSystem {
    type SystemData = (
        WriteStorage<'a, Transform>,
        ReadStorage<'a, Paddle>,
        Read<'a, InputHandler<String, String>>,
        Read<'a, Time>
    );

    fn run(&mut self, (mut transforms, paddles, input, time): Self::SystemData) {
        for (paddle, transform) in (&paddles, &mut transforms).join() {
            let movement = match paddle.side {
                Side::Left => input.axis_value("left_paddle"),
                Side::Right => input.axis_value("right_paddle")
            };

            if let Some(move_amount) = movement {
                let scaled_move_amount = PADDLE_SPEED * time.delta_seconds() * move_amount as f32;
                let paddle_y = transform.translation().y;
                let new_paddle_y = (paddle_y + scaled_move_amount).max(MIN_Y).min(MAX_Y);

                transform.set_y(new_paddle_y);
            }
        }
    }
}