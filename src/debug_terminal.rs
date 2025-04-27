use std::{
    collections::{HashSet, VecDeque},
    io::{self, Write},
    thread,
    time::Duration,
};

use device_query::{DeviceQuery, DeviceState, Keycode};

use c8util::instructions::Instruction;

use crate::{
    run::{draw, print_debug},
    stdin::NonblockingReader,
    system::{
        DISPLAY_HEIGHT, DISPLAY_WIDTH, get_delay_timer, get_i, get_memory_u8, get_pc, get_register,
        get_sound_timer, set_delay_timer, set_i, set_memory_u8, set_pc, set_register,
        set_sound_timer, stack_pop, stack_push,
    },
};

pub struct DebugState {
    /// The last command that was run in the debug terminal.
    /// Used when pressing "enter" on a blank terminal.
    /// NOT used in history.
    pub last_debug_command: String,
    /// The last instructions that were executed.
    pub last_instructions: VecDeque<(u16, u16, Instruction)>,
    /// The lines to print to the right of the screen.
    pub info_lines: Vec<String>,
    /// The state of the registers on the previous frame.
    pub old_register_state: [u8; 16],
    /// The state of I on the previous frame.
    pub old_i_state: (u16, u8, u8),
    /// The state of the display on the previous frame.
    pub old_display_state: [[bool; DISPLAY_HEIGHT]; DISPLAY_WIDTH],
    /// A list of the currently set breakpoints.
    pub breakpoints: HashSet<u16>,
    /// The previous commands run in the session.
    /// Used when pressing up/down in the debug terminal.
    pub history: Vec<String>,
    /// The reader used to read lines from stdin.
    pub reader: NonblockingReader,
    /// The keys that were previously pressed.
    pub last_pressed_keys: Vec<Keycode>,
}

fn print_message(debug_state: &mut DebugState, message: String) {
    print!("\x1b[2K\r> ");
    // This is necessary to clear the [[^A that's printed when arrow keys are pressed
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(1));
        print!("\x1b[2K\r> {message}");
        io::stdout().flush().unwrap();
    });
}

/// Gets the next line to interpret as a command, including history.
fn get_line(debug_state: &mut DebugState) -> String {
    let device_state = DeviceState::new();
    let mut current_history_idx = 0usize;
    loop {
        let keys = device_state.get_keys();

        // History management
        if keys.contains(&Keycode::Up) && !debug_state.last_pressed_keys.contains(&Keycode::Up) {
            // Go back in history
            if debug_state.history.is_empty() {
                print!("\x1b[2K\r> ");
                io::stdout().flush().unwrap();
                continue;
            }
            current_history_idx = current_history_idx
                .saturating_add(1)
                .min(debug_state.history.len());
            let message =
                debug_state.history[debug_state.history.len() - current_history_idx].clone();
            debug_state.reader.set_contents(message.clone());
            print_message(debug_state, message);
        }

        if keys.contains(&Keycode::Down) && !debug_state.last_pressed_keys.contains(&Keycode::Down)
        {
            // Go forward in history
            if debug_state.history.is_empty() {
                print!("\x1b[2K\r> ");
                io::stdout().flush().unwrap();
                continue;
            }
            current_history_idx = current_history_idx.saturating_sub(1);
            if current_history_idx == debug_state.history.len() {
                current_history_idx = current_history_idx.saturating_sub(1);
            }
            let a = debug_state.history.len() - current_history_idx;

            let message = format!(
                "\x1b[2K\r> {}",
                if a < debug_state.history.len() {
                    debug_state.history[debug_state.history.len() - current_history_idx].clone()
                } else {
                    String::new()
                }
            );
            debug_state.reader.set_contents(message.clone());
            print_message(debug_state, message);
        }
        if let Some(line) = debug_state.reader.readline() {
            return line;
        }

        debug_state.last_pressed_keys = keys;
    }
}

