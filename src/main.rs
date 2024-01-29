use core::num;

use wasm_bindgen::prelude::*;
use gloo::{console::log, render::request_animation_frame, utils::{document, iter, window}};
use web_sys::{js_sys::Math, HtmlCanvasElement, CanvasRenderingContext2d};
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

#[wasm_bindgen(main)]
pub fn main() {
    log!("Welcome to the wasm world!");

    let canvas = document()
        .get_element_by_id("pongCanvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>().unwrap();

    let ctx = canvas.get_context("2d").unwrap().unwrap().dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    let score_element = document()
        .get_element_by_id("score")
        .unwrap();

    let day_color = COLOR_MYSTIC_MINT;
    let day_ball_color = COLOR_NOCTURNAL_EXPEDITION;

    let night_color = COLOR_NOCTURNAL_EXPEDITION;
    let night_ball_color = COLOR_MYSTIC_MINT;

    let num_squares_x: usize = canvas.width() as usize / SQUARE_SIZE ;
    let num_squares_y: usize = canvas.height() as usize / SQUARE_SIZE;

    let mut squares: Vec<Vec<&str>> = vec![];

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

    let draw_ball = |x: isize, y: isize, color: &str| {
        ctx.begin_path();
        let _ = ctx.arc(x as f64, y as f64, (SQUARE_SIZE / 2) as f64, 0f64, PI * 2f64);
        ctx.set_fill_style(&JsValue::from_str(color));
        ctx.fill();
        ctx.close_path();
    };

    let draw_squares = || {
        for i in 0..num_squares_x {
            for j in 0..num_squares_y {
                let color = squares[i][j];
                ctx.set_fill_style(&JsValue::from_str(color));
                ctx.fill_rect(
                    (i * SQUARE_SIZE) as f64, 
                    (j * SQUARE_SIZE) as f64, 
                    SQUARE_SIZE as f64, 
                    SQUARE_SIZE as f64);
            }
        }
    };

    let update_square_and_bounce = |x: usize, y: usize, dx: f64, dy: f64, color: &str| -> PositionDiff {

        let mut updated_dx = dx;
        let mut updated_dy = dy;
        let mut angle = 0f64;
        while angle < PI * 2f64 {
            let check_x = x as f64 + angle.cos() * (SQUARE_SIZE / 2) as f64;
            let check_y = y as f64 + angle.sin() * (SQUARE_SIZE / 2) as f64;

            let i = (check_x / SQUARE_SIZE as f64).floor() as usize;
            let j = (check_y / SQUARE_SIZE as f64).floor() as usize;

            if i >= 0 && i < num_squares_x && j >= 0 && j < num_squares_y {
                if squares[i][j] != color {
                    squares.clone()[i][j] = color;

                    if angle.cos().abs() > angle.sin().abs() {
                        updated_dx = -updated_dx
                    } else {
                        updated_dy = -updated_dy;
                    }

                    updated_dx += random_num(-0.25, 0.25);
                    updated_dy += random_num(-0.25, 0.25);
                }
            }

            angle += PI / 4f64;
        }

        PositionDiff {
            dx: updated_dx,
            dy: updated_dy
        }
    };

    let update_score_element = || {

    };

    let check_boundary_collision = |x: usize, y: usize, dx: f64, dy: f64| -> PositionDiff {
        let mut dx = dx;
        let mut dy = dy;
        if x as f64 + dx > canvas.width() as f64 - SQUARE_SIZE as f64 / 2f64 || x as f64 + dx < SQUARE_SIZE as f64 / 2f64 {
            dx = -dx;
        }

        if y as f64 + dy > canvas.height() as f64 - SQUARE_SIZE as f64 / 2f64 || y as f64 + dy < SQUARE_SIZE as f64 / 2f64 {
            dy = -dy;
        }

        PositionDiff {
            dx: dx,
            dy: dy
        }
    };

    let mut draw = || {
        ctx.clear_rect(0f64, 0f64, canvas.width() as f64, canvas.height() as f64);
        draw_squares();

        draw_ball(x1, y1, day_ball_color);
        let bounce1 = update_square_and_bounce(x1 as usize, y1 as usize, dx1, dy1, day_color);
        dx1 = bounce1.dx;
        dy1 = bounce1.dy;

        draw_ball(x2, y2, night_ball_color);
        let bounce2 = update_square_and_bounce(x2 as usize, y2 as usize, dx2, dy2, night_color);
        dx2 = bounce2.dx;
        dy2 = bounce2.dy;

        let boundary1 = check_boundary_collision(x1 as usize, y1 as usize, dx1, dy1);
        dx1 = boundary1.dx;
        dy1 = boundary1.dy;

        let boundary2 = check_boundary_collision(x2 as usize, y2 as usize, dx2, dy2);
        dx2 = boundary2.dx;
        dy2 = boundary2.dy;

        x1 += dx1 as isize;
        y1 += dy1 as isize;
        x2 += dx2 as isize;
        y2 += dy2 as isize;

        iteration += 1;

        if (iteration % 100) == 0 {
            log!("interation", iteration);
        }
        
        update_score_element();
    };

    draw();
}

fn random_num(min: f64, max: f64) -> f64 {
    random() * (max - min) + min
}