use c8util::{instructions::Instruction, register::Register};

use crate::node::{Expr, ProgramNode};

/// Convert a program into instructions.
pub fn compile(program: &ProgramNode) -> Vec<Instruction> {
    let mut instructions = Vec::new();

    match program.func.statement.expr.value {
        Expr::Constant(ret_val) => {
            instructions = vec![
                // mov $v0, ret_val
                Instruction::SetRegister(
                    Register::V0,
                    u8::try_from(ret_val).expect("value must be < 16"),
                ),
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
        Expr::Unary(op, node) => {}
    }

    instructions
    // let ret_val =
    //     u8::try_from(program.func.statement.expr.value).expect("return value must be < 16");
    // vec![
    //     // mov $v0, ret_val
    //     Instruction::SetRegister(Register::V0, ret_val),
    //     // font $v0
    //     Instruction::FontCharacter(Register::V0),
    //     // mov $v0, 0x0
    //     Instruction::SetRegister(Register::V0, 0x0),
    //     // mov $v1, 0x0
    //     Instruction::SetRegister(Register::V1, 0x0),
    //     // draw $v0, $v1, 0xF
    //     Instruction::Draw(Register::V0, Register::V1, 0xF),
    //     Instruction::Jump(0x20A),
    // ]
}
