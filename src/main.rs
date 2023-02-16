use macroquad::prelude::*;
mod config;
use config::*;
mod ball;
use ball::*;
mod player;
use player::*;

enum GameState {
    Running,
    Win,
    Quit,
}

fn collision(ball: &mut Ball, paddle: &Rect) {
    if ball.circle.overlaps_rect(paddle) {
        ball.velocity.x = -ball.velocity.x
    }
}

fn add_score(ball: &Ball, left_score: &mut usize, right_score: &mut usize) -> bool {
    if ball.circle.x < 50f32 + PADDLE_SIZE.x {
        *right_score += 1;
        true
    } else if ball.circle.x > screen_width() - PADDLE_SIZE.x - 50f32 {
        *left_score += 1;
        true
    } else {
        false
    }
}

fn reset_positions(player1: &mut Player, player2: &mut Player, ball: &mut Ball) {
    *player1 = Player::new(
                        50f32,
                        screen_height() / 2f32 - PADDLE_SIZE.y * 0.5f32,
                        PLAYER1_SPEED,
                        KeyCode::W,
                        KeyCode::S,
                        PlayerType::AI
              );
    *player2 = Player::new(
                        screen_width() - PADDLE_SIZE.x - 50f32,
                        screen_height() / 2f32 - PADDLE_SIZE.y * 0.5f32,
                        PLAYER2_SPEED,
                        KeyCode::Up,
                        KeyCode::Down,
                        PlayerType::AI
              );
    *ball = Ball::new();
}

#[macroquad::main("Pong")]
async fn main() {
    request_new_screen_size(SCREEN_WIDTH, SCREEN_HEIGHT);
    next_frame().await;
    let mut game_state = GameState::Running;
    let mut player1 = Player::new(
                        50f32,
                        screen_height() / 2f32 - PADDLE_SIZE.y * 0.5f32,
                        PLAYER1_SPEED,
                        KeyCode::W,
                        KeyCode::S,
                        PlayerType::Human,
                     );
    let mut player2 = Player::new(
                        screen_width() - PADDLE_SIZE.x - 50f32,
                        screen_height() / 2f32 - PADDLE_SIZE.y * 0.5f32,
                        PLAYER2_SPEED,
                        KeyCode::Up,
                        KeyCode::Down,
                        PlayerType::AI,
                     );
    let mut ball = Ball::new();
    let mut player1_score: usize = 0;
    let mut player2_score: usize = 0;

    loop {
        clear_background(BACKGROUND_COLOR);

        let player1_score_text = format!("{}", player1_score);
        let player2_score_text = format!("{}", player2_score);
        let player1_score_text_dim = measure_text(&player1_score_text, Some(Font::default()), 160u16, 1.0);
        let player2_score_text_dim = measure_text(&player2_score_text, Some(Font::default()), 160u16, 1.0);
        draw_text(&player1_score.to_string(), screen_width() * 0.5f32 - 200f32 - player1_score_text_dim.width * 0.5f32, 160f32, 150.0, WHITE);
        draw_text(&player2_score.to_string(), screen_width() * 0.5 + 200f32 - player2_score_text_dim.width * 0.5f32, 160f32, 150.0, WHITE);

        match game_state {
            GameState::Running => {

            },
            GameState::Win => {

            },
            GameState::Quit => {

            },
        }

        player1.draw();
        player2.draw();
        ball.draw();

        collision(&mut ball, &player1.rect);
        collision(&mut ball, &player2.rect);

        player1.update(&ball);
        ball.update();
        player2.update(&ball);

        if add_score(&ball, &mut player2_score, &mut player1_score) {
            reset_positions(&mut player1, &mut player2, &mut ball);
        }

        next_frame().await
    }

    // TODO: ADD THE GAME STATES
        // TODO: ADD DEMO
        // TODO: ADD LOSE
        // TODO: ADD WIN
}