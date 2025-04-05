use core::panic;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    sync::OnceLock,
    thread,
    time::Duration,
};

use device_query::{DeviceQuery, DeviceState, Keycode};

use crate::{
    debug_terminal::{DebugState, debug_terminal},
    decode::decode,
    execute::execute,
    instructions::Instruction,
    system::{
        DISPLAY_HEIGHT, DISPLAY_WIDTH, Register, decrement_delay_timer, decrement_sound_timer,
        get_delay_timer, get_display, get_full_display, get_i, get_memory_u8, get_memory_u16,
        get_pc, get_register, get_registers, get_sound_timer, get_stack, peek_stack, set_pc,
    },
};

/// The number of instructions to execute per second.
pub const INSTRUCTION_SPEED: usize = 720;

pub static KEYPRESS_MAP: OnceLock<HashMap<Keycode, u8>> = OnceLock::new();
pub static REVERSE_KEYPRESS_MAP: OnceLock<HashMap<u8, Keycode>> = OnceLock::new();

macro_rules! info {
    ($lines:tt, $($arg:tt)*) => {
        $lines.push(format!($($arg)*));
    };
}

/// Info, but appends to the previous line.
macro_rules! infop {
    ($lines:tt, $($arg:tt)*) => {
        let n_lines = $lines.len() - 1;
        $lines[n_lines].push_str(format!($($arg)*).as_str());
    };
}

/// Handles the core loop.
pub fn run() {
    for _ in 0..DISPLAY_HEIGHT + 5 {
        println!();
    }

    let mut n_instructions_executed = 0u128;

    let device_state = DeviceState::new();
    let mut pressed_keys: HashSet<Keycode> = HashSet::new();
    // Used for the GetKey instruction
    #[allow(unused_assignments)] // it thinks this is unused
    let mut last_pressed_keys: HashSet<Keycode> = HashSet::new();

    // Whether we are currently debugging.
    let mut is_debug = false;
    // The last command that was used in the debugger
    // let mut last_debug_command = String::new();

    KEYPRESS_MAP.get_or_init(|| {
        // 1 2 3 C
        // 4 5 6 D
        // 7 8 9 E
        // A 0 B F
        let mut h = HashMap::new();
        h.insert(Keycode::Key1, 0x1);
        h.insert(Keycode::Key2, 0x2);
        h.insert(Keycode::Key3, 0x3);
        h.insert(Keycode::Key4, 0xC);
        h.insert(Keycode::Q, 0x4);
        h.insert(Keycode::W, 0x5);
        h.insert(Keycode::E, 0x6);
        h.insert(Keycode::R, 0xD);
        h.insert(Keycode::A, 0x7);
        h.insert(Keycode::S, 0x8);
        h.insert(Keycode::D, 0x9);
        h.insert(Keycode::F, 0xE);
        h.insert(Keycode::Z, 0xA);
        h.insert(Keycode::X, 0x0);
        h.insert(Keycode::C, 0xB);
        h.insert(Keycode::V, 0xF);
        h
    });
    REVERSE_KEYPRESS_MAP.get_or_init(|| {
        // 1 2 3 C
        // 4 5 6 D
        // 7 8 9 E
        // A 0 B F
        let mut h = HashMap::new();
        h.insert(0x1, Keycode::Key1);
        h.insert(0x2, Keycode::Key2);
        h.insert(0x3, Keycode::Key3);
        h.insert(0xC, Keycode::Key4);
        h.insert(0x4, Keycode::Q);
        h.insert(0x5, Keycode::W);
        h.insert(0x6, Keycode::E);
        h.insert(0xD, Keycode::R);
        h.insert(0x7, Keycode::A);
        h.insert(0x8, Keycode::S);
        h.insert(0x9, Keycode::D);
        h.insert(0xE, Keycode::F);
        h.insert(0xA, Keycode::Z);
        h.insert(0x0, Keycode::X);
        h.insert(0xB, Keycode::C);
        h.insert(0xF, Keycode::V);
        h
    });

    // Used for printing debug messages to the right of the display
    // let mut info_lines: Vec<String> = Vec::with_capacity(DISPLAY_HEIGHT);
    // Detecting changes in register state
    // let mut old_register_state = get_registers();
    // let mut old_i_state = (get_i(), get_memory_u8(get_i()), get_memory_u8(get_i() + 2));
    // The last 3 instructions
    // let mut last_instructions: VecDeque<(u16, u16, Instruction)> = VecDeque::with_capacity(3);
    // The last state of the display
    // let mut old_display_state: [[bool; DISPLAY_HEIGHT]; DISPLAY_WIDTH];
    let mut debug_state = DebugState {
        last_debug_command: String::new(),
        last_instructions: VecDeque::with_capacity(3),
        info_lines: Vec::with_capacity(DISPLAY_HEIGHT),
        old_register_state: get_registers(),
        old_i_state: (get_i(), get_memory_u8(get_i()), get_memory_u8(get_i() + 2)),
        old_display_state: get_full_display(),
    };

    loop {
        debug_state.info_lines.clear();

        // Update keyboard state
        let keys = device_state.get_keys();
        last_pressed_keys = pressed_keys.clone();
        pressed_keys.clear();
        for key in keys {
            pressed_keys.insert(key);
        }

        if pressed_keys.contains(&Keycode::Escape) {
            is_debug = true;
            print!("\x1b[2K\r"); // Clear the current line to remove the escape code
        }

        // Fetch the next instruction
        let instruction_raw = fetch();

        // Decode the instruction
        let Some(instruction) = decode(instruction_raw) else {
            invalid_instruction(instruction_raw);
        };

        // If debugging, print debug info
        if is_debug {
            print_debug(
                &mut n_instructions_executed,
                instruction,
                instruction_raw,
                &mut debug_state,
            );
        }

        debug_state.old_register_state = get_registers();
        debug_state.old_display_state = get_full_display();
        debug_state.old_i_state = (get_i(), get_memory_u8(get_i()), get_memory_u8(get_i() + 2));

        if debug_state.last_instructions.len() == 3 {
            debug_state.last_instructions.pop_back();
        }
        debug_state
            .last_instructions
            .push_front((get_pc() - 2, instruction_raw, instruction));

        // Execute the instruction
        execute(
            instruction,
            &pressed_keys,
            &last_pressed_keys,
            n_instructions_executed,
        );

        // Count down delay and sound timers
        if n_instructions_executed % 12 == 0 {
            decrement_delay_timer();
            decrement_sound_timer();
        }

        // Delay for 1/720 of a second
        thread::sleep(Duration::from_secs_f32(1.0 / INSTRUCTION_SPEED as f32));

        // Draw
        draw(
            n_instructions_executed,
            is_debug,
            &debug_state.old_display_state,
            &mut debug_state.info_lines,
        );

        // If debugging: wait for user input to continue
        if is_debug {
            is_debug = debug_terminal(
                &mut n_instructions_executed,
                instruction,
                instruction_raw,
                &mut debug_state,
            );
        }

        // Misc logging
        n_instructions_executed += 1;
    }
}

