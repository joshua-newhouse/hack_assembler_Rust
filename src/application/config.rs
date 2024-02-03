use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Config {
    pub input_file_path: PathBuf,
    pub output_file_path: PathBuf,
}

impl Config {
    pub fn build(options: Vec<String>) -> Result<Config, &'static str> {
        if options.len() == 0 {
            return Err("no options found");
        }

        if options.len() == 1 {
            return match options.get(0) {
                Some(string_ref) => {
                    let mut output_file_path = string_ref.clone();
                    if output_file_path.ends_with(".asm") {
                        output_file_path = output_file_path.replace(".asm", ".hack");
                    } else {
                        output_file_path.push_str(".hack");
                    }

                    let input_path = PathBuf::from(string_ref);
                    let out_path = PathBuf::from(output_file_path);
                    Ok(Config { input_file_path: input_path, output_file_path: out_path })
                }
                None => Err("failed getting input file path"),
            };
        }

        let mut option_map: HashMap<&str, String> = HashMap::new();
        options.iter()
            .enumerate()
            .for_each(|(idx, option)| {
                if option.starts_with('-') {
                    match option.as_ref() {
                        "-i" => { _ = option_map.insert("-i", String::from(options.get(idx + 1).unwrap())) }
                        "-o" => { _ = option_map.insert("-o", String::from(options.get(idx + 1).unwrap())) }
                        _ => eprintln!("unknown option: {option}"),
                    }
                }
            });

        let input_file_path: String;
        let mut output_file_path: String;

        if let Some(input_path) = option_map.remove("-i") {
            input_file_path = input_path;
        } else {
            return Err("no input file path specified");
        }

        if let Some(output_path) = option_map.remove("-o") {
            output_file_path = output_path;
        } else {
            output_file_path = input_file_path.clone().replace(".asm", ".hack");
            if !output_file_path.ends_with(".hack") {
                output_file_path.push_str(".hack");
            }
        }

        if input_file_path.eq(&output_file_path) {
            return Err("input and output files are the same");
        }

        Ok(Config { input_file_path: PathBuf::from(input_file_path), output_file_path: PathBuf::from(output_file_path) })
    }
}
