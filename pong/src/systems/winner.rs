use amethyst::{
    core::transform::Transform,
    ecs::{
        System, WriteStorage, Join, ReadExpect, Write
    },
    ui::UiText
};

use crate::{
    states::{
        ARENA_WIDTH, ARENA_HEIGHT, ScoreBoard, ScoreText, GameSession
    },
    components::{
        Ball, BALL_VELOCITY_X, BALL_VELOCITY_Y
    }
};

pub const MAX_SCORE: i32 = 10;

pub struct WinnerSystem;

impl<'a> System<'a> for WinnerSystem {
    type SystemData = (
        WriteStorage<'a, Ball>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, UiText>,
        Write<'a, ScoreBoard>,
        ReadExpect<'a, ScoreText>,
        Write<'a, GameSession>
    );

    fn run(&mut self, (
        mut balls,
        mut transforms,
        mut ui_texts,
        mut scoreboard,
        scoretext,
        mut game_session
    ): Self::SystemData)
    {
        for (ball, transform) in (&mut balls, &mut transforms).join() {
            let ball_x = transform.translation().x;

            let did_hit = if ball_x <= ball.radius.0 {
                scoreboard.score_right = (scoreboard.score_right + 1).min(999);

                if let Some(text) = ui_texts.get_mut(scoretext.p2_score) {
                    text.text = scoreboard.score_right.to_string();
                }

                true
            }
            else if ball_x >= ARENA_WIDTH - ball.radius.0 {
                scoreboard.score_left = (scoreboard.score_left + 1).min(999);

                if let Some(text) = ui_texts.get_mut(scoretext.p1_score) {
                    text.text = scoreboard.score_left.to_string();
                }

                true
            }
            else {
                false
            };

            if did_hit {
                ball.direction.x = BALL_VELOCITY_X;
                ball.direction.y = BALL_VELOCITY_Y;
                
                transform.set_xyz(ARENA_WIDTH / 2.0, ARENA_HEIGHT / 2.0, 0.0);

                game_session.round_num += 1;
                game_session.round_time = 0.0;

                println!("Score: | {:^3} | {:^3} |", scoreboard.score_left, scoreboard.score_right);
            }
        }
    }
}