pub fn draw(
    n_instructions_executed: u128,
    is_debug: bool,
    old_display_state: &[[bool; DISPLAY_HEIGHT]; DISPLAY_WIDTH],
    info_lines: &mut [String],
) {
    // Only draw at ~60FPS
    if n_instructions_executed % 12 == 0 || is_debug {
        // Clear the terminal
        for _ in 0..DISPLAY_HEIGHT + 5 {
            print!("\x1b[2K\x1b[1A\r"); // Clear the line, then move the cursor up a line
        }
        print!("\x1b[2K\r"); // Clear the last line

        print!("{}", (0..=DISPLAY_WIDTH).map(|_| "__").collect::<String>());
        // Show a colored square to indicate sound
        if get_sound_timer() > 0 && n_instructions_executed % 12 == 0 {
            print!(" \x1b[43m  \x1b[0m");
        }

        println!();
        for y in 0..DISPLAY_HEIGHT {
            print!("|");
            for (x, old_row) in old_display_state.iter().enumerate() {
                let is_set = get_display(x as u8, y as u8);
                let is_old_set = old_row[y];
                if is_set && is_old_set {
                    print!("\x1b[47m  \x1b[0m");
                } else if is_set && !is_old_set {
                    print!("\x1b[42m  \x1b[0m");
                } else if !is_set && is_old_set {
                    print!("\x1b[41m  \x1b[0m");
                } else {
                    print!("  ");
                }
            }
            print!("|");
            if is_debug && y < info_lines.len() {
                print!(" {}", info_lines[y]);
            }
            println!();
        }
        println!("|{}|", (0..DISPLAY_WIDTH).map(|_| "__").collect::<String>());
        println!();
        if is_debug {
            println!("Welcome to the debug terminal! h: help, c: continue");
        } else {
            println!();
            println!();
        }
    }
}

