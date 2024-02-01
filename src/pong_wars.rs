#![allow(dead_code)]
use crate::animation::Drawable;
use core::f64::consts::PI;
use gloo::utils::document;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, Element, HtmlCanvasElement};

const COLOR_ARCTIC_POWDER: &'static str = "#F1F6F4";
const COLOR_MYSTIC_MINT: &'static str = "#D9E8E3";
const COLOR_FORSYTHIA: &'static str = "#FFC801";
const COLOR_DEEP_SAFFRON: &'static str = "#FF9932";
const COLOR_NOCTURNAL_EXPEDITION: &'static str = "#114C5A";
const COLOR_OCEANIC_NOIR: &'static str = "#172B36";
const SQUARE_SIZE: f64 = 25f64;

#[derive(Clone, Debug)]
pub struct PongWars {
    canvas: HtmlCanvasElement,
    ctx: CanvasRenderingContext2d,
    score_element: Element,

    x1: f64,
    y1: f64,
    dx1: f64,
    dy1: f64,

    x2: f64,
    y2: f64,
    dx2: f64,
    dy2: f64,

    day_color: &'static str,
    day_ball_color: &'static str,
    night_color: &'static str,
    night_ball_color: &'static str,
    num_squares_x: usize,
    num_squares_y: usize,

    squares: Vec<Vec<&'static str>>,
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

        let x1 = canvas.width() as f64 / 4f64;
        let y1 = canvas.height() as f64 / 2f64;
        let dx1 = 12.5;
        let dy1 = -12.5;

        let x2 = (canvas.width() as f64 / 4f64) * 3f64;
        let y2 = canvas.height() as f64 / 2f64;
        let dx2 = -12.5;
        let dy2 = 12.5;

        PongWars {
            canvas,
            ctx,
            score_element,
            x1,
            y1,
            dx1,
            dy1,
            x2,
            y2,
            dx2,
            dy2,
            day_color,
            day_ball_color,
            night_color,
            night_ball_color,
            num_squares_x,
            num_squares_y,
            squares,
        }
    }

    fn draw_ball(&self, x: f64, y: f64, color: &str) {
        self.ctx.begin_path();
        self.ctx
            .arc(x, y, SQUARE_SIZE / 2f64, 0f64, PI * 2f64)
            .expect("arc failed");
        self.ctx.set_fill_style(&JsValue::from_str(color));
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

    fn update_square_and_bounce(
        &mut self,
        x: f64,
        y: f64,
        dx: f64,
        dy: f64,
        color: &'static str,
    ) -> (f64, f64) {
        let mut updated_dx = dx;
        let mut updated_dy = dy;
        let mut angle = 0f64;

        while angle < PI * 2f64 {
            let check_x = x + (angle.cos() * SQUARE_SIZE / 2f64);
            let check_y = y + (angle.sin() * SQUARE_SIZE / 2f64);

            let i = (check_x / SQUARE_SIZE).floor() as usize;
            let j = (check_y / SQUARE_SIZE).floor() as usize;

            if i < self.num_squares_x && j < self.num_squares_y {
                if self.squares[i][j] != color {
                    self.squares[i][j] = color;

                    if angle.cos().abs() > angle.sin().abs() {
                        updated_dx = -updated_dx
                    } else {
                        updated_dy = -updated_dy;
                    }
                }
            }

            angle += PI / 4f64;
        }

        (updated_dx, updated_dy)
    }

    fn update_score_element(&self) {
        let mut day_score: usize = 0;
        let mut night_score: usize = 0;
        for i in 0..self.num_squares_x {
            for j in 0..self.num_squares_y {
                if self.squares[i][j] == self.day_color {
                    day_score += 1;
                } else if self.squares[i][j] == self.night_color {
                    night_score += 1;
                }
            }
        }
        let score = format!("day {} | night {}", day_score, night_score);
        self.score_element.set_text_content(Some(score.as_str()));
    }

    fn check_boundary_collision(&mut self, x: f64, y: f64, dx: f64, dy: f64) -> (f64, f64) {
        let mut dx = dx;
        let mut dy = dy;

        let width = self.canvas.width() as f64;
        let height = self.canvas.height() as f64;

        let square_half = SQUARE_SIZE / 2f64;

        if x + dx > width - square_half || x + dx < square_half {
            dx = -dx;
        }

        if y + dy > height - square_half || y + dy < square_half {
            dy = -dy;
        }

        (dx, dy)
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

        self.draw_ball(self.x1, self.y1, self.day_ball_color);
        let bounce1 = self.update_square_and_bounce(self.x1, self.y1, self.dx1, self.dy1, self.day_color);
        self.dx1 = bounce1.0;
        self.dy1 = bounce1.1;

        self.draw_ball(self.x2, self.y2, self.night_ball_color);
        let bounce2 = self.update_square_and_bounce(self.x2, self.y2, self.dx2, self.dy2, self.night_color);
        self.dx2 = bounce2.0;
        self.dy2 = bounce2.1;

        let boundary1 = self.check_boundary_collision(self.x1, self.y1, self.dx1, self.dy1);
        self.dx1 = boundary1.0;
        self.dy1 = boundary1.1;

        let boundary2 = self.check_boundary_collision(self.x2, self.y2, self.dx2, self.dy2);
        self.dx2 = boundary2.0;
        self.dy2 = boundary2.1;

        self.x1 += self.dx1;
        self.y1 += self.dy1;
        self.x2 += self.dx2;
        self.y2 += self.dy2;

        self.update_score_element();
    }
}
