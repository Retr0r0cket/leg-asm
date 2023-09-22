use std::fs::File;

use crate::error_handling;

// No input overlap in register vectors
#[derive(serde::Deserialize, Debug)]
pub struct Registers {
    data_registers: Vec<String>,
    inputs: Vec<String>,
    outputs: Vec<String>,
}

// Order doesn't matter for this iterator
impl Iterator for Registers {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        let mut next: Option<String> = None;
        if !self.data_registers.is_empty() {
            next = self.data_registers.pop();
        } else if !self.inputs.is_empty() {
            next = self.inputs.pop();
        } else if !self.outputs.is_empty() {
            next = self.outputs.pop();
        }
        next
    }
}

// Clean this up if you can
impl Registers {
    pub fn new(file_path: &str) -> Registers {
        let registers_file =
            crate::read_file_or_io_error(file_path, error_handling::FilesType::Registers);

        let registers_json_data: Option<Registers> =
            if serde_json::from_reader::<&File, Registers>(&registers_file).is_ok() {
                Some(serde_json::from_reader(registers_file).unwrap())
            } else {
                error_handling::exit_from_json_parsing_error(
                    serde_json::from_reader::<File, Registers>(registers_file).unwrap_err(),
                    error_handling::JsonFileType::Registers,
                );
                None
            }
            .unwrap();
        let decoded_json_data: Registers = registers_json_data.unwrap();
        decoded_json_data.init_checks(file_path);
        decoded_json_data
    }

    // Will probably collapse this into "new" function
    fn init_checks(&self, file_path: &str) {
        let uppercase: bool = self.check_for_uppercase();
        let duplicates: bool = self.check_for_duplicates();

        if uppercase == true || duplicates == true {
            let uppercase_error: &str = match uppercase {
                true => ", uppercase letters are not allowed in register names",
                false => "",
            };
            let duplicates_error: &str = match duplicates {
                true => ", duplicate register names are not allowed",
                false => "",
            };

            println!(
                "Error: error with registers file located at {}{}{}",
                file_path, uppercase_error, duplicates_error
            );
            std::process::exit(50);
        }
    }

    fn check_for_duplicates(&self) -> bool {
        let all_item_iter = self
            .data_registers
            .iter()
            .chain(self.inputs.iter())
            .chain(self.outputs.iter());

        let mut seen: Vec<&String> = Vec::new();
        for register in all_item_iter {
            if seen.contains(&register) {
                true;
            } else {
                seen.push(register);
            }
        }
        false
    }

    fn check_for_uppercase(&self) -> bool {
        for register in self
            .data_registers
            .iter()
            .chain(self.inputs.iter())
            .chain(self.outputs.iter())
        {
            if register.chars().any(char::is_uppercase) {
                true;
            }
        }
        false
    }
}
