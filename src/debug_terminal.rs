use std::{
    collections::VecDeque,
    io::{self, Write},
};

use crate::{
    instructions::Instruction,
    run::{draw, print_debug},
    system::{DISPLAY_HEIGHT, DISPLAY_WIDTH, set_pc},
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
                println!("                 Valid formats for addresses are:");
                println!("                     123     Number");
                println!("                     0x123   Hex");
                println!("                     0b101   Binary");
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
                let addr = args[1];
                let addr = if addr.contains("0x") {
                    match usize::from_str_radix(&addr[2..], 16) {
                        Ok(val) => val,
                        Err(e) => {
                            println!("could not parse hex value {}: {}", addr, e);
                            continue;
                        }
                    }
                } else if addr.contains("0b") {
                    match usize::from_str_radix(&addr[2..], 2) {
                        Ok(val) => val,
                        Err(e) => {
                            println!("could not parse binary value {}: {}", addr, e);
                            continue;
                        }
                    }
                } else {
                    match addr.parse::<usize>() {
                        Ok(val) => val,
                        Err(e) => {
                            println!("could not parse base 10 value {}: {}", addr, e);
                            continue;
                        }
                    }
                };

                if addr & 0x0FFF != addr {
                    println!(
                        "address {:#X} is too large to jump to (should be 12 bits)",
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
            // - VX
            // - I
            // - PC
            // - Delay timer
            // - Sound timer
            // - Memory
            "p" | "print" => {
                debug_state.last_debug_command.clear();
                debug_state.last_debug_command.push_str(line.trim());
                println!("TODO");
            }
            // Set something to a value
            // - VX
            // - I
            // - PC
            // - Delay timer
            // - Sound timer
            // - Memory
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
