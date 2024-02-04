use crate::encoder::codes::Codes;
use crate::parser::instruction::Instruction;

pub struct Encoder<'a> {
    codes: &'a Codes,
}

impl Encoder<'_> {
    pub fn new(codes: &Codes) -> Encoder {
        Encoder { codes }
    }

    pub fn to_binary(&self, instruction: Instruction) -> String {
        String::from("1111000011110000")
    }
}
