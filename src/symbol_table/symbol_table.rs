use crate::parser::instruction::Instruction;

#[derive(Debug)]
pub struct SymbolTable {}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable {}
    }

    pub fn add_symbol(&mut self, instruction: &Instruction) {
        log::debug!("adding symbol {:?} to table", instruction);
    }

    pub fn resolve_addresses(&mut self) {
        log::debug!("resolving variable addresses");
    }
}
