use crate::parser::instruction::Instruction;

#[derive(Debug)]
pub struct Parser {
    failed: bool,
}

impl Parser {
    pub fn new() -> Parser {
        Parser { failed: false }
    }

    pub fn generate_instruction(&mut self, line_number: usize, asm_instr: String) -> Instruction {
        log::debug!("generating instruction...");
        Instruction::LInstr(asm_instr, line_number)
    }

    pub fn failed(&self) -> bool {
        self.failed
    }
}
