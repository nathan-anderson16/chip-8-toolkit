use std::{env, fs::File, io::Read};

use c8util::{decode::decode, instructions::Instruction};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <path>", args[0]);
        return;
    }

    let mut buf = Vec::new();
    let mut f = File::open(&args[1]).expect("failed to open file");
    f.read_to_end(&mut buf).expect("failed to read file");
    disassemble(&buf);
}

fn disassemble(v: &[u8]) {
    v.iter()
        .enumerate()
        .step_by(2)
        .map(|(i, &val)| (u16::from(val) << 8) | u16::from(*v.get(i + 1).unwrap()))
        .for_each(|code| {
            let ins = decode(code);
            if let Some(i) = ins {
                println!("{}", get_instruction(i));
            } else {
                println!("db    {code:#06X}");
            };
        });
}

fn get_instruction(ins: Instruction) -> String {
    match ins {
        Instruction::ExecuteMachineLanguageRoutine => String::from("(bad)"),
        Instruction::Clear => String::from("clear"),
        Instruction::SubroutineReturn => String::from("ret"),
        Instruction::Jump(nnn) => format!("jmp   {nnn:#06X}"),
        Instruction::SubroutineCall(nnn) => format!("call  {nnn:#06X}"),
        Instruction::SkipConditional1(vx, nn) => format!("ske   ${vx}, {nn:#02X}"),
        Instruction::SkipConditional2(vx, nn) => format!("skn   ${vx}, {nn:#02X}"),
        Instruction::SkipConditional3(vx, vy) => format!("ske   ${vx}, ${vy}"),
        Instruction::SetRegister(vx, nn) => format!("mov   ${vx}, {nn:#02X}"),
        Instruction::Add(vx, nn) => format!("add   ${vx}, {nn:#02X}"),
        Instruction::RegSet(vx, vy) => format!("mov   ${vx}, ${vy}"),
        Instruction::BinaryOr(vx, vy) => format!("or    ${vx}, ${vy}"),
        Instruction::BinaryAnd(vx, vy) => format!("and   ${vx}, ${vy}"),
        Instruction::BinaryXor(vx, vy) => format!("xor   ${vx}, ${vy}"),
        Instruction::RegAdd(vx, vy) => format!("add   ${vx}, ${vy}"),
        Instruction::Subtract1(vx, vy) => format!("sub1  ${vx}, ${vy}"),
        Instruction::Subtract2(vx, vy) => format!("sub2  ${vx}, ${vy}"),
        Instruction::ShiftRight(vx, vy) => format!("shr   ${vx}, ${vy}"),
        Instruction::ShiftLeft(vx, vy) => format!("shl   ${vx}, ${vy}"),
        Instruction::SkipConditional4(vx, vy) => format!("skn   ${vx}, ${vy}"),
        Instruction::SetIndexRegister(nnn) => format!("mov   $i, {nnn:#06X}"),
        Instruction::JumpOffset(nnn) => format!("jo    {nnn:#06X}"),
        Instruction::Random(vx, nn) => format!("rand  ${vx}, {nn:#04X}"),
        Instruction::Draw(vx, vy, n) => format!("draw  ${vx}, ${vy}, {n:#02X}"),
        Instruction::SkipIfKey(vx) => format!("skk   ${vx}"),
        Instruction::SkipIfNotKey(vx) => format!("sknk  ${vx}"),
        Instruction::GetDelayTimer(vx) => format!("mov   ${vx}, $d"),
        Instruction::GetKey(vx) => format!("key   ${vx}"),
        Instruction::SetDelayTimer(vx) => format!("mov   $d, ${vx}"),
        Instruction::SetSoundTimer(vx) => format!("mov   $s, ${vx}"),
        Instruction::AddToIndex(vx) => format!("add   $i, ${vx}"),
        Instruction::FontCharacter(vx) => format!("font  ${vx}"),
        Instruction::BCD(vx) => format!("bcd   ${vx}"),
        Instruction::StoreMemory(nn) => format!("store {nn:#02X}"),
        Instruction::LoadMemory(nn) => format!("load  {nn:#02X}"),
        Instruction::Db(nnnn) => format!("db    {nnnn:#06X}"),
    }
}
