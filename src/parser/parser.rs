use crate::encoder::codes::Codes;
use crate::parser::instruction::Instruction;
use crate::parser::instruction::Instruction::{AInstrDecimal, AInstrSymbol, CInstr, Error, LInstr};

pub struct Parser<'a> {
    codes: &'a Codes,
}

impl Parser<'_> {
    pub fn new(codes: &Codes) -> Parser {
        Parser { codes }
    }

    pub fn generate_instruction(&mut self, line_number: usize, asm_instr: String) -> Instruction {
        log::debug!("generating instruction from {asm_instr} at {line_number}...");

        return if is_a_instr(&asm_instr) {
            self.parse_a_instr(line_number, asm_instr)
        } else if is_l_instr(&asm_instr) {
            self.parse_l_instr(line_number, asm_instr)
        } else {
            self.parse_c_instr(line_number, asm_instr)
        }
    }

    pub(super) fn parse_a_instr(&mut self, line_number: usize, asm_instr: String) -> Instruction {
        let value = &asm_instr[1..];

        return if let Ok(val) = value.parse::<u32>() {
            AInstrDecimal(val)
        } else if is_valid_label(value) {
            AInstrSymbol(String::from(value))
        } else {
            log::error!("failed to parse A instruction {asm_instr} on line {line_number}");
            Error
        };
    }

    pub(super) fn parse_l_instr(&mut self, line_number: usize, asm_instr: String) -> Instruction {
        let value = &asm_instr[1..asm_instr.len() - 1];

        return if is_valid_label(value) {
            LInstr(String::from(value), line_number)
        } else {
            log::error!("failed to parse L instruction {asm_instr} on line {line_number}");
            Error
        }
    }

    pub(super) fn parse_c_instr(&mut self, line_number: usize, asm_instr: String) -> Instruction {
        let mut cmp: &str = asm_instr.as_str();
        let mut dst: &str = "";
        let mut jmp: &str = "";

        if let Some((d, c)) = cmp.split_once('=') {
            dst = d;
            cmp = c;
        }

        if let Some((c, j)) = cmp.split_once(';') {
            cmp = c;
            jmp = j;
        }

        let dest: String;
        if self.codes.is_valid_destination(dst) {
            dest = String::from(dst);
        } else {
            log::error!("invalid destination {dst} in C instruction on line {line_number}");
            return Error;
        }

        let jump: String;
        if self.codes.is_valid_jump(jmp) {
            jump = String::from(jmp);
        } else {
            log::error!("invalid jump {jmp} in C instruction on line {line_number}");
            return Error;
        }

        let comp: String;
        if self.codes.is_valid_comp(cmp) {
            comp = String::from(cmp);
        } else {
            log::error!("invalid comp {cmp} in C instruction on line {line_number}");
            return Error;
        }

        CInstr { dest, comp, jump }
    }
}

pub(super) fn is_a_instr(instr: &str) -> bool {
    instr.starts_with('@')
}

pub(super) fn is_l_instr(instr: &str) -> bool {
    instr.starts_with('(') && instr.ends_with(')')
}

pub(super) fn is_valid_label(value: &str) -> bool {
    if value.starts_with(|c: char| c.is_numeric()) {
        return false;
    }

    for c in value.chars() {
        if !(
            c.is_alphanumeric() ||
                c.eq(&'_') ||
                c.eq(&'.') ||
                c.eq(&'$') ||
                c.eq(&':')
        ) {
            return false;
        }
    }

    true
}
