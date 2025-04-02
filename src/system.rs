use core::panic;

/// Creates getters and setters for the given value.
#[macro_export]
macro_rules! get_set {
    () => {};
}

pub const MEMORY_SIZE: usize = 4096;

/// MEMORY: 4KB of RAM
pub static mut MEMORY: [u8; MEMORY_SIZE] = [0u8; MEMORY_SIZE];

/// Get the memory value at the current position.
pub fn get_memory_u8(addr: u16) -> u8 {
    assert!((addr & 0xf000) == 0, "Address must be 12-bit!");
    unsafe { MEMORY[addr as usize] }
}

/// Return a 16-byte memory value at the current position.
pub fn get_memory_u16(addr: u16) -> u16 {
    assert!(((addr + 1) & 0xf000) == 0, "Address must be 12-bit!");
    ((get_memory_u8(addr) as u16) << 8) | get_memory_u8(addr + 1) as u16
}

/// Set the memory value at the current position.
pub fn set_memory(addr: u16, val: u8) {
    assert!((addr & 0xf000) == 0, "Address must be 12-bit!");
    unsafe {
        MEMORY[addr as usize] = val;
    }
}

pub const DISPLAY_WIDTH: usize = 64;
pub const DISPLAY_HEIGHT: usize = 32;

/// DISPLAY: 64x32 pixels, monochrome
pub static mut DISPLAY: [[bool; DISPLAY_HEIGHT]; DISPLAY_WIDTH] =
    [[false; DISPLAY_HEIGHT]; DISPLAY_WIDTH];

/// Gets the current value of the display at the given position.
pub fn get_display(x: u8, y: u8) -> bool {
    assert!(
        (x as usize) < DISPLAY_WIDTH,
        "x-coord ({x}) was out of range of display width ({DISPLAY_WIDTH})"
    );
    assert!(
        (y as usize) < DISPLAY_HEIGHT,
        "y-coord ({y}) was out of range of display width ({DISPLAY_HEIGHT})"
    );

    unsafe { DISPLAY[x as usize][y as usize] }
}

/// Sets the display to the given value at the given position.
pub fn set_display(x: u8, y: u8, val: bool) {
    assert!(
        (x as usize) < DISPLAY_WIDTH,
        "x-coord ({x}) was out of range of display width ({DISPLAY_WIDTH})"
    );
    assert!(
        (y as usize) < DISPLAY_HEIGHT,
        "y-coord ({y}) was out of range of display width ({DISPLAY_HEIGHT})"
    );

    unsafe { DISPLAY[x as usize][y as usize] = val };
}

/// The program counter (PC). Points at the current instruction in memory. Can only address 12 bits of memory.
pub static mut PC: u16 = 0;

pub fn get_pc() -> u16 {
    unsafe { PC }
}

pub fn set_pc(val: u16) {
    assert!((val & 0xF000) == 0, "Address must be 12-bit");

    unsafe { PC = val };
}

/// The index register (I). Points at a location in memory. Can only address 12 bits of memory.
pub static mut I: u16 = 0;

pub fn get_i() -> u16 {
    unsafe { I }
}

pub fn set_i(val: u16) {
    assert!((val & 0xF000) == 0, "Address must be 12-bit");

    unsafe { I = val };
}

pub const STACK_SIZE: usize = 1024;

/// The stack. Contains 16-bit addresses. Used for calling and returning from functions.
pub static STACK: [u16; STACK_SIZE] = [0u16; STACK_SIZE];

// TODO: Getters and setters for stack

/// The delay timer. Decremented at a rate of 60 HZ until it reaches 0.
pub static DELAY_TIMER: u8 = 0;

// TODO: Getters and setters for delay timer

/// The sound timer. Decremeted at a rate of 60 HZ until it reaches 0. Plays a sound as long as it is not 0.
pub static SOUND_TIMER: u8 = 0;

// TODO: Getters and setters for sound timer

/// Registers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Register {
    V0 = 0,
    V1,
    V2,
    V3,
    V4,
    V5,
    V6,
    V7,
    V8,
    V9,
    VA,
    VB,
    VC,
    VD,
    VE,
    /// Also used as the flag register.
    VF,
}

impl From<u8> for Register {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::V0,
            1 => Self::V1,
            2 => Self::V2,
            3 => Self::V3,
            4 => Self::V4,
            5 => Self::V5,
            6 => Self::V6,
            7 => Self::V7,
            8 => Self::V8,
            9 => Self::V9,
            0xA => Self::VA,
            0xB => Self::VB,
            0xC => Self::VC,
            0xD => Self::VD,
            0xE => Self::VE,
            0xF => Self::VF,
            _ => panic!("value {value} could not be converted to register"),
        }
    }
}

pub static mut REGISTERS: [u8; 16] = [0u8; 16];

pub fn get_register(reg: Register) -> u8 {
    unsafe { REGISTERS[reg as usize] }
}

pub fn set_register(reg: Register, val: u8) {
    unsafe { REGISTERS[reg as usize] = val };
}
