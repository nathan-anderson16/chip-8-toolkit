use std::{io, thread, time::Duration};

use crate::{
    instructions::Instruction,
    system::{
        DISPLAY_HEIGHT, DISPLAY_WIDTH, Register, get_display, get_i, get_memory_u8, get_memory_u16,
        get_pc, get_register, set_display, set_i, set_pc, set_register,
    },
};

/// The number of instructions to execute per second.
pub const INSTRUCTION_SPEED: usize = 700;

/// Handles the core loop.
pub fn run() {
    loop {
        // Fetch the next instruction
        let instruction_raw = fetch();

        // Decode the instruction
        let instruction = decode(instruction_raw);

        // Execute the instruction
        execute(instruction);

        // Count down delay and sound timers

        // Delay for 1/700 of a second
        thread::sleep(Duration::from_secs_f32(1.0 / INSTRUCTION_SPEED as f32));

        // Draw
        for y in 0..DISPLAY_HEIGHT {
            for x in 0..DISPLAY_WIDTH {
                let is_set = get_display(x as u8, y as u8);
                print!("{}", if is_set { "-" } else { " " });
            }
            println!();
        }

        // If debugging: wait for user input to continue
        // let mut buf = String::new();
        // io::stdin().read_line(&mut buf).unwrap();
    }
}

/// Fetch the next instruction and increment the PC by 2.
fn fetch() -> u16 {
    let pc = get_pc();
    let instruction = get_memory_u16(pc);
    set_pc(pc + 2);
    instruction
}

fn decode(ins: u16) -> Instruction {
    let first = ((ins & 0xF000) >> 12) as u8;
    let second = ((ins & 0x0F00) >> 8) as u8;
    let third = ((ins & 0x00F0) >> 4) as u8;
    let fourth = (ins & 0x000F) as u8;

    match first {
        0x0 => match second {
            0x0 => match third {
                0xE => match fourth {
                    0x0 => Instruction::Clear,
                    _ => todo!(),
                },
                _ => todo!(),
            },
            _ => todo!(),
        },
        0x1 => Instruction::Jump(ins & 0x0FFF),
        0x6 => Instruction::SetRegister(second.into(), (ins & 0x00FF) as u8),
        0x7 => Instruction::Add(second.into(), (ins & 0x00FF) as u8),
        0xA => Instruction::SetIndexRegister(ins & 0xFFF),
        0xD => Instruction::Draw(second.into(), third.into(), fourth),
        _ => todo!(),
    }
}

fn execute(instruction: Instruction) {
    match instruction {
        Instruction::Clear => {
            // println!("Executing instruction: clear");
            for i in 0..DISPLAY_WIDTH {
                for j in 0..DISPLAY_HEIGHT {
                    set_display(i as u8, j as u8, false);
                }
            }
        }
        Instruction::Jump(addr) => {
            // println!("Executing instruction: jump ({addr})");
            set_pc(addr)
        }
        Instruction::SetRegister(reg, val) => {
            // println!("Executing instruction: set register ({reg:?}) ({val})");
            set_register(reg, val)
        }
        Instruction::Add(reg, val) => {
            // println!("Executing instruction: add ({reg:?}) ({val})");
            let added = get_register(reg) as u16 + val as u16;
            set_register(reg, (added % 255) as u8);
        }
        Instruction::SetIndexRegister(val) => {
            // println!("Executing instruction: set i ({val})");
            set_i(val)
        }
        Instruction::Draw(vx, vy, n) => {
            set_register(Register::VF, 0);

            let sprite_location = get_i();
            let x = get_register(vx) % DISPLAY_WIDTH as u8;
            let y = get_register(vy) % DISPLAY_HEIGHT as u8;

            // Draw each pixel to the screen
            let n_x_pixels_to_draw = (x + 8).min(DISPLAY_WIDTH as u8) - x;
            let n_y_pixels_to_draw = (y + n).min(DISPLAY_HEIGHT as u8) - y;
            for i in 0..n_y_pixels_to_draw {
                let sprite_val = get_memory_u8(sprite_location + i as u16);
                let display_y = y + i;

                for j in (0..n_x_pixels_to_draw).rev() {
                    let is_set = ((sprite_val >> j) & 0x1) != 0;

                    let display_x = x + (n_x_pixels_to_draw - j);
                    let display_val = get_display(display_x, display_y);
                    let new_display_val = display_val ^ is_set;

                    if is_set {
                        set_display(display_x, display_y, new_display_val);

                        if display_val {
                            set_register(Register::VF, 1);
                        }
                    }
                }
            }
        }
        _ => todo!(),
    }
}
