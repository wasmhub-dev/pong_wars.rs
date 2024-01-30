use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::prelude::*;
use gloo::utils::{document, window};
use web_sys::HtmlCanvasElement;

mod pong_wars;
use pong_wars::{Draw, PongWars};

#[wasm_bindgen(main)]
pub fn main() {
    let canvas = document()
        .get_element_by_id("pongCanvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>().unwrap();

    let score_element = document()
        .get_element_by_id("score").unwrap();

    let pong_wars = PongWars::new(canvas, score_element);

    recursive_draw(pong_wars);
}

fn recursive_draw(mut pong_wars: impl Draw + 'static) {

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::new(move || {
        pong_wars.draw();
        request_animation_frame_safe(f.borrow().as_ref().unwrap());
    }));

    request_animation_frame_safe(g.borrow().as_ref().unwrap());
}

fn request_animation_frame_safe(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}