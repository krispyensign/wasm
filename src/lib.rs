use std::sync::atomic::{AtomicU8, Ordering};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    fn wire_begin();
    fn wire_begin_transmission(d: u8);
    fn wire_write(s: &[u8]);
    fn wire_read() -> u8;
    fn wire_end_transmission();
    fn delay(d: i32);
    fn serial_begin(d: i32);
    fn serial_print(s: &[u8]);
    fn wire_on_receive_peripheral_receive_event();
    fn wire_available() -> i32;
}

static X_ATOMIC: AtomicU8 = AtomicU8::new(0);

#[wasm_bindgen]
pub fn controller_setup() {
    wire_begin();
}

#[wasm_bindgen]
pub fn controller_loop() {
    wire_begin_transmission(4);
    wire_write("x is ".as_bytes());
    let x = X_ATOMIC.load(Ordering::Relaxed);
    wire_write(vec!(x).as_slice());
    X_ATOMIC.store(x + 1, Ordering::Relaxed);
    delay(500);
}

#[wasm_bindgen]
pub fn peripheral_setup() {
    wire_begin();
    wire_on_receive_peripheral_receive_event();
    serial_begin(9600);
}

#[wasm_bindgen]
pub fn peripheral_loop() {
    delay(100);
}

#[wasm_bindgen]
pub fn peripheral_receive_event(_c: i32) {
    while 1 < wire_available() {
        let d = wire_read();
        serial_print(vec!(d).as_slice());
    }
    let x = wire_read();
    serial_print(vec!(x).as_slice());
}