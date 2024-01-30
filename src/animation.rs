use std::{cell::RefCell, rc::Rc};

use gloo::utils::window;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::*;

pub trait Drawable {
    fn draw(&mut self);
}

pub fn recursive_draw(mut drawable: impl Drawable + 'static) {

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::new(move || {
        drawable.draw();
        request_animation_frame_safe(f.borrow().as_ref().unwrap());
    }));

    request_animation_frame_safe(g.borrow().as_ref().unwrap());
}

fn request_animation_frame_safe(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}