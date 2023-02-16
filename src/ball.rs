use macroquad::prelude::*;
use crate::config::*;
pub struct Ball {
    pub circle: Circle,
    pub velocity: Vec2,
}

impl Ball {
    pub fn new() -> Self {
        Self {
            circle: Circle::new(SCREEN_WIDTH / 2f32 - 12.5, SCREEN_HEIGHT / 2f32 - 12.5, BALL_RADIUS),
            velocity: vec2(1.0, 1.0).normalize(),
        }
    }

    pub fn draw(&self) {
        draw_circle(self.circle.x, self.circle.y, self.circle.r, BALL_COLOR);
    }

    pub fn update(&mut self) {
        self.circle.x += self.velocity.x * BALL_SPEED;
        self.circle.y += self.velocity.y * BALL_SPEED;
        if self.circle.x < 0f32 {
            self.velocity.x = 1f32;
        }
        if self.circle.x > screen_width() - self.circle.r / 2f32 {
            self.velocity.x = -1f32;
        }
        if self.circle.y < 0f32 {
            self.velocity.y = 1f32;
        }
        if self.circle.y > screen_height() - self.circle.r / 2f32 {
            self.velocity.y = -1f32;
        }
    }
}
