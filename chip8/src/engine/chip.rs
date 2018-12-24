#![allow(non_snake_case, dead_code, unused_variables, unused_mut)]

use rand::*;
use engine::ram::Ram;
use super::utils::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone)]
pub enum RunMode {
    Legacy,
    SuperChip
}

#[derive(Clone)]
pub struct Chip {
    pub I: usize,
    pub mem: Ram,
    V: [u8; 16],
    pub PC: u16,
    pub SP: u8,
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub vid_mem: [[u8; SCREEN_COLUMNS]; SCREEN_ROWS],
    stack: [u16; 16],
    pub key: [u8; 16],
    draw: bool,
    mode: RunMode,
}

impl Chip {
    pub fn new() -> Chip {
        Chip {
            I: 0,
            mem: Ram::new(),
            V: [0; 16],
            PC: 0x200,
            SP: 0,
            delay_timer: 0,
            sound_timer: 0,
            vid_mem: [[0; SCREEN_COLUMNS]; SCREEN_ROWS],
            stack: [0; 16],
            key: [0; 16],
            draw: false,
            mode: RunMode::SuperChip
        }
    }
    
    pub fn key_pressed(&mut self, key: u8) {
        self.key[key as usize] = 1;
    }

    pub fn print_mem(&self, all: bool) {
        self.mem.print(all);
    }

    pub fn load_rom(&mut self, rom: Vec<u8>) {
        self.mem.write_rom(rom);
    }
    pub fn emulate_cycle(&mut self) {
        let opcode = self.fetch();
        self.execute(opcode);
    }
    
    pub fn get_vid_mem(&self) -> [[u8; SCREEN_COLUMNS]; SCREEN_ROWS] {
        self.vid_mem.clone()
    }

    pub fn get_vid_mem_ptr(&self) -> *const u8 {
        let b = Box::new(self.vid_mem);
        Box::into_raw(b) as *const u8
    }

    pub fn clear_keys(&mut self) {
        for index in 0..self.key.len() {
            self.key[index] = 0;
        }
    }

    pub fn fetch(&mut self) -> u16 {
        let opcode = self.mem.read(self.PC as usize);
        self.PC += 2;
        opcode
    }
    
    fn execute(&mut self, opcode: u16) {
        match (opcode & 0xf000) >> 12 {
            0x0 => {
                match opcode & 0x00ff {
                    0x00e0 => self.decode_00E0(opcode),
                    0x00ee => self.decode_00EE(opcode),
                    _ => {} //self.unimplemented(opcode),
                };
            }
            0x1 => self.decode_1NNN(opcode),
            0x2 => self.decode_2NNN(opcode),
            0x3 => self.decode_3XNN(opcode),
            0x4 => self.decode_4XNN(opcode),
            0x5 => self.decode_5XY0(opcode),
            0x6 => self.decode_6XNN(opcode),
            0x7 => self.decode_7XNN(opcode),
            0x8 => {
                match opcode & 0x000f {
                    0x0 => self.decode_8XY0(opcode),
                    0x1 => self.decode_8XY1(opcode),
                    0x2 => self.decode_8XY2(opcode),
                    0x3 => self.decode_8XY3(opcode),
                    0x4 => self.decode_8XY4(opcode),
                    0x5 => self.decode_8XY5(opcode),
                    0x6 => self.decode_8XY6(opcode),
                    0x7 => self.decode_8XY7(opcode),
                    0xe => self.decode_8XYE(opcode),
                    _ => {} //self.unimplemented(opcode),
                }
            }
            0x9 => self.decode_9XY0(opcode),
            0xa => self.decode_ANNN(opcode),
            0xb => self.decode_BNNN(opcode),
            0xc => self.decode_CXNN(opcode),
            0xd => self.decode_DXYN(opcode),
            0xe => {
                match opcode & 0x00ff {
                    0x9e => self.decode_EX9E(opcode),
                    0xa1 => self.decode_EXA1(opcode),
                    _ => {} //self.unimplemented(opcode),
                }
            }
            0xf => {
                match opcode & 0x00ff {
                    0x07 => self.decode_FX07(opcode),
                    0x0a => self.decode_FX0A(opcode),
                    0x15 => self.decode_FX15(opcode),
                    0x18 => self.decode_FX18(opcode),
                    0x1e => self.decode_FX1E(opcode),
                    0x29 => self.decode_FX29(opcode),
                    0x33 => self.decode_FX33(opcode),
                    0x55 => self.decode_FX55(opcode),
                    0x65 => self.decode_FX65(opcode),
                    _ => {} //self.unimplemented(opcode),
                }
            }
            _ => {}//self.unimplemented(opcode),
        };
    }

