#[cfg(test)]
mod config_tests {
    use std::path::PathBuf;

    use crate::Config;

    #[test]
    fn no_options() {
        let config = Config::build(vec![]);
        match config {
            Ok(_) => assert!(false, "received Ok Config when no options were specified"),
            Err(_) => assert!(true),
        }
    }

    #[test]
    fn input_file_only_with_asm_extension() {
        let config = Config::build(vec!["path/to/assembly-file.asm".to_string()]);
        match config {
            Ok(config) => {
                assert_eq!(config.input_file_path, PathBuf::from("path/to/assembly-file.asm"));
                assert_eq!(config.output_file_path, PathBuf::from("path/to/assembly-file.hack"));
            }
            Err(e) => assert!(false, "received Err Config {}", e),
        }
    }

    #[test]
    fn input_file_only_without_asm_extension() {
        let config = Config::build(vec!["path/to/assembly-file".to_string()]);
        match config {
            Ok(config) => {
                assert_eq!(config.input_file_path, PathBuf::from("path/to/assembly-file"));
                assert_eq!(config.output_file_path, PathBuf::from("path/to/assembly-file.hack"));
            }
            Err(e) => assert!(false, "received Err Config {}", e),
        }
    }

    #[test]
    fn input_and_output_specified_successfully1() {
        let config = Config::build(
            vec![
                "-i".to_string(),
                "path/to/assembly-file".to_string(),
                "-o".to_string(),
                "target/binary".to_string(),
            ]
        );

        match config {
            Ok(config) => {
                assert_eq!(config.input_file_path, PathBuf::from("path/to/assembly-file"));
                assert_eq!(config.output_file_path, PathBuf::from("target/binary"));
            }
            Err(e) => assert!(false, "received Err Config {}", e),
        }
    }

    #[test]
    fn input_and_output_specified_successfully2() {
        let config = Config::build(
            vec![
                "-o".to_string(),
                "target/binary".to_string(),
                "-i".to_string(),
                "path/to/assembly-file".to_string(),
            ]
        );

        match config {
            Ok(config) => {
                assert_eq!(config.input_file_path, PathBuf::from("path/to/assembly-file"));
                assert_eq!(config.output_file_path, PathBuf::from("target/binary"));
            }
            Err(e) => assert!(false, "received Err Config {}", e),
        }
    }

    #[test]
    fn input_and_output_with_same_path() {
        let path_to_same_file = "path/to/assembly-file".to_string();
        let config = Config::build(
            vec![
                "-o".to_string(),
                path_to_same_file.clone(),
                "-i".to_string(),
                path_to_same_file.clone(),
            ]
        );

        match config {
            Ok(config) => assert!(false, "received Ok config when input and output paths are the same: {:?}", config),
            Err(_) => assert!(true),
        }
    }

    #[test]
    fn no_input_but_output() {
        let config = Config::build(
            vec![
                "-o".to_string(),
                "target/binary".to_string(),
            ]
        );

        match config {
            Ok(config) => assert!(false, "received Ok config when no input specified: {:?}", config),
            Err(_) => assert!(true),
        }
    }

    #[test]
    fn no_output_but_input_with_asm_extension() {
        let config = Config::build(
            vec![
                "-i".to_string(),
                "path/to/assembly-file.asm".to_string(),
            ]
        );

        match config {
            Ok(config) => {
                assert_eq!(config.input_file_path, PathBuf::from("path/to/assembly-file.asm"));
                assert_eq!(config.output_file_path, PathBuf::from("path/to/assembly-file.hack"));
            },
            Err(e) => assert!(false, "received Err Config {}", e),
        }
    }

    #[test]
    fn no_output_but_input_without_asm_extension() {
        let config = Config::build(
            vec![
                "-i".to_string(),
                "path/to/assembly-file".to_string(),
            ]
        );

        match config {
            Ok(config) => {
                assert_eq!(config.input_file_path, PathBuf::from("path/to/assembly-file"));
                assert_eq!(config.output_file_path, PathBuf::from("path/to/assembly-file.hack"));
            },
            Err(e) => assert!(false, "received Err Config {}", e),
        }
    }
}
