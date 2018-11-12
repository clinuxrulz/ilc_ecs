#![feature(custom_attribute)]

extern crate wasm_bindgen;
extern crate web_sys; 
extern crate js_sys;

use web_sys::console;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn hello_world() { 
    console::log_1(&"hello world (from wasm)".into());
}

