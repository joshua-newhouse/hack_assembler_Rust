use log::LevelFilter::{Debug, Info};
use simple_logger::SimpleLogger;

fn main() {
    SimpleLogger::new()
        .with_level(if let Ok(_) = std::env::var("DEBUG") { Debug } else { Info })
        .init()
        .unwrap();

    let args: Vec<_> = std::env::args().collect();

    let config: hack_assembler::Config;
    match hack_assembler::Config::build(args[1..].to_vec()) {
        Ok(c) => config = c,
        Err(msg) => {
            log::error!("{msg}");
            std::process::exit(1);
        }
    }

    log::debug!("assembling with the following configuration\n{:#?}", config);

    let codes = hack_assembler::Codes::new();
    let encoder = hack_assembler::Encoder::new(&codes);
    let parser = hack_assembler::Parser::new(&codes);
    let symbol_table = hack_assembler::SymbolTable::new();
    let mut application = hack_assembler::Application::new(config, parser, symbol_table, encoder);

    if let Err(code) = application.run() {
        std::process::exit(code);
    }
}
