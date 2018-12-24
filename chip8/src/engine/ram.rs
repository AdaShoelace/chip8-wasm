#![allow(dead_code, unused_variables)]

use wasm_bindgen::prelude::*;

pub const MEM_START: u16 = 0x200;
pub const MEM_SIZE: usize = 4096;


#[wasm_bindgen]
#[derive(Clone)]
pub struct Ram {
    mem: [u8; 4096],
}

#[wasm_bindgen]
impl Ram {
    pub fn new() -> Ram {
        let mut ram = Ram { mem: [0; MEM_SIZE] };
        let sprites: [[u8;5]; 16] = [
            [0xF0, 0x90, 0x90, 0x90, 0xF0],
            [0x20, 0x60, 0x20, 0x20, 0x70],
            [0xF0, 0x10, 0xF0, 0x80, 0xF0],
            [0xF0, 0x10, 0xF0, 0x10, 0xF0],
            [0x90, 0x90, 0xF0, 0x10, 0x10],
            [0xF0, 0x80, 0xF0, 0x10, 0xF0],
            [0xF0, 0x80, 0xF0, 0x90, 0xF0],
            [0xF0, 0x10, 0x20, 0x40, 0x40],
            [0xF0, 0x90, 0xF0, 0x90, 0xF0],
            [0xF0, 0x90, 0xF0, 0x10, 0xF0],
            [0xF0, 0x90, 0xF0, 0x90, 0x90],
            [0xE0, 0x90, 0xE0, 0x90, 0xE0],
            [0xF0, 0x80, 0x80, 0x80, 0xF0],
            [0xE0, 0x90, 0x90, 0x90, 0xE0],
            [0xF0, 0x80, 0xF0, 0x80, 0xF0],
            [0xF0, 0x80, 0xF0, 0x80, 0x80],
        ];

        //let mut i = 0x50;
        let mut i = 0;
        for sprite in &sprites {
            for pos in sprite {
                ram.mem[i] = *pos;
                i += 1;
            }
        }
        ram
    }
    pub fn get_meta_address(&self) -> *const [u8; 4096] {
        &self.mem
    }
    pub fn get_length(&self) -> usize {
        self.mem.len()
    }

    pub fn print(&self, sprite: bool) {
        let mut i; 
        
        if sprite {
            i = 0;
        } else {
            i = 0x200;
        }

        while i < self.get_length() - 2000 {
            println!(
                "Addr: {:#4X} opcode: {:#2X} {:#2X}",
                i,
                self.mem[i],
                self.mem[i + 1]
            );
            i += 2;
        }
    }

    pub fn write_rom(&mut self, rom: Vec<u8>) {
        let mut j = 0x200;
        for i in rom.iter() {
            self.mem[j] = *i;
            j += 1;
        }
    }

    pub fn read(&self, addr: usize) -> u16 {
        let mut ret: u16 = self.mem[addr] as u16;
        ret = (ret << 8) | self.mem[addr + 1] as u16;
        ret as u16
    }

    pub fn write(&mut self, addr: usize, data: u8) {
        self.mem[addr] = data;
    }
}

