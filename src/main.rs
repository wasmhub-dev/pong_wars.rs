use core::num;

use wasm_bindgen::prelude::*;
use gloo::{console::log, render::request_animation_frame, utils::{document, iter, window}};
use web_sys::{js_sys::Math, HtmlCanvasElement, CanvasRenderingContext2d};

mod pong_wars;
use pong_wars::PongWars;


#[wasm_bindgen(main)]
pub fn main() {
    log!("Welcome to the wasm world!");

    let canvas = document()
        .get_element_by_id("pongCanvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>().unwrap();

    let mut pong_wars = PongWars::new(canvas);
    pong_wars.draw();
    log!("request_animation_frame");
    request_animation_frame(move |_| {
        log!("request_animation_frame");
        pong_wars.draw();
    });
}