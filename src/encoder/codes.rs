use std::collections::HashMap;

pub struct Codes {
    pub(crate) dest_codes: HashMap<&'static str, &'static str>,
    pub(crate) jump_codes: HashMap<&'static str, &'static str>,
    pub(crate) comp_codes: HashMap<&'static str, &'static str>,
}

impl Codes {
    pub fn new() -> Codes {
        let dest_codes = get_dest_codes();
        let comp_codes = get_comp_codes();
        let jump_codes = get_jump_codes();

        Codes { dest_codes, jump_codes, comp_codes }
    }

    pub(crate) fn is_valid_destination(&self, dest: &str) -> bool {
        self.dest_codes.contains_key(dest)
    }

    pub(crate) fn is_valid_jump(&self, jump: &str) -> bool {
        self.jump_codes.contains_key(jump)
    }

    pub(crate) fn is_valid_comp(&self, comp: &str) -> bool {
        self.comp_codes.contains_key(comp)
    }
}

fn get_dest_codes() -> HashMap<&'static str, &'static str> {
    HashMap::from(
        [
            ("", "000"),
            ("M", "001"),
            ("D", "010"),
            ("MD", "011"),
            ("A", "100"),
            ("AM", "101"),
            ("AD", "110"),
            ("AMD", "111"),
        ]
    )
}

fn get_jump_codes() -> HashMap<&'static str, &'static str> {
    HashMap::from(
        [
            ("", "000"),
            ("JGT", "001"),
            ("JEQ", "010"),
            ("JGE", "011"),
            ("JLT", "100"),
            ("JNE", "101"),
            ("JLE", "110"),
            ("JMP", "111"),
        ]
    )
}

fn get_comp_codes() -> HashMap<&'static str, &'static str> {
    HashMap::from(
        [
            ("0", "0101010"),
            ("1", "0111111"),
            ("-1", "0111010"),
            ("D", "0001100"),
            ("A", "0110000"),
            ("M", "1110000"),
            ("!D", "0001101"),
            ("!A", "0110001"),
            ("!M", "1110001"),
            ("-D", "0001111"),
            ("-A", "0110011"),
            ("-M", "1110011"),
            ("D+1", "0011111"),
            ("A+1", "0110111"),
            ("M+1", "1110111"),
            ("D-1", "0001110"),
            ("A-1", "0110010"),
            ("M-1", "1110010"),
            ("D+A", "0000010"),
            ("D+M", "1000010"),
            ("D-A", "0010011"),
            ("D-M", "1010011"),
            ("A-D", "0000111"),
            ("M-D", "1000111"),
            ("D&A", "0000000"),
            ("D&M", "1000000"),
            ("D|A", "0010101"),
            ("D|M", "1010101"),
        ]
    )
}