/// Handles the debug terminal, and returns whether debug mode should stay enabled.
#[allow(clippy::too_many_lines)]
pub fn debug_terminal(
    n_instructions_executed: &mut u128,
    instruction: Instruction,
    instruction_raw: u16,
    debug_state: &mut DebugState,
) -> bool {
    loop {
        // Clear the line, print the prompt, and move the cursor to the end of the prompt
        println!("\x1b[2K\r> ");
        print!("\x1b[1A\x1b[2C");
        io::stdout().flush().unwrap();

        let mut line = get_line(debug_state);
        // let mut line = String::new();
        // io::stdin().read_line(&mut line).unwrap();

        // Remove escapes
        line = line.replace(['\x1b'], "");

        if line.trim() == "" {
            line = debug_state.last_debug_command.clone();
        }

        if debug_state
            .history
            .last()
            .is_none_or(|val| *val != line.trim())
        {
            debug_state.history.push(line.trim().to_string());
        }

        let args = line.trim().split(' ').collect::<Vec<_>>();
        match args[0] {
            // Print help
            "h" | "help" => {
                debug_state.last_debug_command.clear();
                debug_state.last_debug_command.push_str(line.trim());
                if args.len() > 1 {
                    print!("Unexpected args for command {}: ", args[0]);
                    for arg in &args[1..] {
                        print!("{arg} ");
                    }
                    println!();
                    continue;
                }
                println!("b | breakpoint  Manage breakpoints");
                println!("                    Usage:");
                println!(
                    "                        <b | breakpoint> <address>               Set a breakpoint at the given address"
                );
                println!(
                    "                        <b | breakpoint> <l | list>              List all breakpoints"
                );
                println!(
                    "                        <b | breakpoint> <r | remove> <address>  Remove the breakpoint at the given address"
                );
                println!("                    Valid formats for address are:");
                println!("                        123     Number");
                println!("                        0x123   Hex");
                println!("                        0b101   Binary");
                println!();
                println!("c, continue     Exit debug mode and continue program execution");
                println!();
                println!("h, help         Print this message");
                println!();
                println!(
                    "j, jump         Set PC to the given address. Addresses must be <= 12-bit."
                );
                println!("                    Usage: <j | jump> <address>");
                println!("                        Valid formats for addresses are:");
                println!("                            123     Number");
                println!("                            0x123   Hex");
                println!("                            0b101   Binary");
                println!();
                println!("n, next         Execute the next instruction");
                println!();
                println!(
                    "p, print        Print the value in the given register or at the given address"
                );
                println!("                    Usage: <p | print> <target>");
                println!("                    Valid targets are:");
                println!("                        VX         Register VX");
                println!("                        i, index   Register I");
                println!("                        pc         Register PC");
                println!("                        s, delay   Delay timer");
                println!("                        s, sound   Sound timer");
                println!("                        address    The byte in memory at the address");
                println!("                            Valid formats for addresses are:");
                println!("                                123     Number");
                println!("                                0x123   Hex");
                println!("                                0b101   Binary");
                println!();
                println!("pop             Pop the stack.");
                println!();
                println!("push            Push the given value to the stack.");
                println!("                    Usage: push <value>");
                println!("                    value must be <= 8 bits");
                println!("                    Valid formats for value are:");
                println!("                        123     Number");
                println!("                        0x123   Hex");
                println!("                        0b101   Binary");
                println!();
                println!(
                    "s, set          Set the value in the given register or at the given address"
                );
                println!("                    Usage: <s | set> <target> <value>");
                println!("                    Valid targets are:");
                println!("                        VX         Register VX");
                println!("                            value must be <= 8 bits");
                println!("                        i, index   Register I");
                println!("                            value must be <= 12 bits");
                println!("                        pc         Register PC");
                println!("                            value must be <= 12 bits");
                println!("                        s, delay   Delay timer");
                println!("                            value must be <= 8 bits");
                println!("                        s, sound   Sound timer");
                println!("                            value must be <= 8 bits");
                println!("                        address    The byte in memory at the address");
                println!("                            value must be <= 8 bits");
                println!("                            Valid formats for addresses are:");
                println!("                                123     Number");
                println!("                                0x123   Hex");
                println!("                                0b101   Binary");
                println!();
                println!(
                    "x, examine       Examine (print) the given number of bytes at the given address."
                );
                println!("                    Usage: <x | examine> <num> <address>");
                println!(
                    "                        Valid formats for both addresses and the number of bytes to print are:"
                );
                println!("                            123     Number");
                println!("                            0x123   Hex");
                println!("                            0b101   Binary");
            }
            // Continue program execution
            "c" | "continue" => {
                if args.len() > 1 {
                    print!("Unexpected args for command {}: ", args[0]);
                    for arg in args[1..].iter() {
                        print!("{} ", arg);
                    }
                    println!();
                    continue;
                }
                for _ in 0..DISPLAY_HEIGHT + 5 {
                    println!();
                }
                debug_state.last_debug_command.clear();
                return false;
            }
            // Next instruction
            "n" | "next" => {
                debug_state.last_debug_command.clear();
                debug_state.last_debug_command.push_str(line.trim());
                if args.len() > 1 {
                    print!("Unexpected args for command {}: ", args[0]);
                    for arg in args[1..].iter() {
                        print!("{} ", arg);
                    }
                    println!();
                    continue;
                }
                for _ in 0..DISPLAY_HEIGHT + 5 {
                    println!();
                }
                return true;
            }
            // Jump to the given address.
            "j" | "jump" => {
                debug_state.last_debug_command.clear();
                debug_state.last_debug_command.push_str(line.trim());
                if args.len() != 2 {
                    println!("usage: {} address", args[0]);
                    continue;
                }
                let Some(addr) = str_to_num(args[1]) else {
                    continue;
                };

                if addr & 0x0FFF != addr {
                    println!(
                        "address {:#06X} is too large to jump to (should be 12 bits)",
                        addr
                    );
                }
                set_pc(addr as u16);
                debug_redraw(
                    debug_state,
                    instruction,
                    instruction_raw,
                    n_instructions_executed,
                );
                continue;
            }
            // Print the value of something
            // - v{x}: VX
            // - i: I
            // - pc: PC
            // - d | delay: Delay timer
            // - s | sound: Sound timer
            // - addr: Memory
            "p" | "print" => {
                debug_state.last_debug_command.clear();
                debug_state.last_debug_command.push_str(line.trim());
                if args.len() != 2 {
                    println!("invalid usage of command {}", args[0]);
                    continue;
                }
                // Registers
                if args[1].starts_with(['v', 'V']) {
                    if args[1].len() != 2 {
                        println!("invalid usage of command {}", args[0]);
                        continue;
                    }
                    let reg_idx = match usize::from_str_radix(&args[1][1..2], 16) {
                        Ok(val) => val,
                        Err(e) => {
                            println!("could not parse hex value {}: {}", args[1], e);
                            continue;
                        }
                    };
                    println!("{:#04X}", get_register((reg_idx as u8).into()));
                    continue;
                }
                match args[1] {
                    "i" | "index" => {
                        println!("{:#06X}", get_i());
                    }
                    "pc" => {
                        println!("{:#06X}", get_pc());
                    }
                    "d" | "delay" => {
                        println!("{:#04X}", get_delay_timer());
                    }
                    "s" | "sound" => {
                        println!("{:#04X}", get_sound_timer());
                    }
                    // Unknown => try to interpret as an address
                    _ => {
                        let Some(addr) = str_to_num(args[1]) else {
                            continue;
                        };
                        if addr & 0x0FFF != addr {
                            println!(
                                "address {:#06X} is too large to jump to (should be 12 bits)",
                                addr
                            );
                        }
                        println!("{:#04X}", get_memory_u8(addr as u16));
                    }
                }
                continue;
            }
            // Set something to a value
            // - v{x}: VX
            // - i: I
            // - pc: PC
            // - d | delay: Delay timer
            // - s | sound: Sound timer
            // - addr: Memory
            "s" | "set" => {
                debug_state.last_debug_command.clear();
                debug_state.last_debug_command.push_str(line.trim());
                if args.len() != 3 {
                    println!("invalid usage of command {}", args[0]);
                    continue;
                }
                let Some(val) = str_to_num(args[2]) else {
                    continue;
                };
                // Registers
                if args[1].starts_with(['v', 'V']) {
                    if args[1].len() != 2 {
                        println!("invalid usage of command {}", args[0]);
                        continue;
                    }
                    let reg_idx = match usize::from_str_radix(&args[1][1..2], 16) {
                        Ok(val) => val,
                        Err(e) => {
                            println!("could not parse hex value {}: {}", args[1], e);
                            continue;
                        }
                    };
                    if val & 0xFF != val {
                        println!(
                            "could not set {}: value ({}) was more than 8 bits",
                            args[1], args[2]
                        );
                        continue;
                    }
                    set_register((reg_idx as u8).into(), val as u8);
                    debug_redraw(
                        debug_state,
                        instruction,
                        instruction_raw,
                        n_instructions_executed,
                    );
                    continue;
                }
                match args[1] {
                    "i" | "index" => {
                        if val & 0x0FFF != val {
                            println!(
                                "could not set {}: value ({}) was more than 12 bits",
                                args[1], args[2]
                            );
                            continue;
                        }
                        set_i(val as u16);
                        debug_redraw(
                            debug_state,
                            instruction,
                            instruction_raw,
                            n_instructions_executed,
                        );
                        continue;
                    }
                    "pc" => {
                        let val = val + 2;
                        if val & 0x0FFF != val {
                            println!(
                                "could not set {}: value ({}) was more than 12 bits",
                                args[1], args[2]
                            );
                            continue;
                        }
                        set_pc(val as u16);
                        debug_redraw(
                            debug_state,
                            instruction,
                            instruction_raw,
                            n_instructions_executed,
                        );
                        continue;
                    }
                    "d" | "delay" => {
                        if val & 0xFF != val {
                            println!(
                                "could not set {}: value ({}) was more than 8 bits",
                                args[1], args[2]
                            );
                            continue;
                        }
                        set_delay_timer(val as u8);
                        debug_redraw(
                            debug_state,
                            instruction,
                            instruction_raw,
                            n_instructions_executed,
                        );
                        continue;
                    }
                    "s" | "sound" => {
                        if val & 0xFF != val {
                            println!(
                                "could not set {}: value ({}) was more than 8 bits",
                                args[1], args[2]
                            );
                            continue;
                        }
                        set_sound_timer(val as u8);
                        debug_redraw(
                            debug_state,
                            instruction,
                            instruction_raw,
                            n_instructions_executed,
                        );
                        continue;
                    }
                    // Unknown => try to interpret as an address
                    _ => {
                        let Some(addr) = str_to_num(args[1]) else {
                            continue;
                        };
                        if addr & 0x0FFF != addr {
                            println!(
                                "address {:#06X} is too large to jump to (should be 12 bits)",
                                addr
                            );
                            continue;
                        }
                        if val & 0xFF != val {
                            println!(
                                "could not set memory at {}: value ({}) was more than 8 bits",
                                args[1], args[2]
                            );
                            continue;
                        }
                        set_memory_u8(addr as u16, val as u8);
                        continue;
                    }
                }
            }
            // Push to stack
            "push" => {
                debug_state.last_debug_command.clear();
                debug_state.last_debug_command.push_str(line.trim());
                if args.len() != 2 {
                    println!("invalid usage of command {}", args[0]);
                    continue;
                }
                let Some(addr) = str_to_num(args[1]) else {
                    continue;
                };
                if addr & 0x0FFF != addr {
                    println!(
                        "address {:#06X} is too large to push to stack (should be 12 bits)",
                        addr
                    );
                    continue;
                }
                stack_push(addr as u16);
                debug_redraw(
                    debug_state,
                    instruction,
                    instruction_raw,
                    n_instructions_executed,
                );
                continue;
            }
            // Pop from stack
            "pop" => {
                debug_state.last_debug_command.clear();
                debug_state.last_debug_command.push_str(line.trim());
                match stack_pop() {
                    None => {
                        println!("could not pop: stack was empty");
                        continue;
                    }
                    Some(val) => {
                        println!("{:#06X}", val);
                        debug_redraw(
                            debug_state,
                            instruction,
                            instruction_raw,
                            n_instructions_executed,
                        );
                        continue;
                    }
                }
            }
            // Manage breakpoints
            "b" | "breakpoint" => {
                // b 0x200: Set a breakpoint at 0x200
                // b l | list: List breakpoints
                // b r | remove 0x200: Delete the breakpoint at 0x200
                debug_state.last_debug_command.clear();
                debug_state.last_debug_command.push_str(line.trim());
                if args.len() < 2 {
                    println!("invalid usage of command {}", args[0]);
                    continue;
                }
                match args[1] {
                    // List breakpoints
                    "l" | "list" => {
                        if args.len() != 2 {
                            println!("invalid usage of command {}", args[0]);
                            continue;
                        }
                        let mut breakpoints = debug_state.breakpoints.iter().collect::<Vec<_>>();
                        breakpoints.sort();
                        for b in breakpoints {
                            println!("{:#06X}", b);
                        }
                    }
                    // Delete a breakpoint
                    "r" | "remove" => {
                        if args.len() != 3 {
                            println!("invalid usage of command {}", args[0]);
                            continue;
                        }
                        let Some(addr) = str_to_num(args[2]) else {
                            continue;
                        };
                        if addr & 0x0FFF != addr {
                            // This address will never be in breakpoints
                            continue;
                        }
                        if !debug_state.breakpoints.remove(&(addr as u16)) {
                            println!("address {:#06X} was not a breakpoint", addr);
                        }
                        continue;
                    }
                    // Add a new breakpoint
                    _ => {
                        if args.len() != 2 {
                            println!("invalid usage of command {}", args[0]);
                            continue;
                        }
                        let Some(addr) = str_to_num(args[1]) else {
                            continue;
                        };
                        if addr & 0x0FFF != addr {
                            println!("address {:#06X} is too large (should be 12 bits)", addr);
                            continue;
                        }
                        if !debug_state.breakpoints.insert(addr as u16) {
                            println!("address {:#06X} was already a breakpoint", addr);
                        }
                        continue;
                    }
                }
            }
            "x" | "examine" => {
                if args.len() != 3 {
                    println!("invalid usage of command {}", args[0]);
                    continue;
                }
                let Some(n) = str_to_num(args[1]) else {
                    continue;
                };
                let Some(addr) = str_to_num(args[2]) else {
                    continue;
                };

                // Print memory at 8 bytes per line
                for i in 0..n {
                    if i % 8 == 0 {
                        if i != 0 {
                            println!();
                        }
                        print!("{:#06X}:  ", addr + i);
                    }
                    print!("{:#04X} ", get_memory_u8((addr + i) as u16));
                }
                println!();
                continue;
            }
            // Key press
            // Key release
            // Unknown instruction or blank line
            _ => {
                if line.trim() != "" {
                    println!("Unknown command: {}", line.trim());
                }
            }
        };
    }
}

