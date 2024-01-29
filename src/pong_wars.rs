use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d};
use wasm_bindgen::prelude::*;
use core::f64::consts::PI;
use js_sys::Math::random;

const COLOR_ARCTIC_POWDER: &'static str = "#F1F6F4";
const COLOR_MYSTIC_MINT: &'static str = "#D9E8E3";
const COLOR_FORSYTHIA: &'static str = "#FFC801";
const COLOR_DEEP_SAFRON: &'static str = "#FF9932";
const COLOR_NOCTURNAL_EXPEDITION: &'static str = "#114C5A";
const COLOR_OCEANIC_NOIR: &'static str = "#172B36";
const SQUARE_SIZE: usize = 25;

struct PositionDiff {
    dx: f64,
    dy: f64
}

pub struct PongWars {
    canvas: HtmlCanvasElement,
    ctx: CanvasRenderingContext2d,

    x1: isize,
    y1: isize,
    dx1: f64,
    dy1: f64,

    x2: isize,
    y2: isize,
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
    pub fn new(canvas: HtmlCanvasElement) -> Self {
        let ctx = canvas.get_context("2d").unwrap().unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        let day_color = COLOR_MYSTIC_MINT;
        let day_ball_color = COLOR_NOCTURNAL_EXPEDITION;
    
        let night_color = COLOR_NOCTURNAL_EXPEDITION;
        let night_ball_color = COLOR_MYSTIC_MINT;
    
        let num_squares_x: usize = canvas.width() as usize / SQUARE_SIZE ;
        let num_squares_y: usize = canvas.height() as usize / SQUARE_SIZE;

        let mut squares: Vec<Vec<&'static str>> = vec![];

        for i in 0..num_squares_x {
            let mut row: Vec<&str> = vec![];
            for y in 0..num_squares_y {
                row.insert(y, if i < (num_squares_x / 2) { day_color } else { night_color });
            }
            squares.insert(i, row);
        }

        let mut x1 = canvas.width() as isize / 4;
        let mut y1 = canvas.height() as isize / 2;
        let mut dx1 = 12.5;
        let mut dy1 = -12.5;
    
        let mut x2 = (canvas.width() as isize / 4) * 3;
        let mut y2 = canvas.height() as isize / 2;
        let mut dx2 = -12.5;
        let mut dy2 = 12.5;
    
        let mut iteration = 0;

        PongWars {
            canvas,
            ctx,
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

    fn draw_ball(&self, x: isize, y: isize, color: &str) {
        self.ctx.begin_path();
        let _ = self.ctx.arc(x as f64, y as f64, (SQUARE_SIZE / 2) as f64, 0f64, PI * 2f64);
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
                    (i * SQUARE_SIZE) as f64, 
                    (j * SQUARE_SIZE) as f64, 
                    SQUARE_SIZE as f64, 
                    SQUARE_SIZE as f64);
            }
        }
    }

    fn update_square_and_bounce(&mut self, x: usize, y: usize, dx: f64, dy: f64, color: &str) -> PositionDiff {
        let mut updated_dx = dx;
        let mut updated_dy = dy;
        let mut angle = 0f64;
        while angle < PI * 2f64 {
            let check_x = x as f64 + angle.cos() * (SQUARE_SIZE / 2) as f64;
            let check_y = y as f64 + angle.sin() * (SQUARE_SIZE / 2) as f64;

            let i = (check_x / SQUARE_SIZE as f64).floor() as usize;
            let j = (check_y / SQUARE_SIZE as f64).floor() as usize;

            if i >= 0 && i < self.num_squares_x && j >= 0 && j < self.num_squares_y {
                if self.squares[i][j] != color {
                    self.squares.clone()[i][j] = color;

                    if angle.cos().abs() > angle.sin().abs() {
                        updated_dx = -updated_dx
                    } else {
                        updated_dy = -updated_dy;
                    }

                    updated_dx += self.random_num(-0.25, 0.25);
                    updated_dy += self.random_num(-0.25, 0.25);
                }
            }

            angle += PI / 4f64;
        }

        PositionDiff {
            dx: updated_dx,
            dy: updated_dy
        }
    }

    fn update_score_element(&self) {
    }

    fn check_boundary_collision(&mut self, x: usize, y: usize, dx: f64, dy: f64) -> PositionDiff {
        let mut dx = dx;
        let mut dy = dy;
        if x as f64 + dx > self.canvas.width() as f64 - SQUARE_SIZE as f64 / 2f64 || x as f64 + dx < SQUARE_SIZE as f64 / 2f64 {
            dx = -dx;
        }

        if y as f64 + dy > self.canvas.height() as f64 - SQUARE_SIZE as f64 / 2f64 || y as f64 + dy < SQUARE_SIZE as f64 / 2f64 {
            dy = -dy;
        }

        PositionDiff {
            dx: dx,
            dy: dy
        }
    }

    pub fn draw(&mut self) {
        self.ctx.clear_rect(0f64, 0f64, self.canvas.width() as f64, self.canvas.height() as f64);
        self.draw_squares();

        self.draw_ball(self.x1, self.y1, self.day_ball_color);
        let bounce1 = self.update_square_and_bounce(self.x1 as usize, self.y1 as usize, self.dx1, self.dy1, self.day_color);
        self.dx1 = bounce1.dx;
        self.dy1 = bounce1.dy;

        self.draw_ball(self.x2, self.y2, self.night_ball_color);
        let bounce2 = self.update_square_and_bounce(self.x2 as usize, self.y2 as usize, self.dx2, self.dy2, self.night_color);
        self.dx2 = bounce2.dx;
        self.dy2 = bounce2.dy;

        let boundary1 = self.check_boundary_collision(self.x1 as usize, self.y1 as usize, self.dx1, self.dy1);
        self.dx1 = boundary1.dx;
        self.dy1 = boundary1.dy;

        let boundary2 = self.check_boundary_collision(self.x2 as usize, self.y2 as usize, self.dx2, self.dy2);
        self.dx2 = boundary2.dx;
        self.dy2 = boundary2.dy;

        self.x1 += self.dx1 as isize;
        self.y1 += self.dy1 as isize;
        self.x2 += self.dx2 as isize;
        self.y2 += self.dy2 as isize;

        self.update_score_element();
    }
    fn random_num(&self, min: f64, max: f64) -> f64 {
        random() * (max - min) + min
    }
}