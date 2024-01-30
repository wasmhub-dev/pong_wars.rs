use core::num;
use std::{cell::RefCell, rc::Rc};

use js_sys::Math::log;
use wasm_bindgen::prelude::*;
use gloo::{console::log, utils::{body, document, iter, window}};
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

    let score_element = document().get_element_by_id("score").unwrap();

    let mut pong_wars = PongWars::new(canvas, score_element);

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::new(move || {
        pong_wars.draw();
        request_animation_frame(f.borrow().as_ref().unwrap());
    }));

    request_animation_frame(g.borrow().as_ref().unwrap());
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}