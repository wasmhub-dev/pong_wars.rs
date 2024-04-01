#![allow(dead_code)]
use crate::animation::Drawable;
use core::f64::consts::PI;
use gloo::utils::document;
use wasm_bindgen::prelude::*;
use web_sys::{js_sys::Math::abs, CanvasRenderingContext2d, Element, HtmlCanvasElement};
use rand::prelude::*;

const COLOR_ARCTIC_POWDER: &'static str = "#F1F6F4";
const COLOR_MYSTIC_MINT: &'static str = "#D9E8E3";
const COLOR_FORSYTHIA: &'static str = "#FFC801";
const COLOR_DEEP_SAFFRON: &'static str = "#FF9932";
const COLOR_NOCTURNAL_EXPEDITION: &'static str = "#114C5A";
const COLOR_OCEANIC_NOIR: &'static str = "#172B36";
const SQUARE_SIZE: f64 = 25f64;

const MIN_SPEED: f64 = 5f64;
const MAX_SPEED: f64 = 25f64;

const DAY_COLOR: &'static str = COLOR_MYSTIC_MINT;
const NIGHT_COLOR: &'static str = COLOR_NOCTURNAL_EXPEDITION;


#[derive(Clone, Debug)]
struct Ball {
    x: f64,
    y: f64,
    dx: f64,
    dy: f64,
    reverse_color: &'static str,
    ball_color: &'static str,
}

#[derive(Clone, Debug)]
pub struct PongWars {
    canvas: HtmlCanvasElement,
    ctx: CanvasRenderingContext2d,
    score_element: Element,

    num_squares_x: usize,
    num_squares_y: usize,

    squares: Vec<Vec<&'static str>>,

    balls: Vec<Ball>,
}

impl PongWars {
    pub fn new() -> Self {
        let canvas = document()
            .get_element_by_id("pongCanvas")
            .unwrap()
            .dyn_into::<HtmlCanvasElement>()
            .unwrap();

        let score_element = document()
            .get_element_by_id("score")
            .unwrap();

        let ctx = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        let day_color = COLOR_MYSTIC_MINT;
        let day_ball_color = COLOR_NOCTURNAL_EXPEDITION;

        let night_color = COLOR_NOCTURNAL_EXPEDITION;
        let night_ball_color = COLOR_MYSTIC_MINT;

        let num_squares_x: usize = canvas.width() as usize / SQUARE_SIZE as usize;
        let num_squares_y: usize = canvas.height() as usize / SQUARE_SIZE as usize;

        let mut squares: Vec<Vec<&'static str>> = vec![];

        let balls = vec![
            Ball {
                x: canvas.width() as f64 / 4f64,
                y: canvas.height() as f64 / 2f64,
                dx: 12.5f64,
                dy: -12.5f64,
                reverse_color: day_color,
                ball_color: day_ball_color,
            },
            Ball {
                x: (canvas.width() as f64 / 4f64) * 3f64,
                y: canvas.height() as f64 / 2f64,
                dx: -12.5f64,
                dy: 12.5f64,
                reverse_color: night_color,
                ball_color: night_ball_color,
            },
        ];

        for i in 0..num_squares_x {
            let mut row: Vec<&str> = vec![];

            let color = if i < (num_squares_x / 2) {
                day_color
            } else {
                night_color
            };

            for j in 0..num_squares_y {
                row.insert(
                    j,
                    color,
                );
            };

            squares.insert(i, row);
        }

        PongWars {
            canvas,
            ctx,
            score_element,
            num_squares_x,
            num_squares_y,
            squares,
            balls,
        }
    }

    fn draw_ball(&self, ball_index: usize) {
        let ball = self.balls.get(ball_index).unwrap();
        self.ctx.begin_path();
        self.ctx
            .arc(ball.x, ball.y, SQUARE_SIZE / 2f64, 0f64, PI * 2f64)
            .expect("arc failed");
        self.ctx.set_fill_style(&JsValue::from_str(ball.ball_color));
        self.ctx.fill();
        self.ctx.close_path();
    }

