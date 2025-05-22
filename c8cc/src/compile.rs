use c8util::{instructions::Instruction, register::Register};

use crate::node::ProgramNode;

/// Convert a program into instructions.
pub fn compile(program: &ProgramNode) -> Vec<Instruction> {
    let ret_val =
        u8::try_from(program.func.statement.expr.value).expect("return value must be < 16");
    vec![
        // mov $v0, ret_val
        Instruction::SetRegister(Register::V0, ret_val),
        // font $v0
        Instruction::FontCharacter(Register::V0),
        // mov $v0, 0x0
        Instruction::SetRegister(Register::V0, 0x0),
        // mov $v1, 0x0
        Instruction::SetRegister(Register::V1, 0x0),
        // draw $v0, $v1, 0xF
        Instruction::Draw(Register::V0, Register::V1, 0xF),
        Instruction::Jump(0x20A),
    ]
}
