#![allow(dead_code, unused_variables, non_snake_case)]

pub const SCREEN_COLUMNS: usize = 64;
pub const SCREEN_ROWS: usize = 32;
pub const SCALE: usize = 20;

pub fn get_NNN(opcode: u16) -> u16 {
    opcode & 0x0fff
}

pub fn get_NN(opcode: u16) -> u16 {
    opcode & 0x00ff
}

pub fn get_N(opcode: u16) -> u16 {
    opcode & 0x000f
}

pub fn get_X(opcode: u16) -> u16 {
    (opcode & 0x0f00) >> 8
}

pub fn get_Y(opcode: u16) -> u16 {
    (opcode & 0x00f0) >> 4
}

#[cfg(test)]
mod tests {

    use utils::*;
    const opcode: u16 = 0xd104; 

    #[test]
    fn test_X() { 
        let res = get_X(opcode) as usize;
        assert!(res == 0x1);
    }
    #[test]
    fn test_Y() { 
        let res = get_Y(opcode) as usize;
        assert!(res == 0x0);
    }
    #[test]
    fn test_N() { 
        let res = get_N(opcode) as usize;
        assert!(res == 0x4);
    }
    #[test]
    fn test_NN() { 
        let res = get_NN(opcode);
        assert!(res == 0x04);
    }
    #[test]
    fn test_NNN() { 
        let res = get_NNN(opcode);
        assert!(res == 0x104);
    }
}
