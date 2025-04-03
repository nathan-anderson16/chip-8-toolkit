use core::panic;
#[allow(unused_imports)] // used in debugging
use std::io;
use std::{
    collections::{HashMap, HashSet},
    sync::OnceLock,
    thread,
    time::{Duration, SystemTime},
};

use device_query::{DeviceQuery, DeviceState, Keycode};

use crate::{
    instructions::Instruction,
    system::{
        DISPLAY_HEIGHT, DISPLAY_WIDTH, Register, decrement_delay_timer, decrement_sound_timer,
        get_delay_timer, get_display, get_i, get_memory_u8, get_memory_u16, get_pc, get_register,
        get_sound_timer, set_delay_timer, set_display, set_i, set_memory_u8, set_memory_u16,
        set_pc, set_register, set_sound_timer, stack_pop, stack_push,
    },
};

/// The number of instructions to execute per second.
pub const INSTRUCTION_SPEED: usize = 700;

pub static KEYPRESS_MAP: OnceLock<HashMap<Keycode, u8>> = OnceLock::new();
pub static REVERSE_KEYPRESS_MAP: OnceLock<HashMap<u8, Keycode>> = OnceLock::new();

/// Handles the core loop.
pub fn run() {
    for _ in 0..DISPLAY_HEIGHT {
        println!();
    }

    let mut n_instructions_executed = 0u128;

    let device_state = DeviceState::new();
    let mut pressed_keys: HashSet<Keycode> = HashSet::new();
    // Used for GetKey
    #[allow(unused_assignments)] // it thinks this is unused
    let mut last_pressed_keys: HashSet<Keycode> = HashSet::new();

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

    loop {
        // Update keyboard state
        let keys = device_state.get_keys();
        last_pressed_keys = pressed_keys.clone();
        pressed_keys.clear();
        for key in keys {
            pressed_keys.insert(key);
        }

        // Fetch the next instruction
        let instruction_raw = fetch();

        // Decode the instruction
        let instruction = decode(instruction_raw);

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
        // Only draw at ~60FPS
        if n_instructions_executed % 12 == 0 {
            // if true {
            // Clear the terminal
            // print!("\x1b[1A"); // Move cursor up 1 line
            for _ in 0..DISPLAY_HEIGHT + 3 {
                print!("\x1b[2K\x1b[1A\r"); // Clear the line, then move the cursor up a line
            }
            print!("\x1b[2K\r"); // Clear the last line

            print!("{}", (0..DISPLAY_WIDTH).map(|_| "__").collect::<String>());
            // Show a colored square to indicate sound
            if get_sound_timer() > 0 && n_instructions_executed % 12 == 0 {
                print!(" \x1b[43m  \x1b[0m");
            }

            // print!(
            //     "{} pressed keys: {:?}, pc: {:#x}, current instruction: {:#x} ({:?}), next instruction: {:#x}",
            //     (0..DISPLAY_WIDTH).map(|_| "_").collect::<String>(),
            //     pressed_keys,
            //     get_pc(),
            //     get_memory_u16(get_pc() - 2),
            //     instruction,
            //     get_memory_u16(get_pc())
            // );
            println!();
            for y in 0..DISPLAY_HEIGHT {
                for x in 0..DISPLAY_WIDTH {
                    let is_set = get_display(x as u8, y as u8);
                    print!("{}", if is_set { "\x1b[47m  \x1b[0m" } else { "  " });
                }
                println!("|");
            }
            println!("{}", (0..DISPLAY_WIDTH).map(|_| "__").collect::<String>());
            println!();
        }

        // If debugging: wait for user input to continue
        // let mut buf = String::new();
        // io::stdin().read_line(&mut buf).unwrap();

        // Misc logging
        n_instructions_executed += 1;
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

fn decode(ins: u16) -> Instruction {
    let first = ((ins & 0xF000) >> 12) as u8;
    let second = ((ins & 0x0F00) >> 8) as u8;
    let third = ((ins & 0x00F0) >> 4) as u8;
    let fourth = (ins & 0x000F) as u8;

    match first {
        0x0 => match second {
            0x0 => match third {
                0xE => match fourth {
                    0xE => Instruction::SubroutineReturn,
                    0x0 => Instruction::Clear,
                    _ => invalid_instruction(ins),
                },
                _ => invalid_instruction(ins),
            },
            _ => invalid_instruction(ins),
        },
        0x1 => Instruction::Jump(ins & 0x0FFF),
        0x2 => Instruction::SubroutineCall(ins & 0x0FFF),
        0x3 => Instruction::SkipConditional1(second.into(), (ins & 0x00FF) as u8),
        0x4 => Instruction::SkipConditional2(second.into(), (ins & 0x00FF) as u8),
        0x5 => match fourth {
            0 => Instruction::SkipConditional3(second.into(), third.into()),
            _ => invalid_instruction(ins),
        },
        0x6 => Instruction::SetRegister(second.into(), (ins & 0xff) as u8),
        0x7 => Instruction::Add(second.into(), (ins & 0x00FF) as u8),
        0x8 => match fourth {
            0 => Instruction::RegSet(second.into(), third.into()),
            1 => Instruction::BinaryOr(second.into(), third.into()),
            2 => Instruction::BinaryAnd(second.into(), third.into()),
            3 => Instruction::BinaryXor(second.into(), third.into()),
            4 => Instruction::RegAdd(second.into(), third.into()),
            5 => Instruction::Subtract1(second.into(), third.into()),
            6 => Instruction::ShiftRight(second.into(), third.into()),
            7 => Instruction::Subtract2(second.into(), third.into()),
            0xE => Instruction::ShiftLeft(second.into(), third.into()),
            _ => invalid_instruction(ins),
        },
        0x9 => match fourth {
            0 => Instruction::SkipConditional4(second.into(), third.into()),
            _ => invalid_instruction(ins),
        },
        0xA => Instruction::SetIndexRegister(ins & 0xFFF),
        0xB => Instruction::JumpOffset(ins & 0xFFF),
        0xC => Instruction::Random(second.into(), (ins & 0x00FF) as u8),
        0xD => Instruction::Draw(second.into(), third.into(), fourth),
        0xE => match ins & 0x00FF {
            0x9E => Instruction::SkipIfKey(second.into()),
            0xA1 => Instruction::SkipIfNotKey(second.into()),
            _ => invalid_instruction(ins),
        },
        0xF => match ins & 0x00FF {
            0x07 => Instruction::GetDelayTimer(second.into()),
            0x0A => Instruction::GetKey(second.into()),
            0x15 => Instruction::SetDelayTimer(second.into()),
            0x18 => Instruction::SetSoundTimer(second.into()),
            0x1E => Instruction::AddToIndex(second.into()),
            0x29 => Instruction::FontCharacter(second.into()),
            0x33 => Instruction::BCD(second.into()),
            0x55 => Instruction::StoreMemory(second),
            0x65 => Instruction::LoadMemory(second),
            _ => invalid_instruction(ins),
        },
        _ => invalid_instruction(ins),
    }
}

fn execute(
    instruction: Instruction,
    pressed_keys: &HashSet<Keycode>,
    last_pressed_keys: &HashSet<Keycode>,
    n_instructions_executed: u128,
) {
    match instruction {
        // 0NNN
        Instruction::ExecuteMachineLanguageRoutine => {
            panic!("Instruction ExecuteMachineLanguageRoutine (0NNN) is illegal!");
        }
        // 00E0
        Instruction::Clear => {
            // println!("Executing instruction: clear");
            for i in 0..DISPLAY_WIDTH {
                for j in 0..DISPLAY_HEIGHT {
                    set_display(i as u8, j as u8, false);
                }
            }
        }
        // 00EE
        Instruction::SubroutineReturn => {
            // println!("Executing instruction: subroutine return");
            set_pc(
                stack_pop()
                    .expect("attempted to execute a subroutine return when the stack was empty"),
            );
        }
        // 1NNN
        Instruction::Jump(nnn) => {
            // println!("Executing instruction: jump ({addr})");
            set_pc(nnn);
        }
        // 2NNN
        Instruction::SubroutineCall(nnn) => {
            // println!("Executing instruction: subroutine call ({addr})");
            stack_push(get_pc());
            set_pc(nnn);
        }
        // 3XNN
        Instruction::SkipConditional1(vx, nn) => {
            if get_register(vx) == nn {
                set_pc(get_pc() + 2);
            }
        }
        // 4XNN
        Instruction::SkipConditional2(vx, nn) => {
            if get_register(vx) != nn {
                set_pc(get_pc() + 2);
            }
        }
        // 5XNN
        Instruction::SkipConditional3(vx, vy) => {
            if get_register(vx) == get_register(vy) {
                set_pc(get_pc() + 2);
            }
        }
        // 6XNN
        Instruction::SetRegister(vx, nn) => {
            // println!("Executing instruction: set register ({reg:?}) ({val})");
            set_register(vx, nn)
        }
        // 7XNN
        Instruction::Add(vx, nn) => {
            // println!("Executing instruction: add ({reg:?}) ({val})");
            set_register(vx, get_register(vx).wrapping_add(nn));
        }
        // 8XY0
        Instruction::RegSet(vx, vy) => {
            // println!("Executing instruction: register set ({vx:?}, {vy:?});
            set_register(vx, get_register(vy));
        }
        // 8XY1
        Instruction::BinaryOr(vx, vy) => {
            // println!("Executing instruction: binary or ({vx:?}, {vy:?});
            set_register(vx, get_register(vx) | get_register(vy));
            set_register(Register::VF, 0);
        }
        // 8XY2
        Instruction::BinaryAnd(vx, vy) => {
            // println!("Executing instruction: binary and ({vx:?}, {vy:?});
            set_register(vx, get_register(vx) & get_register(vy));
            set_register(Register::VF, 0);
        }
        // 8XY3
        Instruction::BinaryXor(vx, vy) => {
            // println!("Executing instruction: binary xor ({vx:?}, {vy:?});
            set_register(vx, get_register(vx) ^ get_register(vy));
            set_register(Register::VF, 0);
        }
        // 8XY4
        Instruction::RegAdd(vx, vy) => {
            let sum = get_register(vx) as u16 + get_register(vy) as u16;
            set_register(vx, (sum % 255) as u8);
            set_register(Register::VF, if sum > 255 { 1 } else { 0 });
        }
        // 8XY5
        Instruction::Subtract1(vx, vy) => {
            let subbed = get_register(vx) as i16 - get_register(vy) as i16;
            set_register(vx, get_register(vx).wrapping_sub(get_register(vy)));
            set_register(Register::VF, if subbed < 0 { 0 } else { 1 });
        }
        // 8XY6
        Instruction::ShiftRight(vx, vy) => {
            set_register(vx, get_register(vy)); // TODO: Add option to disable
            let old_vx = get_register(vx);
            set_register(vx, (get_register(vx) >> 1) & 0b01111111);
            set_register(Register::VF, old_vx & 1);
        }
        // 8XY7
        Instruction::Subtract2(vx, vy) => {
            let subbed = get_register(vy) as i16 - get_register(vx) as i16;
            set_register(vx, get_register(vy).wrapping_sub(get_register(vx)));
            set_register(Register::VF, if subbed < 0 { 0 } else { 1 });
        }
        // 8XYE
        Instruction::ShiftLeft(vx, vy) => {
            set_register(vx, get_register(vy)); // TODO: Add option to disable
            let old_vx = get_register(vx);
            set_register(vx, (get_register(vx) << 1) & 0b11111110);
            set_register(
                Register::VF,
                if old_vx & 0b10000000 == 0b10000000 {
                    1
                } else {
                    0
                },
            );
        }
        // 9XY0
        Instruction::SkipConditional4(vx, vy) => {
            if get_register(vx) != get_register(vy) {
                set_pc(get_pc() + 2);
            }
        }
        // ANNN
        Instruction::SetIndexRegister(nnn) => {
            // println!("Executing instruction: set i ({val})");
            set_i(nnn);
        }
        // BNNN
        Instruction::JumpOffset(nnn) => {
            set_pc(nnn + get_register(Register::V0) as u16);
        }
        // CXNN
        Instruction::Random(vx, nnn) => {
            let duration_since_epoch = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap();
            let timestamp_nanos = duration_since_epoch.as_nanos();

            set_register(vx, (timestamp_nanos & (nnn as u128)) as u8);
        }
        // DXYN
        Instruction::Draw(vx, vy, n) => {
            // Wait until just after vblank to draw
            if (n_instructions_executed % 12) != 1 {
                set_pc(get_pc() - 2);
                return;
            }
            set_register(Register::VF, 0);

            let sprite_location = get_i();
            let x = get_register(vx) % DISPLAY_WIDTH as u8;
            let y = get_register(vy) % DISPLAY_HEIGHT as u8;

            // Draw each pixel to the screen
            for i in 0..n {
                let display_y = y + i;
                if display_y as usize >= DISPLAY_HEIGHT {
                    continue;
                }
                let sprite_val = get_memory_u8(sprite_location + i as u16);

                for j in (0..8).rev() {
                    let display_x = x + 8 - j - 1;
                    if display_x as usize >= DISPLAY_WIDTH {
                        continue;
                    }
                    let is_set = ((sprite_val >> j) & 0x1) != 0;
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
        // EX9E
        Instruction::SkipIfKey(vx) => {
            // println!("Executing instruction: skip if key ({reg}: get_register(reg))");
            let key = get_register(vx);
            // println!("{}", key);
            if let Some(keycode) = REVERSE_KEYPRESS_MAP.get().unwrap().get(&key) {
                if pressed_keys.contains(keycode) {
                    set_pc(get_pc() + 2);
                }
            }
        }
        // EXA1
        Instruction::SkipIfNotKey(vx) => {
            // println!("Executing instruction: skip if not key ({reg}: get_register(reg))");
            let key = get_register(vx);
            let keycode = REVERSE_KEYPRESS_MAP.get().unwrap().get(&key).unwrap();
            if !pressed_keys.contains(keycode) {
                set_pc(get_pc() + 2);
            }
        }
        // FX07
        Instruction::GetDelayTimer(vx) => {
            // println!("Executing instruction: get delay timer ({reg:?})");
            set_register(vx, get_delay_timer());
        }
        // FX07
        Instruction::SetDelayTimer(vx) => {
            // println!("Executing instruction: set delay timer ({reg:?})");
            set_delay_timer(get_register(vx));
        }
        // FX07
        Instruction::SetSoundTimer(vx) => {
            // println!("Executing instruction: set sound timer ({reg:?})");
            set_sound_timer(get_register(vx));
        }
        // FX1E
        Instruction::AddToIndex(vx) => {
            set_i(get_i() + get_register(vx) as u16);
        }
        // FX0A
        Instruction::GetKey(vx) => {
            if let Some(key) = last_pressed_keys.iter().find(|k| !pressed_keys.contains(k)) {
                set_register(vx, *KEYPRESS_MAP.get().unwrap().get(key).unwrap());
            } else {
                set_pc(get_pc() - 2);
            }
        }
        // FX29
        Instruction::FontCharacter(vx) => {
            let char = get_register(vx);
            set_memory_u16(get_i(), 0x50 + (char as u16) * 5);
            // set_pc(0x50 + (char as u16) * 5);
        }
        // FX33
        Instruction::BCD(vx) => {
            let val = get_register(vx);
            let hundreds = val / 100;
            let tens = (val % 100) / 10;
            let ones = val % 10;

            set_memory_u8(get_i(), hundreds);
            set_memory_u8(get_i() + 1, tens);
            set_memory_u8(get_i() + 2, ones);
        }
        // FX55
        Instruction::StoreMemory(vx) => {
            for i in 0..=vx {
                set_memory_u8(get_i(), get_register(i.into()));
                set_i(get_i() + 1);
            }
        }
        // FX65
        Instruction::LoadMemory(vx) => {
            for i in 0..=vx {
                set_register(i.into(), get_memory_u8(get_i()));
                set_i(get_i() + 1);
            }
        }
    }
}
