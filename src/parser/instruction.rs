#[derive(Debug)]
pub enum Instruction {
    AInstrDecimal(u32),
    AInstrSymbol(String),
    LInstr(String, usize),
    CInstr { dest: Option<String>, comp: String, jump: Option<String> },
    Error
}
