#[cfg(test)]
mod parser_tests {
    use crate::{Codes, Parser};
    use crate::parser::instruction::Instruction;
    use crate::parser::parser::{is_a_instr, is_l_instr, is_valid_label};

    #[test]
    fn is_a_instr_true() {
        assert!(is_a_instr("@17"));
        assert!(is_a_instr("@some_label"));
    }

    #[test]
    fn is_a_instr_false() {
        assert!(!is_a_instr("(LABEL)"));
        assert!(!is_a_instr("D=M-1"));
        assert!(!is_a_instr("D=M-1;JEQ"));
        assert!(!is_a_instr("M-1;JEQ"));
        assert!(!is_a_instr("M-1"));
    }

    #[test]
    fn is_l_instr_true() {
        assert!(is_l_instr("(LABEL)"));
    }

    #[test]
    fn is_l_instr_false() {
        assert!(!is_l_instr("@some_label"));
        assert!(!is_l_instr("(LABEL"));
        assert!(!is_l_instr("LABEL)"));
        assert!(!is_l_instr("D=M-1"));
        assert!(!is_l_instr("D=M-1;JEQ"));
        assert!(!is_l_instr("M-1;JEQ"));
        assert!(!is_l_instr("M-1"));
    }

    #[test]
    fn is_valid_label_true() {
        assert!(is_valid_label("La_b11$e:l."));
        assert!(is_valid_label("_b1La1$e:l."));
        assert!(is_valid_label(".b1La1$e:l_"));
        assert!(is_valid_label("$b1La1.e:l_"));
    }

    #[test]
    fn is_valid_label_false() {
        assert!(!is_valid_label("@some_label"));
        assert!(!is_valid_label("1some_label"));
        assert!(!is_valid_label("so,me_label"));
        assert!(!is_valid_label("so-me_label"));
        assert!(!is_valid_label("so=me_label"));
        assert!(!is_valid_label("so+me_label"));
        assert!(!is_valid_label("so<me_label"));
    }

    #[test]
    fn parse_a_instr_decimal() {
        let codes = Codes::new();
        let mut parser = Parser::new(&codes);
        let instruction = parser.parse_a_instr(44, String::from("@3245"));

        assert_eq!(Instruction::AInstrDecimal(3245), instruction);
        assert!(!parser.failed())
    }

    #[test]
    fn parse_a_instr_decimal_invalid() {
        let codes = Codes::new();
        let mut parser = Parser::new(&codes);
        let instruction = parser.parse_a_instr(44, String::from("@-1"));

        assert_eq!(Instruction::Error, instruction);
        assert!(parser.failed())
    }

    #[test]
    fn parse_a_instr_symbol() {
        let codes = Codes::new();
        let mut parser = Parser::new(&codes);
        let instruction = parser.parse_a_instr(44, String::from("@_b1La1$e:l."));

        assert_eq!(Instruction::AInstrSymbol(String::from("_b1La1$e:l.")), instruction);
        assert!(!parser.failed())
    }

    #[test]
    fn parse_a_instr_symbol_invalid() {
        let codes = Codes::new();
        let mut parser = Parser::new(&codes);
        let instruction = parser.parse_a_instr(44, String::from("@7_b1La1$e:l."));

        assert_eq!(Instruction::Error, instruction);
        assert!(parser.failed())
    }

    #[test]
    fn parse_l_instr_valid() {
        let codes = Codes::new();
        let mut parser = Parser::new(&codes);
        let instruction = parser.parse_l_instr(32, String::from("(SOME_LABEL)"));

        assert_eq!(Instruction::LInstr(String::from("SOME_LABEL"), 32), instruction);
        assert!(!parser.failed())
    }

    #[test]
    fn parse_l_instr_invalid() {
        let codes = Codes::new();
        let mut parser = Parser::new(&codes);
        let instruction = parser.parse_l_instr(32, String::from("(;SOME_LABEL)"));

        assert_eq!(Instruction::Error, instruction);
        assert!(parser.failed())
    }

    #[test]
    fn parse_c_instr_valid_comp_only() {
        let codes = Codes::new();
        let mut parser = Parser::new(&codes);
        let instruction = parser.parse_c_instr(32, String::from("M-1"));

        assert_eq!(
            Instruction::CInstr {
                dest: String::from(""),
                comp: String::from("M-1"),
                jump: String::from(""),
            },
            instruction
        );
        assert!(!parser.failed())
    }

    #[test]
    fn parse_c_instr_valid_comp_and_dest() {
        let codes = Codes::new();
        let mut parser = Parser::new(&codes);
        let instruction = parser.parse_c_instr(32, String::from("A=M-1"));

        assert_eq!(
            Instruction::CInstr {
                dest: String::from("A"),
                comp: String::from("M-1"),
                jump: String::from(""),
            },
            instruction
        );
        assert!(!parser.failed())
    }

    #[test]
    fn parse_c_instr_valid_comp_and_jump() {
        let codes = Codes::new();
        let mut parser = Parser::new(&codes);
        let instruction = parser.parse_c_instr(32, String::from("M-1;JEQ"));

        assert_eq!(
            Instruction::CInstr {
                dest: String::from(""),
                comp: String::from("M-1"),
                jump: String::from("JEQ"),
            },
            instruction
        );
        assert!(!parser.failed())
    }

    #[test]
    fn parse_c_instr_valid_comp_and_jump_and_dest() {
        let codes = Codes::new();
        let mut parser = Parser::new(&codes);
        let instruction = parser.parse_c_instr(32, String::from("MD=M-1;JEQ"));

        assert_eq!(
            Instruction::CInstr {
                dest: String::from("MD"),
                comp: String::from("M-1"),
                jump: String::from("JEQ"),
            },
            instruction
        );
        assert!(!parser.failed())
    }
}
