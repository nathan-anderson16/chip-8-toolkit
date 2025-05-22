use c8util::{decode::decode, instructions::Instruction};

use crate::{
    run::REVERSE_KEYPRESS_MAP,
    system::{get_memory_u16, get_register},
};

/// Fancy formatting of instructions (register values, jump predictions, etc)
pub trait FancyInstruction {
    fn fancy_fmt(&self) -> String;
}

impl FancyInstruction for Instruction {
    #[allow(clippy::too_many_lines)]
    fn fancy_fmt(&self) -> String {
        match *self {
            Self::ExecuteMachineLanguageRoutine => {
                String::from("ExecuteMachineLanguageRoutine (Invalid)")
            }
            Self::Clear => String::from("Clear"),
            Self::SubroutineReturn => String::from("SubroutineReturn"),
            Self::Jump(nnn) => {
                let instruction_raw = get_memory_u16(nnn);
                let instruction = decode(instruction_raw);
                if let Some(ins) = instruction {
                    format!("Jump({nnn:#06X}) -> {}", ins.fancy_fmt())
                } else {
                    format!("Jump({nnn:#06X}) -> (invalid)")
                }
            }
            Self::SubroutineCall(nnn) => {
                format!("SubroutineCall({nnn:#06X})")
            }
            Self::SkipConditional1(vx, nn) => {
                format!("SkipEqual({vx} -> {:#04X}, {:#04X})", get_register(vx), nn)
            }
            Self::SkipConditional2(vx, nn) => String::from(
                format!(
                    "SkipNotEqual({vx} -> {:#04X}, {:#04X})",
                    get_register(vx),
                    nn
                )
                .as_str(),
            ),
            Self::SkipConditional3(vx, vy) => format!(
                "SkipEqual({vx} -> {:#04X}, {vy} -> {:#04X})",
                get_register(vx),
                get_register(vy)
            ),

            Self::SetRegister(vx, nn) => {
                format!("SetRegister({vx}, {nn:#04X})")
            }
            Self::Add(vx, nn) => format!(
                "SetRegister({vx} -> {:#04X}, {:#04X})",
                get_register(vx),
                nn
            ),
            Self::RegSet(vx, vy) => format!("SetRegister({vx}, {vy} -> {:#04X})", get_register(vy)),
            Self::BinaryOr(vx, vy) => format!(
                "BinaryOr({vx} -> {:#04X}, {vy} -> {:#04X})",
                get_register(vx),
                get_register(vy)
            ),
            Self::BinaryAnd(vx, vy) => format!(
                "BinaryAnd({vx} -> {:#04X}, {vy} -> {:#04X})",
                get_register(vx),
                get_register(vy)
            ),
            Self::BinaryXor(vx, vy) => format!(
                "BinaryXor({vx} -> {:#04X}, {vy} -> {:#04X})",
                get_register(vx),
                get_register(vy)
            ),
            Self::RegAdd(vx, vy) => format!(
                "Add({vx} -> {:#04X}, {vy} -> {:#04X})",
                get_register(vx),
                get_register(vy)
            ),
            Self::Subtract1(vx, vy) => format!(
                "Subtract({vx} -> {:#04X}, {vy} -> {:#04X}) ({vx} - {vy})",
                get_register(vx),
                get_register(vy)
            ),
            Self::ShiftRight(vx, vy) => format!(
                "ShiftRight({vx} -> {:#04X}, {vy} -> {:#04X})",
                get_register(vx),
                get_register(vy)
            ),
            Self::Subtract2(vx, vy) => format!(
                "Subtract({vx} -> {:#04X}, {vy} -> {:#04X}) ({vy} - {vx})",
                get_register(vx),
                get_register(vy)
            ),
            Self::ShiftLeft(vx, vy) => format!(
                "ShiftLeft({vx} -> {:#04X}, {vy} -> {:#04X})",
                get_register(vx),
                get_register(vy)
            ),
            Self::SkipConditional4(vx, vy) => format!(
                "SkipNotEqual({vx} -> {:#04X}, {vy} -> {:#04X})",
                get_register(vx),
                get_register(vy)
            ),
            Self::SetIndexRegister(nnn) => format!("SetI({nnn:#06X})"),
            Self::JumpOffset(nnn) => format!("JumpOffset({nnn:#06X})"),
            Self::Random(vx, nn) => format!("Random({vx}, {nn:#04X})"),
            Self::Draw(vx, vy, n) => format!(
                "Draw({vx} -> {:#04X}, {vy} -> {:#04X}, {:#04X})",
                get_register(vx),
                get_register(vy),
                n
            ),
            Self::SkipIfKey(vx) => format!(
                "SkipIfKey({vx} -> {:#04X} ({:?}))",
                get_register(vx),
                REVERSE_KEYPRESS_MAP
                    .get()
                    .unwrap()
                    .get(&get_register(vx))
                    .unwrap()
            ),
            Self::SkipIfNotKey(vx) => format!(
                "SkipIfNotKey({vx} -> {:#04X} ({:?}))",
                get_register(vx),
                REVERSE_KEYPRESS_MAP
                    .get()
                    .unwrap()
                    .get(&get_register(vx))
                    .unwrap()
            ),
            Self::GetDelayTimer(vx) => format!("GetDelayTimer({vx})"),
            Self::GetKey(vx) => format!("GetKey({vx})"),
            Self::SetDelayTimer(vx) => {
                format!("SetDelayTimer({vx} -> {:#04X})", get_register(vx))
            }
            Self::SetSoundTimer(vx) => {
                format!("SetSoundTimer({vx} -> {:#04X})", get_register(vx))
            }
            Self::AddToIndex(vx) => {
                format!("AddToI({vx} -> {:#04X})", get_register(vx))
            }
            Self::FontCharacter(vx) => {
                format!("FontAddress({vx} -> {:#04X})", get_register(vx))
            }
            Self::BCD(vx) => format!("BinaryCodedDecimal({vx} -> {:#04X})", get_register(vx)),
            Self::StoreMemory(n) => format!("StoreMemory({n})"),
            Self::LoadMemory(n) => format!("LoadMemory({n})"),
            Self::Db(nnnn) => format!("db {nnnn}"),
        }
    }
}
