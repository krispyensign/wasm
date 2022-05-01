#include <Arduino.h>
#include <Wire.h>

#include "../pkg/wasm_bg.h"

static wasm_rt_memory_t s_memory;

int main(void)
{
    init();
    
    /* Initialize the Wasm runtime. */
    wasm_rt_init();
    Z_wasm_bg_init();
    wasm_rt_allocate_memory(&s_memory, 1, 1);


#if defined(USBCON)
    USBDevice.attach();
#endif

    setup();

    for (;;) {
        loop();
        if (serialEventRun) serialEventRun();
    }
    return 0;
}

void wire_begin() {
    Wire.begin();
}

void wire_begin_transmission(u8 address) {
    Wire.beginTransmission(address);
}

void wire_write(u32 ptr, u32 size) {

    Wire.write(v, s)
}