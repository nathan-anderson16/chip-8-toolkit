use std::{
    collections::VecDeque,
    io::{self, Write},
};

use crate::{
    instructions::Instruction,
    run::{draw, print_debug},
    system::{
        DISPLAY_HEIGHT, DISPLAY_WIDTH, get_delay_timer, get_i, get_memory_u8, get_pc, get_register,
        get_sound_timer, set_pc,
    },
};

pub struct DebugState {
    pub last_debug_command: String,
    pub last_instructions: VecDeque<(u16, u16, Instruction)>,
    pub info_lines: Vec<String>,
    pub old_register_state: [u8; 16],
    pub old_i_state: (u16, u8, u8),
    pub old_display_state: [[bool; DISPLAY_HEIGHT]; DISPLAY_WIDTH],
}

/// Handles the debug terminal, and returns whether debug mode should stay enabled.
pub fn debug_terminal(
    n_instructions_executed: &mut u128,
    instruction: Instruction,
    instruction_raw: u16,
    debug_state: &mut DebugState,
) -> bool {
    loop {
        println!("> ");
        print!("\x1b[1A\x1b[2C");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();

        // Remove escapes
        line = line.replace(['\x1b'], "");

        if line.trim() == "" {
            line = debug_state.last_debug_command.clone();
        }
        let args = line.trim().split(" ").collect::<Vec<_>>();
        match args[0] {
            // Print help
            "h" | "help" => {
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
                println!("h, help          Print this message");
                println!("c, continue      Exit debug mode and continue program execution");
                println!("n, next          Execute the next instruction");
                println!(
                    "j, jump address  Set PC to the given address. Addresses must be <= 12-bit."
                );
                println!("                     Valid formats for addresses are:");
                println!("                         123     Number");
                println!("                         0x123   Hex");
                println!("                         0b101   Binary");
                println!(
                    "p, print         Print the value in the given register or at the given address"
                );
                println!("                 Valid registers to print are:");
                println!("                     VX         Register VX");
                println!("                     i, index   Register I");
                println!("                     pc         Register PC");
                println!("                     s, delay   Delay timer");
                println!("                     s, sound   Sound timer");
                println!("                     address    The byte in memory at the address");
                println!("                         Valid formats for addresses are:");
                println!("                             123     Number");
                println!("                             0x123   Hex");
                println!("                             0b101   Binary");
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
                println!("TODO");
            }
            // Push to stack
            "push" => {
                debug_state.last_debug_command.clear();
                debug_state.last_debug_command.push_str(line.trim());
                println!("TODO");
            }
            // Pop from stack
            "pop" => {
                debug_state.last_debug_command.clear();
                debug_state.last_debug_command.push_str(line.trim());
                println!("TODO");
            }
            // Manage breakpoints
            "b" | "breakpoint" => {
                debug_state.last_debug_command.clear();
                debug_state.last_debug_command.push_str(line.trim());
                // b 0x200: Set a breakpoint at 0x200
                // b l | list: List breakpoints
                // b d | delete 0x200: Delete the breakpoint at 0x200
                println!("TODO");
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
