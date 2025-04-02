use crate::system::Register;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    /// 00E0. Clear the screen.
    Clear,
    /// 1NNN. Jump to the given address.
    Jump(u16),
    /// 6XNN. Set the register to the given value.
    SetRegister(Register, u8),
    /// 7XNN. Add the given value to the register. Does NOT set the carry bit of an overflow occurs.
    Add(Register, u8),
    /// ANNN. Set the index register to the given value.
    SetIndexRegister(u16),
    /// DXYN. Draw a N pixel tall sprite from the memory location pointed to by I, with x-coord VX and y-coord VY (the coordinates will wrap).
    /// All pixels "on" in the sprite will flip the pixels it is drawn to (0 -> 1, 1 -> 0).
    /// - If any pixel is turned off by this, VF is set to 1. Otherwise, it's set to 0.
    /// All pixels "off" in the sprite are treated as transparent.
    /// The drawing of the sprite should not wrap.
    Draw(Register, Register, u8),
}
