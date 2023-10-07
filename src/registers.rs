use crate::error_handling;

// No input overlap in register vectors
#[derive(serde::Deserialize, Debug)]
pub struct Registers {
    data_registers: Vec<String>,
    inputs: Vec<String>,
    outputs: Vec<String>,
}

impl Registers {
    pub fn init(file_path: &str) -> Registers {
        let registers_file =
            crate::read_file_or_io_error(file_path, error_handling::FilesType::Registers);

        let raw_json: Result<Registers, serde_json::Error> =
            serde_json::from_reader::<&std::fs::File, Registers>(&registers_file);
        if raw_json.is_err() {
            error_handling::exit_from_json_parsing_error(
                serde_json::from_reader::<std::fs::File, Registers>(registers_file).unwrap_err(),
                error_handling::JsonFileType::Registers,
            );
        }
        let json_data: Registers = raw_json.unwrap();

        let mut seen: Vec<&String> = Vec::new();
        let registers_list = [
            json_data.data_registers.to_owned(),
            json_data.inputs.to_owned(),
            json_data.outputs.to_owned(),
        ]
        .concat();

        for register in registers_list.iter() {
            if register.chars().any(char::is_uppercase) {
                println!("Error: uppercase letters are not allowed in register names");
                println!("{}", file_path);
                std::process::exit(50);
            }
            if seen.contains(&register) {
                println!("Error: duplicate register names are not allowed");
                println!("{}", file_path);
                std::process::exit(50);
            }
            seen.push(&register);
        }
        json_data
    }
}
