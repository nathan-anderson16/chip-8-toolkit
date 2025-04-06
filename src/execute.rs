use std::{collections::HashSet, time::SystemTime};

use device_query::Keycode;

use crate::{
    instructions::Instruction,
    run::{KEYPRESS_MAP, REVERSE_KEYPRESS_MAP},
    system::{
        DISPLAY_HEIGHT, DISPLAY_WIDTH, Register, get_delay_timer, get_display, get_i,
        get_memory_u8, get_pc, get_register, set_delay_timer, set_display, set_i, set_memory_u8,
        set_memory_u16, set_pc, set_register, set_sound_timer, stack_pop, stack_push,
    },
};

pub fn execute(
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
                set_pc(get_pc().saturating_sub(2));
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
                set_pc(get_pc().saturating_sub(2));
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