    fn draw_squares(&self) {
        for i in 0..self.num_squares_x {
            for j in 0..self.num_squares_y {
                let color = self.squares[i][j];
                self.ctx.set_fill_style(&JsValue::from_str(color));
                self.ctx.fill_rect(
                    i as f64 * SQUARE_SIZE,
                    j as f64 * SQUARE_SIZE,
                    SQUARE_SIZE,
                    SQUARE_SIZE,
                );
            }
        }
    }

    fn check_square_collision(
        &mut self,
        ball_index: usize,
    ) {
        let ball = self.balls.get_mut(ball_index).unwrap();
        let mut angle = 0f64;

        while angle < PI * 2f64 {
            let check_x = ball.x + (angle.cos() * SQUARE_SIZE / 2f64);
            let check_y = ball.y + (angle.sin() * SQUARE_SIZE / 2f64);

            let i = (check_x / SQUARE_SIZE).floor() as usize;
            let j = (check_y / SQUARE_SIZE).floor() as usize;

            if i < self.num_squares_x && j < self.num_squares_y {
                if self.squares[i][j] != ball.reverse_color {
                    self.squares[i][j] = ball.reverse_color;

                    if angle.cos().abs() > angle.sin().abs() {
                        ball.dx = -ball.dx;
                    } else {
                        ball.dy = -ball.dy;
                    }
                }
            }

            angle += PI / 4f64;
        }
    }

    fn update_score_element(&self) {
        let mut day_score: usize = 0;
        let mut night_score: usize = 0;
        for i in 0..self.num_squares_x {
            for j in 0..self.num_squares_y {
                if self.squares[i][j] == DAY_COLOR {
                    day_score += 1;
                } else if self.squares[i][j] == NIGHT_COLOR {
                    night_score += 1;
                }
            }
        }
        let score = format!("day {} | night {}", day_score, night_score);
        self.score_element.set_text_content(Some(score.as_str()));
    }

    fn check_boundary_collision(&mut self, ball_index: usize) {
        let ball = self.balls.get_mut(ball_index).unwrap();
        let width = self.canvas.width() as f64;
        let height = self.canvas.height() as f64;
        if ball.x + ball.dx > width - (SQUARE_SIZE / 2f64) || ball.x + ball.dx < SQUARE_SIZE / 2f64 {
            ball.dx = -ball.dx;
        }

        if ball.y + ball.dy > height - (SQUARE_SIZE / 2f64) || ball.y + ball.dy < SQUARE_SIZE / 2f64 {
            ball.dy = -ball.dy;
        }
    }

    fn add_randomness(&mut self, ball_index: usize) {
        let ball = self.balls.get_mut(ball_index).unwrap();
        let mut rng = thread_rng();
        ball.dx += rng.gen::<f64>() * 0.01f64 - 0.005f64;
        ball.dy += rng.gen::<f64>() * 0.01f64 - 0.005f64;

        ball.dx = f64::min(f64::max(ball.dx, -MAX_SPEED), MAX_SPEED);
        ball.dy = f64::min(f64::max(ball.dy, -MAX_SPEED), MAX_SPEED);

        if abs(ball.dx) < MIN_SPEED {
            ball.dx = if ball.dx > 0f64 { MIN_SPEED } else { -MIN_SPEED };
        }

        if abs(ball.dy) < MIN_SPEED {
            ball.dy = if ball.dy > 0f64 { MIN_SPEED } else { -MIN_SPEED };
        }
    }

    fn update_ball(&mut self, ball_index: usize) {
        let ball = self.balls.get_mut(ball_index).unwrap();
        ball.x += ball.dx;
        ball.y += ball.dy;
    }
}

impl Drawable for PongWars {
    fn draw(&mut self) {
        self.ctx.clear_rect(
            0f64,
            0f64,
            self.canvas.width() as f64,
            self.canvas.height() as f64,
        );

        self.draw_squares();

        for i in 0..self.balls.len() {
            self.draw_ball(i);
            self.check_square_collision(i);
            self.check_boundary_collision(i);
    
            self.update_ball(i);
    
            self.add_randomness(i);
        }
    }
}
