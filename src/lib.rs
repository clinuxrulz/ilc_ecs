#![feature(custom_attribute)]
#![feature(extern_prelude)]

extern crate wasm_bindgen;
extern crate web_sys; 
extern crate js_sys;
extern crate sodium_rust_push_pull;

use core::{mem};
use web_sys::console;
use wasm_bindgen::prelude::*;
use sodium_rust_push_pull::sodium::*;

#[wasm_bindgen]
pub fn hello_world() { 
    console::log_1(&"hello world (from wasm)".into());
}

#[wasm_bindgen]
pub fn sodium_ctx_new() -> *const SodiumCtx {
    Box::into_raw(Box::new(SodiumCtx::new()))
}

#[wasm_bindgen]
pub fn start_frp(sodium_ctx: *const SodiumCtx, state:&JsValue, on_tick:&js_sys::Function) -> *mut StreamSink<i32> { 
    let this = &JsValue::null();
    
    let sodium_ctx = unsafe { &*sodium_ctx };
    
    let s_tick: StreamSink<i32> = sodium_ctx.new_stream_sink();

    // these 3 are being captured, so clone the reference to obtain the
    // value of type js-reference instead of the reference to a js-reference
    let this = this.clone();
    let state = state.clone();
    let on_tick = on_tick.clone();
    
    s_tick.listen(move |now| { // <-- "move" to allow the values to be moved into and owned by the lambda
        on_tick.call1(&this, &state);
    });

    s_tick.send(&0); //this works!

    //try to return the stream for further sending via js
    let boxed = Box::new(s_tick);
    Box::into_raw(boxed)
}

#[wasm_bindgen]
pub fn send_frp(ptr: *mut StreamSink<i32>, value: i32) {
    //Something here is failing

    let s = unsafe { Box::from_raw(ptr) };
    s.send(&value);
    mem::forget(s);
}