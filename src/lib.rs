use std::sync::atomic::{AtomicU8, Ordering};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    fn wire_begin();
    fn wire_begin_transmission(d: u8);
    fn wire_write(s: &[u8]);
    fn wire_end_transmission();
    fn delay(d: i32);
}

static X_ATOMIC: AtomicU8 = AtomicU8::new(0);

#[wasm_bindgen]
pub fn setup() {
    wire_begin();
}

#[wasm_bindgen]
pub fn main_loop() {
    wire_begin_transmission(4);
    wire_write("x is ".as_bytes());
    let x = X_ATOMIC.load(Ordering::Relaxed);
    wire_write(vec!(x).as_slice());
    X_ATOMIC.store(x + 1, Ordering::Relaxed);
    delay(500);
}
