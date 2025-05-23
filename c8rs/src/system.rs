use std::sync::{LazyLock, Mutex};

use c8util::register::Register;

pub const MEMORY_SIZE: usize = 4096;

/// MEMORY: 4KB of RAM
pub static mut MEMORY: [u8; MEMORY_SIZE] = [0u8; MEMORY_SIZE];

/// Get the memory value at the current position.
pub fn get_memory_u8(addr: u16) -> u8 {
    assert!((addr & 0xf000) == 0, "Address must be 12-bit!");
    // SAFETY: single threaded
    unsafe { MEMORY[addr as usize] }
}

/// Return a 16-byte memory value at the current position.
pub fn get_memory_u16(addr: u16) -> u16 {
    assert!(((addr + 1) & 0xf000) == 0, "Address must be 12-bit!");
    (u16::from(get_memory_u8(addr)) << 8) | u16::from(get_memory_u8(addr + 1))
}

/// Set the memory value at the current position.
pub fn set_memory_u8(addr: u16, val: u8) {
    assert!((addr & 0xf000) == 0, "Address must be 12-bit!");
    // SAFETY: single threaded
    unsafe {
        MEMORY[addr as usize] = val;
    }
}

/// Set the memory value at the current position.
pub fn set_memory_u16(addr: u16, val: u16) {
    assert!((addr & 0xf000) == 0, "Address must be 12-bit!");
    set_memory_u8(addr, ((val >> 8) & 0x00FF) as u8);
    set_memory_u8(addr + 1, (val & 0x00FF) as u8);
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

    // SAFETY: single threaded
    unsafe { DISPLAY[x as usize][y as usize] }
}

/// Returns the full display.
pub fn get_full_display() -> [[bool; DISPLAY_HEIGHT]; DISPLAY_WIDTH] {
    // SAFETY: single threaded
    unsafe { DISPLAY }
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

    // SAFETY: single threaded
    unsafe { DISPLAY[x as usize][y as usize] = val };
}

/// The program counter (PC). Points at the current instruction in memory. Can only address 12 bits of memory.
pub static mut PC: u16 = 0;

pub fn get_pc() -> u16 {
    // SAFETY: single threaded
    unsafe { PC }
}

pub fn set_pc(val: u16) {
    assert!((val & 0xF000) == 0, "Address must be 12-bit");

    // SAFETY: single threaded
    unsafe { PC = val };
}

/// The index register (I). Points at a location in memory. Can only address 12 bits of memory.
pub static mut I: u16 = 0;

pub fn get_i() -> u16 {
    // SAFETY: single threaded
    unsafe { I }
}

pub fn set_i(val: u16) {
    assert!((val & 0xF000) == 0, "Address must be 12-bit");

    // SAFETY: single threaded
    unsafe { I = val };
}

pub const STACK_SIZE: usize = 16;

/// The stack. Contains 16-bit addresses. Used for calling and returning from functions.
pub static mut STACK: LazyLock<Mutex<Vec<u16>>> =
    LazyLock::new(|| Mutex::new(Vec::with_capacity(STACK_SIZE)));

pub fn stack_push(val: u16) {
    // SAFETY: single threaded
    #[allow(static_mut_refs)]
    unsafe {
        STACK.lock().unwrap().push(val);
    };
}

pub fn stack_pop() -> Option<u16> {
    // SAFETY: single threaded
    #[allow(static_mut_refs)]
    unsafe {
        STACK.lock().unwrap().pop()
    }
}

pub fn get_stack() -> Vec<u16> {
    // SAFETY: single threaded
    #[allow(static_mut_refs)]
    unsafe {
        STACK.lock().unwrap().clone()
    }
}

pub fn peek_stack() -> Option<u16> {
    // SAFETY: single threaded
    #[allow(static_mut_refs)]
    unsafe {
        let stack = STACK.lock().unwrap();
        if stack.len() > 0 {
            Some(stack[stack.len() - 1])
        } else {
            None
        }
    }
}

/// The delay timer. Decremented at a rate of 60 HZ until it reaches 0.
pub static mut DELAY_TIMER: u8 = 0;

pub fn get_delay_timer() -> u8 {
    // SAFETY: single threaded
    unsafe { DELAY_TIMER }
}

pub fn set_delay_timer(val: u8) {
    // SAFETY: single threaded
    unsafe { DELAY_TIMER = val }
}

pub fn decrement_delay_timer() {
    // SAFETY: single threaded
    unsafe { DELAY_TIMER = DELAY_TIMER.saturating_sub(1) }
}

/// The sound timer. Decremeted at a rate of 60 HZ until it reaches 0. Plays a sound as long as it is not 0.
pub static mut SOUND_TIMER: u8 = 0;

pub fn get_sound_timer() -> u8 {
    // SAFETY: single threaded
    unsafe { SOUND_TIMER }
}

pub fn set_sound_timer(val: u8) {
    // SAFETY: single threaded
    unsafe { SOUND_TIMER = val }
}

pub fn decrement_sound_timer() {
    // SAFETY: single threaded
    unsafe { SOUND_TIMER = SOUND_TIMER.saturating_sub(1) }
}

pub static mut REGISTERS: [u8; 16] = [0u8; 16];

pub fn get_registers() -> [u8; 16] {
    // SAFETY: single threaded
    unsafe { REGISTERS }
}

pub fn get_register(reg: Register) -> u8 {
    // SAFETY: single threaded
    unsafe { REGISTERS[reg as usize] }
}

pub fn set_register(reg: Register, val: u8) {
    // SAFETY: single threaded
    unsafe { REGISTERS[reg as usize] = val };
}
