use std::fmt::Debug;

use crate::register::Register;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    /// 0NNN. Pause execution of the program and call a subroutine written in machine language instead.
    /// NOT TO BE USED.
    ExecuteMachineLanguageRoutine,
    /// 00E0. Clear the screen.
    Clear,
    /// 00EE. Return from a subroutine.
    SubroutineReturn,
    /// 1NNN. Jump to the given address.
    Jump(u16),
    /// 2NNN. Jump to the subroutine at the given address, pushing the current PC to the stack.
    SubroutineCall(u16),
    /// 3XNN. Increase PC by 2 if the value in the given register is equal to NN.
    SkipConditional1(Register, u8),
    /// 4XNN. Increase PC by 2 if the value in the given register is not equal to NN.
    SkipConditional2(Register, u8),
    /// 5XNN. Increase PC by 2 if the values in the registers are equal.
    SkipConditional3(Register, Register),
    /// 6XNN. Set the register to the given value.
    SetRegister(Register, u8),
    /// 7XNN. Add the given value to the register. Does NOT set the carry bit of an overflow occurs.
    Add(Register, u8),
    /// 8XY0. Set VX to the value of VY.
    RegSet(Register, Register),
    /// 8XY1. Set VX to VX | VY.
    BinaryOr(Register, Register),
    /// 8XY2. Set VX to VX & VY.
    BinaryAnd(Register, Register),
    /// 8XY3. Set VX to VX ^ VY.
    BinaryXor(Register, Register),
    /// 8XY4. Set VX to VX + VY. If VX overflows, set VF to 1. Otherwise, set VF to 0.
    RegAdd(Register, Register),
    /// 8XY5. Set VX to VX - VY. If the operation underflows, set VF to 0. Otherwise, set VF to 1.
    Subtract1(Register, Register),
    /// 8XY6. Set VX to VY. Shift the value of VX right one bit. If the bit that was shifted out was 1, set VF to 1. Otherwise, set VF to 0. TODO: Add an option to disable the "Set VX to VY" behavior.
    ShiftRight(Register, Register),
    /// 8XY7. Set VX to VY - VX. If the operation underflows, set VF to 0. Otherwise, set VF to 1.
    Subtract2(Register, Register),
    /// 8XYE. Set VX to VY. Shift the value of VX left one bit. If the bit that was shifted out was 1, set VF to 1. Otherwise, set VF to 0. TODO: Add an option to disable the "Set VX to VY" behavior.
    ShiftLeft(Register, Register),
    /// 9XY0. Increase PC by 2 if the values in the registers are not equal.
    SkipConditional4(Register, Register),
    /// ANNN. Set the index register to the given value.
    SetIndexRegister(u16),
    /// BNNN. Jump to the address NNN + V0. TODO: Add compatibility option to jump to act as BXNN: jump to address XNN + the value of VX.
    JumpOffset(u16),
    /// CXNN. Generate a random number, AND it with NN, and put the result in VX.
    Random(Register, u8),
    /// DXYN. Draw a N pixel tall sprite from the memory location pointed to by I, with x-coord VX and y-coord VY (the coordinates will wrap).
    ///   All pixels "on" in the sprite will flip the pixels it is drawn to (0 -> 1, 1 -> 0).
    ///   - If any pixel is turned off by this, VF is set to 1. Otherwise, it's set to 0.
    ///
    /// All pixels "off" in the sprite are treated as transparent.
    ///
    /// The drawing of the sprite should not wrap.
    Draw(Register, Register, u8),
    /// EX9E. Increment PC by 2 if the key corresponding to the value in VX is pressed.
    SkipIfKey(Register),
    /// EXA1. Increment PC by 2 if the key corresponding to the value in VX is not pressed.
    SkipIfNotKey(Register),
    /// FX07. Set VX to the current value of the delay timer.
    GetDelayTimer(Register),
    /// FX0A. Block until a key is pressed, then put that key into VX.
    GetKey(Register),
    /// FX15. Set the delay timer to the value in VX.
    SetDelayTimer(Register),
    /// FX18. Set the sound timer to the value in VX.
    SetSoundTimer(Register),
    /// FX1E. Add the value of VX to I.
    AddToIndex(Register),
    /// FX29. Set I to the address of the hexadecimal character in VX.
    FontCharacter(Register),
    /// FX33. Convert the binary number in VX to three decimal digits, then store those digits in memory at the address pointed to by I.
    BCD(Register),
    /// FX55. Store the values of each register from V0 to VX, inclusive, in successive memory addresses, starting at I. TODO: Add a compatibility option to increment I each time a register is stored.
    StoreMemory(u8),
    /// FX65. Load the values of each register from V0 to VX, inclusive, at successive memory addresses, starting at I. TODO: Add a compatibility option to increment I each time a register is loaded.
    LoadMemory(u8),
}
