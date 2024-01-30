use wasm_bindgen::prelude::*;

mod pong_wars;
mod animation;
use pong_wars::PongWars;
use animation::recursive_draw;

#[wasm_bindgen(main)]
pub fn main() {
    let pong_wars = PongWars::new();
    recursive_draw(pong_wars);
}