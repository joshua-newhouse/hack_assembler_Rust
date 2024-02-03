use crate::parser::instruction::Instruction;

#[derive(Debug)]
pub struct Encoder {}

impl Encoder {
    pub fn new() -> Encoder {
        Encoder {}
    }

    pub fn to_binary(&self, instruction: Instruction) -> String {
        format!("{:?}", instruction)
    }
}