/// Redraw the screen in debug mode.
/// This is not a full redraw, and it should only be used when things like registers are changed in debug mode but we don't want to advance another instruction.
fn debug_redraw(
    debug_state: &mut DebugState,
    instruction: Instruction,
    instruction_raw: u16,
    n_instructions_executed: &mut u128,
) {
    for _ in 0..DISPLAY_HEIGHT + 5 {
        println!();
    }
    debug_state.info_lines.clear();
    print_debug(
        n_instructions_executed,
        instruction,
        instruction_raw,
        debug_state,
    );
    draw(
        *n_instructions_executed,
        true,
        &debug_state.old_display_state,
        &mut debug_state.info_lines,
    );
    io::stdout().flush().unwrap();
}

/// Try to convert the given string to a number.
/// Supports hex (0x123), binary (0b111), and base 10 (123).
fn str_to_num(addr: &str) -> Option<usize> {
    if addr.contains("0x") {
        match usize::from_str_radix(&addr[2..], 16) {
            Ok(val) => Some(val),
            Err(e) => {
                println!("could not parse hex value {}: {}", addr, e);
                None
            }
        }
    } else if addr.contains("0b") {
        match usize::from_str_radix(&addr[2..], 2) {
            Ok(val) => Some(val),
            Err(e) => {
                println!("could not parse binary value {}: {}", addr, e);
                None
            }
        }
    } else {
        match addr.parse::<usize>() {
            Ok(val) => Some(val),
            Err(e) => {
                println!("could not parse base 10 value {}: {}", addr, e);
                None
            }
        }
    }
}
