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
pub fn start_frp(sodium_ctx: *const SodiumCtx, prefix:&JsValue, on_tick:&js_sys::Function) -> *mut StreamSink<i32> { 
    let this = &JsValue::null();
    
    let sodium_ctx = unsafe { &*sodium_ctx };
    
    let s_tick: StreamSink<i32> = sodium_ctx.new_stream_sink();

    // these 3 are being captured, so clone the reference to obtain the
    // value of type js-reference instead of the reference to a js-reference
    let this = this.clone();
    let prefix = prefix.as_string().expect("prefix must be a string!");
    let on_tick = on_tick.clone();
    
    s_tick.listen(move |now| { // <-- "move" to allow the values to be moved into and owned by the lambda
        let now = now.to_string();
        let output = format!("{}{}", prefix, now);
        let output = JsValue::from_str(&output);
        on_tick.call1(&this, &output);
    });

    //return the stream for further sending via js
    //this is done by putting it in a box and returning a pointer
    let boxed = Box::new(s_tick);
    Box::into_raw(boxed)
}

#[wasm_bindgen]
pub fn send_frp(ptr: *mut StreamSink<i32>, value: i32) {
    //extract the boxed pointer to get the stream
    let s = unsafe { Box::from_raw(ptr) };

    //now we can use it as we wish :D
    s.send(&value);

    //keep the stream as a raw pointer - don't Drop it
    mem::forget(s);
}
