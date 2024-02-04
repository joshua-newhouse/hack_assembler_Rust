use crate::encoder::codes::Codes;
use crate::parser::instruction::Instruction;
use crate::SymbolTable;

pub struct Encoder<'a> {
    codes: &'a Codes,
}

impl Encoder<'_> {
    pub fn new(codes: &Codes) -> Encoder {
        Encoder { codes }
    }

    pub fn to_binary(&self, instruction: Instruction, symbol_table: &SymbolTable) -> String {
        match instruction {
            Instruction::AInstrDecimal(val) => {
                let mut binary = String::from("0");
                binary.push_str(format!("{val:015b}\n").as_str());
                binary
            },
            Instruction::AInstrSymbol(s) => {
                let mut binary = String::from("0");
                let symbol_address = symbol_table.get_symbol_address(s.as_str());
                binary.push_str(format!("{symbol_address:015b}\n").as_str());
                binary
            },
            Instruction::CInstr { dest, comp, jump } => {
                let mut binary = String::from("111");
                binary.push_str(self.codes.comp_codes.get(comp.as_str()).unwrap());
                binary.push_str(self.codes.dest_codes.get(dest.as_str()).unwrap());
                binary.push_str(self.codes.jump_codes.get(jump.as_str()).unwrap());
                binary.push('\n');
                binary
            },
            _ => String::from("0000000000000000\n"),
        }
    }
}
