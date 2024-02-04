#[cfg(test)]
mod symbol_table_tests {
    use crate::parser::instruction::Instruction::{AInstrSymbol, LInstr};
    use crate::SymbolTable;

    #[test]
    fn test_symbol_table() {
        let mut symbol_table = SymbolTable::new();
        let var1 = AInstrSymbol(String::from("VAR1"));
        let var2 = AInstrSymbol(String::from("FORWARD_REF"));
        let label1 = LInstr(String::from("LABEL1"), 0);
        let label2 = LInstr(String::from("LABEL2"), 1);
        let label3 = LInstr(String::from("FORWARD_REF"), 10);
        let var3 = AInstrSymbol(String::from("VAR3"));
        let var4 = AInstrSymbol(String::from("APPLE"));

        assert_eq!(Ok(()), symbol_table.add_symbol(&label1));
        assert_eq!(Ok(()), symbol_table.add_symbol(&label2));
        assert_eq!(Err(()), symbol_table.add_symbol(&label2));
        assert_eq!(Ok(()), symbol_table.add_symbol(&var1));
        assert_eq!(Ok(()), symbol_table.add_symbol(&var2));
        assert_eq!(Ok(()), symbol_table.add_symbol(&var1));
        assert_eq!(Ok(()), symbol_table.add_symbol(&var3));
        assert_eq!(Ok(()), symbol_table.add_symbol(&label3));
        assert_eq!(Ok(()), symbol_table.add_symbol(&var4));

        symbol_table.resolve_addresses();
        assert_eq!(0, symbol_table.get_symbol_address("LABEL1"));
        assert_eq!(0, symbol_table.get_symbol_address("LABEL2"));
        assert_eq!(8, symbol_table.get_symbol_address("FORWARD_REF"));
        assert_eq!(16, symbol_table.get_symbol_address("VAR1"));
        assert_eq!(17, symbol_table.get_symbol_address("VAR3"));
        assert_eq!(18, symbol_table.get_symbol_address("APPLE"));
    }
}