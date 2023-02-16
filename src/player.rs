use macroquad::prelude::*;
use crate::config::*;
use crate::ball::Ball;

pub enum PlayerType {
    Human,
    AI,
}

pub struct Player {
    pub rect: Rect,
    pub speed: f32,
    move_up: KeyCode,
    move_down: KeyCode,
    player_type: PlayerType,
}

impl Player {
    pub fn new(x: f32, y:f32, speed: f32, move_up: KeyCode, move_down: KeyCode, player_type: PlayerType) -> Self {
        Self {
            rect: Rect::new(
                    x,
                    y,
                    PADDLE_SIZE.x,
                    PADDLE_SIZE.y,
                ),
            speed,
            move_up,
            move_down,
            player_type,
        }
    }

    pub fn update(&mut self, ball: &Ball) {
        match self.player_type {
            PlayerType::Human => {
                if is_key_down(self.move_up) {
                    if self.rect.y <= 0.0 {
                        self.rect.y = 0.0;
                    } else {
                        self.rect.y -= self.speed;
                    }
                } else if is_key_down(self.move_down) {
                    if self.rect.y >= screen_height() - self.rect.h {
                        self.rect.y = screen_height() - self.rect.h
                    } else {
                        self.rect.y += self.speed;
                    }
                }
            },
            PlayerType::AI => {
                if ball.circle.point().y > self.rect.center().y {
                    self.rect.y += 2.5f32;
                } else if ball.circle.point().y < self.rect.center().y {
                    self.rect.y -= 2.5f32;
                }
            }
        }
    }

    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, PADDLE_COLOR)
    }
}