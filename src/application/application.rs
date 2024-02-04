use std::fs::File;
use std::io::{BufRead, BufReader, Write};

use log::LevelFilter;

use crate::{Config, Encoder, Parser, SymbolTable};
use crate::parser::instruction::Instruction;

pub struct Application<'a> {
    config: Config,
    parser: Parser<'a>,
    symbol_table: SymbolTable,
    encoder: Encoder<'a>,
    failed: bool,
}

impl Application<'_> {
    pub fn new<'a>(
        config: Config,
        parser: Parser<'a>,
        symbol_table: SymbolTable,
        encoder: Encoder<'a>,
    ) -> Application<'a> {
        Application { config, parser, symbol_table, encoder, failed: false }
    }

    pub fn run(&mut self) -> Result<(), i32> {
        log::debug!("assembling...");

        let input_reader = match File::open(&self.config.input_file_path) {
            Ok(file) => BufReader::new(file),
            Err(e) => {
                log::error!("failed opening {:#?} for reading\n{e}", &self.config.input_file_path);
                return Err(2);
            }
        };

        let output_file = match File::create(&self.config.output_file_path) {
            Ok(file) => file,
            Err(e) => {
                log::error!("failed opening {:#?} for writing\n{e}", &self.config.output_file_path);
                return Err(4);
            }
        };

        let instructions = self.parse_input_into_instructions(input_reader);

        if self.failed {
            return Err(8);
        }

        self.symbol_table.resolve_addresses();

        self.write_instructions_to_file(instructions, output_file);

        if self.failed {
            return Err(16);
        }

        Ok(())
    }

    fn parse_input_into_instructions(&mut self, input_reader: BufReader<File>) -> Vec<Instruction> {
        return input_reader.lines()
            .flatten()
            .map(|line| {
                match line.split_once("//") {
                    Some((code, _)) => String::from(code),
                    None => line,
                }
            })
            .map(|mut line| {
                line.retain(|c| !c.is_whitespace());
                line
            })
            .filter(|line| !line.is_empty())
            .enumerate()
            .map(|(line_num, asm_instr)| self.parser.generate_instruction(line_num, asm_instr))
            .inspect(|instr|
                match instr {
                    Instruction::Error => self.failed = true,
                    Instruction::AInstrSymbol(_) | Instruction::LInstr(_, _) => {
                        if let Err(_) = self.symbol_table.add_symbol(instr) {
                            self.failed = true;
                        }
                    }
                    _ => (),
                }
            )
            .filter(|instr|
                match instr {
                    Instruction::LInstr(_, _) | Instruction::Error => false,
                    _ => true,
                }
            )
            .collect();
    }

    fn write_instructions_to_file(&mut self, instructions: Vec<Instruction>, mut output_file: File) {
        instructions.into_iter()
            .inspect(|instr| {
                if log::max_level() == LevelFilter::Debug {
                    println!("{:?}", instr)
                }
            })
            .map(|instr| self.encoder.to_binary(instr, &self.symbol_table))
            .inspect(|instr| {
                if log::max_level() == LevelFilter::Debug {
                    println!("{instr}")
                }
            })
            .for_each(|binary_instr| {
                if self.failed {
                    return;
                }

                match output_file.write_all(binary_instr.as_bytes()) {
                    Ok(_) => (),
                    Err(e) => {
                        log::error!("failed writing instruction to file\n{e}");
                        self.failed = true;
                    }
                }
            });
    }
}
