use std::collections::HashMap;

use crate::parser::instruction::Instruction;

pub struct SymbolTable {
    symbol_addresses: HashMap<String, u32>,
    var_addresses_assigned: u32,
    jump_labels_count: usize,
    next_var_address: u32,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable {
            symbol_addresses: initial_symbol_map(),
            var_addresses_assigned: 0,
            jump_labels_count: 0,
            next_var_address: 16,
        }
    }

    pub fn add_symbol(&mut self, instruction: &Instruction) -> Result<(), ()> {
        log::debug!("adding symbol {:?} to table", instruction);

        match instruction {
            Instruction::AInstrSymbol(s) => if !self.symbol_addresses.contains_key(s.as_str()) {
                self.symbol_addresses.insert(String::from(s), u32::MAX - self.var_addresses_assigned);
                self.var_addresses_assigned += 1;
            }
            Instruction::LInstr(s, ln) => {
                if let Some(existing_definition) = self.symbol_addresses.remove(s.as_str()) {
                    if u16::MAX as u32 > existing_definition {
                        log::error!("attempt to redefine label {s} on line {ln} when it already has address {existing_definition}");
                        return Err(());
                    }
                }

                let next_jmp_lbl_addr = ln - self.jump_labels_count;
                self.symbol_addresses.insert(String::from(s), next_jmp_lbl_addr as u32);
                self.jump_labels_count += 1;
            }
            _ => ()
        }

        Ok(())
    }

    pub fn resolve_addresses(&mut self) {
        log::debug!("resolving variable addresses");

        let mut variables: Vec<_> = self.symbol_addresses.iter_mut()
            .filter(|(_, address)| (u16::MAX as u32) < **address)
            .collect();

        variables.sort_by(|(_, addr1), (_, addr2)| addr2.cmp(addr1));

        variables.iter_mut()
            .for_each(|(_, address)| {
                **address = self.next_var_address;
                self.next_var_address += 1;
            });
    }

    pub fn get_symbol_address(&self, symbol: &str) -> u32 {
        match self.symbol_addresses.get(symbol) {
            None => 0,
            Some(addr) => *addr,
        }
    }
}

fn initial_symbol_map() -> HashMap<String, u32> {
    HashMap::from([
        ("R0".to_string(), 0),
        ("R1".to_string(), 1),
        ("R2".to_string(), 2),
        ("R3".to_string(), 3),
        ("R4".to_string(), 4),
        ("R5".to_string(), 5),
        ("R6".to_string(), 6),
        ("R7".to_string(), 7),
        ("R8".to_string(), 8),
        ("R9".to_string(), 9),
        ("R10".to_string(), 10),
        ("R11".to_string(), 11),
        ("R12".to_string(), 12),
        ("R13".to_string(), 13),
        ("R14".to_string(), 14),
        ("R15".to_string(), 15),
        ("SP".to_string(), 0),
        ("LCL".to_string(), 1),
        ("ARG".to_string(), 2),
        ("THIS".to_string(), 3),
        ("THAT".to_string(), 4),
        ("SCREEN".to_string(), 16_384),
        ("KBD".to_string(), 24_576),
    ])
}
