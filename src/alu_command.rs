use crate::instruction::Instruction;

#[derive(Debug, Clone, Copy)]
pub enum Value {
    Byte(u8),
    Word(u16),
    DWord(u32),
    QWord(u64),
}

#[derive(Debug, Clone, Copy)]
pub struct AluCommand {
    instruction: Instruction,
    src_val: Value,
    dst_val: Value,
}