    fn decode_0NNN(&mut self, opcode: u16) {
        println!("opcode 0NNN not implemented but used");
    }

    fn decode_00E0(&mut self, opcode: u16) {
        self.vid_mem = [[0; SCREEN_COLUMNS]; SCREEN_ROWS];
    }

    fn decode_00EE(&mut self, opcode: u16) {
        self.SP = self.SP.wrapping_sub(1);
        self.PC = self.stack[self.SP as usize];
    }

    fn decode_1NNN(&mut self, opcode: u16) {
        self.PC = get_NNN(opcode);
    }

    fn decode_2NNN(&mut self, opcode: u16) {
        self.stack[self.SP as usize] = self.PC;
        self.SP = self.SP.wrapping_add(1);
        self.PC = get_NNN(opcode);
    }

    fn decode_3XNN(&mut self, opcode: u16) {
        if self.V[get_X(opcode) as usize] == get_NN(opcode) as u8 {
            self.PC += 2;
        }
    }

    fn decode_4XNN(&mut self, opcode: u16) {
        if self.V[get_X(opcode) as usize] != get_NN(opcode) as u8 {
            self.PC += 2;
        }
    }

    fn decode_5XY0(&mut self, opcode: u16) {
        if self.V[get_X(opcode) as usize] == self.V[get_Y(opcode) as usize] {
            self.PC += 2;
        }
    }

    fn decode_6XNN(&mut self, opcode: u16) {
        self.write_to_reg(get_X(opcode) as u8, get_NN(opcode) as u8);
    }

    fn decode_7XNN(&mut self, opcode: u16) {
        let vx = self.V[get_X(opcode) as usize];
        self.V[get_X(opcode) as usize] = vx.wrapping_add(get_NN(opcode) as u8);
    }

    fn decode_8XY0(&mut self, opcode: u16) {
        self.V[get_X(opcode) as usize] = self.V[get_Y(opcode) as usize];
    }

    fn decode_8XY1(&mut self, opcode: u16) {
        self.V[get_X(opcode) as usize] |= self.V[get_Y(opcode) as usize];
    }

    fn decode_8XY2(&mut self, opcode: u16) {
        self.V[get_X(opcode) as usize] &= self.V[get_Y(opcode) as usize];
    }

    fn decode_8XY3(&mut self, opcode: u16) {
        self.V[get_X(opcode) as usize] ^= self.V[get_Y(opcode) as usize];
    }

    fn decode_8XY4(&mut self, opcode: u16) {
        let res = self.V[get_X(opcode) as usize] as u16 + self.V[get_Y(opcode) as usize] as u16;
        self.V[get_X(opcode) as usize] = res as u8;
        self.V[0xf] = if res > 0xff { 1 } else { 0 };
    }

    fn decode_8XY5(&mut self, opcode: u16) {
        self.V[0xf] = (self.V[get_X(opcode) as usize] >= self.V[get_Y(opcode) as usize]) as u8;

        self.V[get_X(opcode) as usize] =
            self.V[get_X(opcode) as usize].wrapping_sub(self.V[get_Y(opcode) as usize]);
    }

    fn decode_8XY6(&mut self, opcode: u16) {
        self.V[0xf] = self.V[get_X(opcode) as usize] & 1;

        match self.mode.clone() {
            RunMode::SuperChip => self.V[get_X(opcode) as usize] >>= 1,
            RunMode::Legacy => self.V[get_X(opcode) as usize] = self.V[get_Y(opcode) as usize] >> 1,
        }
    }

    fn decode_8XY7(&mut self, opcode: u16) {
        self.V[0xf] = (self.V[get_X(opcode) as usize] <= self.V[get_Y(opcode) as usize]) as u8;

        self.V[get_X(opcode) as usize] =
            self.V[get_Y(opcode) as usize].wrapping_sub(self.V[get_X(opcode) as usize]);
    }

    fn decode_8XYE(&mut self, opcode: u16) {
        self.V[0xf] = self.read_reg(get_X(opcode) as u8) >> 7;
        self.V[get_X(opcode) as usize] = self.V[get_Y(opcode) as usize] << 1;
    }

