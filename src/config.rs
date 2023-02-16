use macroquad::prelude::*;

// WINDOW SETTINGS
pub const SCREEN_WIDTH: f32 = 1280f32;
pub const SCREEN_HEIGHT: f32 = 800f32;
pub const BACKGROUND_COLOR: Color = BLACK;

// WIN/LOSE SETTINGS
pub const BEST_OUT_OF: u32 = 32u32;

// PADDLE SETTINGS
pub const PADDLE_SIZE: Vec2 = Vec2::from_array([25.0, 200.0]);
pub const PADDLE_COLOR: Color = WHITE;

// PLAYER SETTINGS
pub const PLAYER1_SPEED: f32 = 5.0f32;
pub const PLAYER2_SPEED: f32 = 5.0f32;

// BALL SETTINGS
pub const BALL_SPEED: f32 = 5.0f32;
pub const BALL_RADIUS: f32 = 25f32;
pub const BALL_COLOR: Color = WHITE;
