use crate::instructions::Instruction;

pub fn decode(ins: u16) -> Option<Instruction> {
    let first = ((ins & 0xF000) >> 12) as u8;
    let second = ((ins & 0x0F00) >> 8) as u8;
    let third = ((ins & 0x00F0) >> 4) as u8;
    let fourth = (ins & 0x000F) as u8;

    match first {
        0x0 => match second {
            0x0 => match third {
                0xE => match fourth {
                    0xE => Some(Instruction::SubroutineReturn),
                    0x0 => Some(Instruction::Clear),
                    _ => None,
                },
                _ => None,
            },
            _ => None,
        },
        0x1 => Some(Instruction::Jump(ins & 0x0FFF)),
        0x2 => Some(Instruction::SubroutineCall(ins & 0x0FFF)),
        0x3 => Some(Instruction::SkipConditional1(
            second.into(),
            (ins & 0x00FF) as u8,
        )),
        0x4 => Some(Instruction::SkipConditional2(
            second.into(),
            (ins & 0x00FF) as u8,
        )),
        0x5 => match fourth {
            0 => Some(Instruction::SkipConditional3(second.into(), third.into())),
            _ => None,
        },
        0x6 => Some(Instruction::SetRegister(second.into(), (ins & 0xff) as u8)),
        0x7 => Some(Instruction::Add(second.into(), (ins & 0x00FF) as u8)),
        0x8 => match fourth {
            0 => Some(Instruction::RegSet(second.into(), third.into())),
            1 => Some(Instruction::BinaryOr(second.into(), third.into())),
            2 => Some(Instruction::BinaryAnd(second.into(), third.into())),
            3 => Some(Instruction::BinaryXor(second.into(), third.into())),
            4 => Some(Instruction::RegAdd(second.into(), third.into())),
            5 => Some(Instruction::Subtract1(second.into(), third.into())),
            6 => Some(Instruction::ShiftRight(second.into(), third.into())),
            7 => Some(Instruction::Subtract2(second.into(), third.into())),
            0xE => Some(Instruction::ShiftLeft(second.into(), third.into())),
            _ => None,
        },
        0x9 => match fourth {
            0 => Some(Instruction::SkipConditional4(second.into(), third.into())),
            _ => None,
        },
        0xA => Some(Instruction::SetIndexRegister(ins & 0xFFF)),
        0xB => Some(Instruction::JumpOffset(ins & 0xFFF)),
        0xC => Some(Instruction::Random(second.into(), (ins & 0x00FF) as u8)),
        0xD => Some(Instruction::Draw(second.into(), third.into(), fourth)),
        0xE => match ins & 0x00FF {
            0x9E => Some(Instruction::SkipIfKey(second.into())),
            0xA1 => Some(Instruction::SkipIfNotKey(second.into())),
            _ => None,
        },
        0xF => match ins & 0x00FF {
            0x07 => Some(Instruction::GetDelayTimer(second.into())),
            0x0A => Some(Instruction::GetKey(second.into())),
            0x15 => Some(Instruction::SetDelayTimer(second.into())),
            0x18 => Some(Instruction::SetSoundTimer(second.into())),
            0x1E => Some(Instruction::AddToIndex(second.into())),
            0x29 => Some(Instruction::FontCharacter(second.into())),
            0x33 => Some(Instruction::BCD(second.into())),
            0x55 => Some(Instruction::StoreMemory(second)),
            0x65 => Some(Instruction::LoadMemory(second)),
            _ => None,
        },
        _ => None,
    }
}
