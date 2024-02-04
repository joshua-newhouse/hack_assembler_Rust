#[derive(Debug, PartialEq)]
pub enum Instruction {
    AInstrDecimal(u32),
    AInstrSymbol(String),
    LInstr(String, usize),
    CInstr { dest: String, comp: String, jump: String },
    Error
}
