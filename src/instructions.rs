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
                format!("SetRegister({vx}, {:#04X})", nn)
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
            Self::SetIndexRegister(nnn) => format!("SetI({:#06X})", nnn),
            Self::JumpOffset(nnn) => format!("JumpOffset({:#06X})", nnn),
            Self::Random(vx, nn) => format!("Random({vx}, {:#04X})", nn),
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
        }
    }
}

// impl Debug for Instruction {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match *self {
//             Self::ExecuteMachineLanguageRoutine => {
//                 f.write_str("ExecuteMachineLanguageRoutine (Invalid)")
//             }
//             Self::Clear => f.write_str("Clear"),
//             Self::SubroutineReturn => f.write_str("SubroutineReturn"),
//             Self::Jump(nnn) => {
//                 let instruction_raw = get_memory_u16(nnn);
//                 let instruction = decode(instruction_raw);
//                 if let Some(ins) = instruction {
//                     f.write_str(format!("Jump({:#06X}) -> {:?}", nnn, ins).as_str())
//                 } else {
//                     f.write_str(format!("Jump({:#06X}) -> (invalid)", nnn,).as_str())
//                 }
//             }
//             Self::SubroutineCall(nnn) => {
//                 f.write_str(format!("SubroutineCall({:#06X})", nnn).as_str())
//             }
//             Self::SkipConditional1(vx, nn) => f.write_str(
//                 format!("SkipEqual({vx} -> {:#04X}, {:#04X})", get_register(vx), nn).as_str(),
//             ),
//             Self::SkipConditional2(vx, nn) => f.write_str(
//                 format!(
//                     "SkipNotEqual({vx} -> {:#04X}, {:#04X})",
//                     get_register(vx),
//                     nn
//                 )
//                 .as_str(),
//             ),
//             Self::SkipConditional3(vx, vy) => f.write_str(
//                 format!(
//                     "SkipEqual({vx} -> {:#04X}, {vy} -> {:#04X})",
//                     get_register(vx),
//                     get_register(vy)
//                 )
//                 .as_str(),
//             ),
//             Self::SetRegister(vx, nn) => {
//                 f.write_str(format!("SetRegister({vx}, {:#04X})", nn).as_str())
//             }
//             Self::Add(vx, nn) => f.write_str(
//                 format!(
//                     "SetRegister({vx} -> {:#04X}, {:#04X})",
//                     get_register(vx),
//                     nn
//                 )
//                 .as_str(),
//             ),
//             Self::RegSet(vx, vy) => f.write_str(
//                 format!("SetRegister({vx}, {vy} -> {:#04X})", get_register(vy)).as_str(),
//             ),
//             Self::BinaryOr(vx, vy) => f.write_str(
//                 format!(
//                     "BinaryOr({vx} -> {:#04X}, {vy} -> {:#04X})",
//                     get_register(vx),
//                     get_register(vy)
//                 )
//                 .as_str(),
//             ),
//             Self::BinaryAnd(vx, vy) => f.write_str(
//                 format!(
//                     "BinaryAnd({vx} -> {:#04X}, {vy} -> {:#04X})",
//                     get_register(vx),
//                     get_register(vy)
//                 )
//                 .as_str(),
//             ),
//             Self::BinaryXor(vx, vy) => f.write_str(
//                 format!(
//                     "BinaryXor({vx} -> {:#04X}, {vy} -> {:#04X})",
//                     get_register(vx),
//                     get_register(vy)
//                 )
//                 .as_str(),
//             ),
//             Self::RegAdd(vx, vy) => f.write_str(
//                 format!(
//                     "Add({vx} -> {:#04X}, {vy} -> {:#04X})",
//                     get_register(vx),
//                     get_register(vy)
//                 )
//                 .as_str(),
//             ),
//             Self::Subtract1(vx, vy) => f.write_str(
//                 format!(
//                     "Subtract({vx} -> {:#04X}, {vy} -> {:#04X}) ({vx} - {vy})",
//                     get_register(vx),
//                     get_register(vy)
//                 )
//                 .as_str(),
//             ),
//             Self::ShiftRight(vx, vy) => f.write_str(
//                 format!(
//                     "ShiftRight({vx} -> {:#04X}, {vy} -> {:#04X})",
//                     get_register(vx),
//                     get_register(vy)
//                 )
//                 .as_str(),
//             ),
//             Self::Subtract2(vx, vy) => f.write_str(
//                 format!(
//                     "Subtract({vx} -> {:#04X}, {vy} -> {:#04X}) ({vy} - {vx})",
//                     get_register(vx),
//                     get_register(vy)
//                 )
//                 .as_str(),
//             ),
//             Self::ShiftLeft(vx, vy) => f.write_str(
//                 format!(
//                     "ShiftLeft({vx} -> {:#04X}, {vy} -> {:#04X})",
//                     get_register(vx),
//                     get_register(vy)
//                 )
//                 .as_str(),
//             ),
//             Self::SkipConditional4(vx, vy) => f.write_str(
//                 format!(
//                     "SkipNotEqual({vx} -> {:#04X}, {vy} -> {:#04X})",
//                     get_register(vx),
//                     get_register(vy)
//                 )
//                 .as_str(),
//             ),
//             Self::SetIndexRegister(nnn) => f.write_str(format!("SetI({:#06X})", nnn).as_str()),
//             Self::JumpOffset(nnn) => f.write_str(format!("JumpOffset({:#06X})", nnn).as_str()),
//             Self::Random(vx, nn) => f.write_str(format!("Random({vx}, {:#04X})", nn).as_str()),
//             Self::Draw(vx, vy, n) => f.write_str(
//                 format!(
//                     "Draw({vx} -> {:#04X}, {vy} -> {:#04X}, {:#04X})",
//                     get_register(vx),
//                     get_register(vy),
//                     n
//                 )
//                 .as_str(),
//             ),
//             Self::SkipIfKey(vx) => f.write_str(
//                 format!(
//                     "SkipIfKey({vx} -> {:#04X} ({:?}))",
//                     get_register(vx),
//                     REVERSE_KEYPRESS_MAP
//                         .get()
//                         .unwrap()
//                         .get(&get_register(vx))
//                         .unwrap()
//                 )
//                 .as_str(),
//             ),
//             Self::SkipIfNotKey(vx) => f.write_str(
//                 format!(
//                     "SkipIfNotKey({vx} -> {:#04X} ({:?}))",
//                     get_register(vx),
//                     REVERSE_KEYPRESS_MAP
//                         .get()
//                         .unwrap()
//                         .get(&get_register(vx))
//                         .unwrap()
//                 )
//                 .as_str(),
//             ),
//             Self::GetDelayTimer(vx) => f.write_str(format!("GetDelayTimer({vx})").as_str()),
//             Self::GetKey(vx) => f.write_str(format!("GetKey({vx})").as_str()),
//             Self::SetDelayTimer(vx) => {
//                 f.write_str(format!("SetDelayTimer({vx} -> {:#04X})", get_register(vx)).as_str())
//             }
//             Self::SetSoundTimer(vx) => {
//                 f.write_str(format!("SetSoundTimer({vx} -> {:#04X})", get_register(vx)).as_str())
//             }
//             Self::AddToIndex(vx) => {
//                 f.write_str(format!("AddToI({vx} -> {:#04X})", get_register(vx)).as_str())
//             }
//             Self::FontCharacter(vx) => {
//                 f.write_str(format!("FontAddress({vx} -> {:#04X})", get_register(vx)).as_str())
//             }
//             Self::BCD(vx) => f.write_str(
//                 format!("BinaryCodedDecimal({vx} -> {:#04X})", get_register(vx)).as_str(),
//             ),
//             Self::StoreMemory(n) => f.write_str(format!("StoreMemory({n})").as_str()),
//             Self::LoadMemory(n) => f.write_str(format!("LoadMemory({n})").as_str()),
//         }
//     }
// }