    fn decode_9XY0(&mut self, opcode: u16) {
        if self.V[get_X(opcode) as usize] != self.V[get_Y(opcode) as usize] {
            self.PC += 2;
        }
    }

    fn decode_ANNN(&mut self, opcode: u16) {
        self.I = get_NNN(opcode) as usize;
    }

    fn decode_BNNN(&mut self, opcode: u16) {
        self.PC = get_NNN(opcode) + self.V[0x0] as u16;
    }

    fn decode_CXNN(&mut self, opcode: u16) {
        let mut rng = thread_rng().gen::<u32>();
        self.V[get_X(opcode) as usize] = rng as u8 & get_NN(opcode) as u8;
    }

    pub fn decode_DXYN(&mut self, opcode: u16) {
        let col = (self.V[get_X(opcode) as usize] % 64) as usize;
        let row = (self.V[get_Y(opcode) as usize] % 32) as usize;
        let height = get_N(opcode) as usize;
        let mut pixel: u16;
        self.V[0xf] = 0;

        for row_off in 0..height {
            pixel = self.mem.read((self.I as usize + row_off) as usize);
            pixel >>= 8;
            for col_off in 0..8 {
                if (pixel & 0x80 >> col_off) > 0 {
                    if self.vid_mem[(row + row_off) % 32][(col + col_off) % 64] == 1 {
                        self.V[0xf] = 1;
                    }
                    self.vid_mem[(row + row_off) % 32][(col + col_off) % 64] ^= 1
                }
            }
        }
    }

    fn decode_EX9E(&mut self, opcode: u16) {
        if self.key[self.V[get_X(opcode) as usize] as usize] == 1 {
            self.PC += 2;
        }
    }

    fn decode_EXA1(&mut self, opcode: u16) {
        if self.key[self.V[get_X(opcode) as usize] as usize] != 1 {
            self.PC += 2;
        }
    }

    fn decode_FX07(&mut self, opcode: u16) {
        self.V[get_X(opcode) as usize] = self.delay_timer;
    }

    fn decode_FX0A(&mut self, opcode: u16) {
        let mut pressed = false;

        for i in 0..self.key.len() {
            if self.key[i] == 1 {
                &mut self.write_to_reg(get_X(opcode) as u8, i as u8);
                pressed = true;
            }
        }
        if !pressed {
            self.PC -= 2;
        }
    }

    fn decode_FX15(&mut self, opcode: u16) {
        self.delay_timer = self.V[get_X(opcode) as usize] as u8;
    }

    fn decode_FX18(&mut self, opcode: u16) {
        //set sound timer = to VX
        self.sound_timer = self.V[get_X(opcode) as usize];
    }

    fn decode_FX1E(&mut self, opcode: u16) {
        self.I += self.V[get_X(opcode) as usize] as usize;
    }

    fn decode_FX29(&mut self, opcode: u16) {
        //might not be what the manual meant
        self.I = self.V[get_X(opcode) as usize]  as usize * 5;
    }

    fn decode_FX33(&mut self, opcode: u16) {
        let mut bcd: u8 = self.V[get_X(opcode) as usize] as u8;
        self.mem.write(self.I as usize + 0, bcd / 100);
        self.mem.write(self.I as usize + 1, (bcd % 100) / 10);
        self.mem.write(self.I as usize + 2, bcd % 10);

    }

    fn decode_FX55(&mut self, opcode: u16) {
        let last_reg = get_X(opcode) as usize;
        for j in 0..last_reg + 1 {
            self.mem.write(self.I + j, self.V[j]);
        }
    }

    fn decode_FX65(&mut self, opcode: u16) {
        let last_reg = get_X(opcode) as usize;
        for j in 0..last_reg + 1 {
            self.V[j] = ((self.mem.read(self.I + j) & 0xFF00) >> 8) as u8;
        }
    }

    fn write_to_reg(&mut self, i: u8, val: u8) {
        self.V[i as usize] = val;
    }

    fn read_reg(&self, i: u8) -> u8 {
        self.V[i as usize]
    }

    fn unimplemented(&self, opcode: u16) {
        println!("Unimplemented opcode: {:#04X}", opcode);
    }

    pub fn debug_print(&self, opcode: u16) {
        println!("Opcode: {:#04X} self.I: {:#04X}", opcode, self.I);
    }
}