pub fn print_debug(
    n_instructions_executed: &mut u128,
    instruction: Instruction,
    instruction_raw: u16,
    debug_state: &mut DebugState,
) {
    // If the current instruction is draw, skip to the next vertical blank
    if let Instruction::Draw(_, _, _) = instruction {
        while *n_instructions_executed % 12 != 1 {
            *n_instructions_executed += 1;
        }
    }

    let info_lines = &mut debug_state.info_lines;

    info!(
        info_lines,
        "|---------------------INSTRUCTIONS-----------------------"
    );
    // Previous instructions
    for instruction in debug_state.last_instructions.iter().rev() {
        info!(
            info_lines,
            "| \x1b[2;37m{:#06X}: {:#04X} -> {:?}\x1b[0m",
            instruction.0,
            instruction.1,
            instruction.2
        );
    }
    // Current instruction
    info!(
        info_lines,
        "| \x1b[32m{:#06X}: {:#04X} -> {:?}\x1b[0m",
        get_pc() - 2,
        instruction_raw,
        instruction
    );
    // Next instruction
    let mut next_instruction;
    let (_, mut next_addr) = (Some(instruction), get_pc() - 2);
    for _ in 0..3 {
        (next_instruction, next_addr) = predict_instruction(next_addr);
        if let Some(ins) = next_instruction {
            info!(
                info_lines,
                "| \x1b[2;37m{:#06X}: {:#04X} -> {:?}\x1b[0m",
                next_addr,
                get_memory_u16(next_addr),
                ins
            );
        } else {
            info!(
                info_lines,
                "| \x1b[2;37m{:#06X}: {:#04X} -> (invalid)\x1b[0m",
                next_addr,
                get_memory_u16(next_addr)
            );
        }
    }

    // Registers + stack
    let stack = get_stack();
    info!(
        info_lines,
        "|-----REGISTERS-----|----STACK({:X})----|------TIMERS------|",
        stack.len()
    );
    for (i, old_reg) in debug_state.old_register_state.iter().enumerate() {
        // Register
        let reg = get_register((i as u8).into());

        if *old_reg != reg {
            // Register changed
            info!(
                info_lines,
                "| \x1b[32mV{:X}: {:#04X}   \x1b[2;37m{:#04X}\x1b[0m   |", i, reg, old_reg
            );
        } else {
            // Register did not change
            info!(info_lines, "| V{:X}: {:#04X}          |", i, reg);
        }

        // Stack
        if i < stack.len() {
            // Stack contains an entry
            infop!(info_lines, "     {:#06X}     |", stack[i]);
        } else {
            // Stack does not contain an entry
            infop!(info_lines, "                |");
        }

        // Delay timer
        if i == 0 {
            infop!(info_lines, "   DELAY   {:#04X}   |", get_delay_timer());
        } else if i == 1 {
            infop!(info_lines, "   SOUND   {:#04X}   |", get_sound_timer());
        } else {
            infop!(info_lines, "                  |");
        }
    }
    info!(
        info_lines,
        "|-------------------|----------------|------------------|"
    );
    // I
    let i_state = (get_i(), get_memory_u8(get_i()), get_memory_u8(get_i() + 2));
    if debug_state.old_i_state != i_state {
        info!(
            info_lines,
            "| \x1b[32mI: {:#06X} -> {:#04X} {:#04X}  \x1b[2;37m{:#06X} -> {:#04X} {:#04X}\x1b[0m           |",
            i_state.0,
            i_state.1,
            i_state.2,
            debug_state.old_i_state.0,
            debug_state.old_i_state.1,
            debug_state.old_i_state.2
        );
    } else {
        info!(
            info_lines,
            "| I: {:#06X} -> {:#04X} {:#04X}                                |",
            i_state.0,
            i_state.1,
            i_state.2
        );
    }
    info!(
        info_lines,
        "|-------------------------------------------------------|"
    );
}

/// Given an instruction, predict the next instruction and its address.
/// This is not always accurate.
fn predict_instruction(addr: u16) -> (Option<Instruction>, u16) {
    let Some(ins) = decode(get_memory_u16(addr)) else {
        return (None, addr + 2);
    };
    match ins {
        Instruction::Jump(nnn) => (decode(get_memory_u16(nnn)), nnn),
        Instruction::JumpOffset(nnn) => (
            decode(get_memory_u16(get_register(Register::V0) as u16 + nnn)),
            nnn,
        ),
        Instruction::SubroutineCall(nnn) => (decode(get_memory_u16(nnn)), nnn),
        Instruction::SubroutineReturn => {
            if let Some(s) = peek_stack() {
                (decode(get_memory_u16(s)), s)
            } else {
                (decode(get_memory_u16(addr + 2)), addr + 2) // TODO change this to be something more clear?
            }
        }
        _ => (decode(get_memory_u16(addr + 2)), addr + 2),
    }
}

/// Fetch the next instruction and increment the PC by 2.
fn fetch() -> u16 {
    let pc = get_pc();
    let instruction = get_memory_u16(pc);
    set_pc(pc + 2);
    instruction
}

fn invalid_instruction(instruction: u16) -> ! {
    panic!("Invalid instruction at {:#x}: {:#x}", get_i(), instruction);
}
