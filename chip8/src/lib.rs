#![feature(custom_attribute)]
#![allow(unused_mut)]
extern crate wasm_bindgen;
extern crate rand;
extern crate js_sys;

mod engine;

use wasm_bindgen::prelude::*;
use engine::chip::Chip;
use js_sys::*;

#[wasm_bindgen(module = "../www/index")]
extern "C" {
    fn setMainLoop(f: &Closure<FnMut()>);
    fn setVideoBuffer(vid_mem: *const u8);
    fn getKeys() -> u16;
    fn init();
}

#[wasm_bindgen]
pub struct ClosureHandle(Closure<FnMut()>);

#[wasm_bindgen]
pub fn run(rom: Uint8Array) -> ClosureHandle {
    let mut chip = Chip::new();

    rom.for_each(&mut |current, index, _array| {
        chip.mem.write(0x200 + index as usize, current)
    });

    init();
    let cb = Closure::wrap(Box::new(move || {
        (0..8).for_each(|_| { chip.emulate_cycle();});
        chip.clear_keys();
        let key = getKeys();

        if key < 17 {
            chip.key[key as usize] = 1;
        }
        if chip.delay_timer > 0 {
            chip.delay_timer -= 1;
        }
        if chip.sound_timer > 0 {
            chip.sound_timer -= 1;
        }

        let vid_ptr = chip.get_vid_mem_ptr();
        setVideoBuffer(vid_ptr);
    }) as Box<FnMut()>);

    setMainLoop(&cb);
    ClosureHandle(cb)
}