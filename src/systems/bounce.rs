use amethyst::{
    core::{
        transform::Transform
    },
    ecs::{
        System, WriteStorage, ReadStorage, Join
    }
};

use nalgebra::{
    Vector2, Rotation2, Real
};

use crate::{
    pong::ARENA_HEIGHT,
    components::{
        Ball, Paddle, PADDLE_HEIGHT, Side, BALL_VELOCITY
    }
};

pub struct BounceSystem;

impl<'a> System<'a> for BounceSystem {
    type SystemData = (
        WriteStorage<'a, Ball>,
        ReadStorage<'a, Paddle>,
        ReadStorage<'a, Transform>
    );

    fn run(&mut self, (
        mut balls,
        paddles,
        transforms
    ): Self::SystemData)
    {
        for (ball, ball_transform) in (&mut balls, &transforms).join() {
            let ball_x = ball_transform.translation().x;
            let ball_y = ball_transform.translation().y;

            if (ball_y >= ARENA_HEIGHT - ball.radius && ball.direction.y > 0.0) || (ball_y <= ball.radius && ball.direction.y < 0.0) {
                ball.direction.y = -ball.direction.y;
            }

            for (paddle, paddle_transform) in (&paddles, &transforms).join() {
                let paddle_x = paddle_transform.translation().x;
                let paddle_y = paddle_transform.translation().y;

                let paddle_corner_x = paddle_x - paddle.width / 2.0;
                let paddle_corner_y = paddle_y - paddle.height / 2.0;

                let in_rect = point_in_rect(
                    ball_x,
                    ball_y,
                    paddle_corner_x - ball.radius,
                    paddle_corner_y - ball.radius,
                    paddle_corner_x + paddle.width + ball.radius,
                    paddle_corner_y + paddle.height + ball.radius);

                let right_direction = match paddle.side {
                    Side::Left => ball.direction.x < 0.0,
                    Side::Right => ball.direction.x > 0.0
                };

                if in_rect && right_direction {
                    println!("Inside");

                    let diff = paddle_y - ball_y;
                    let coeff = if diff.abs() < 0.3 {
                        0.
                    } else {
                        // diff / (PADDLE_HEIGHT / 2.0)
                        diff / PADDLE_HEIGHT
                    };

                    let angle = <f32 as Real>::pi() / 2.0 * coeff;

                    let normal = match paddle.side {
                        Side::Left => {
                            let r = Rotation2::new(-angle);
                            let n = Vector2::new(1., 0.);

                            r * n
                        },
                        Side::Right => {
                            let r = Rotation2::new(angle);
                            let n = Vector2::new(-1., 0.);

                            r * n
                        }
                    };

                    ball.direction.x = normal.x;
                    ball.direction.y = normal.y;

                    // dbg!(diff);
                    // dbg!(ball.velocity.y);

                    // let mut v = ball.velocity.clone();
                    // v.apply(|val| val * 2.0);
                    // let dot_product = v.dot(&normal);
                    // let length = normal.norm_squared();

                    // let r = ball.velocity - (dot_product / length) * normal;

                    // ball.velocity.x = r.x;
                    // ball.velocity.y = r.y;
                }
            }
        }
    }
}

fn point_in_rect(point_x: f32, point_y: f32, left: f32, bottom: f32, right: f32, top: f32) -> bool {
    point_x >= left && point_x <= right && point_y >= bottom && point_y <= top
}