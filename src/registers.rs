use crate::error_handling;

pub struct Registers {
    data_registers: Vec<String>,
    inputs: Vec<String>,
    outputs: Vec<String>,
}

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

impl Registers {
    pub fn new(file_path: &str) -> Registers {
        let registers_file_data:Result<std::fs::File, std::io::Error> = std::fs::File::open(file_path);
        let registers_file_data_result: std::fs::File = match registers_file_data {
            Ok(file) => file,
            Err(error) => error_handling::exit_from_io_error(error, error_handling::FilesType::Registers, file_path),
        };
        let registers_json_data: Registers  = serde_json::from_reader(registers_file_data).unwrap();
        registers_json_data.init_checks(file_path);
        registers_json_data
    }

    fn init_checks(&self, file_path: &str )  {
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

            println!("Error: error with registers file located at {}{}{}", file_path, uppercase_error, duplicates_error);
            std::process::exit(1);
        }  
    }
    
    fn check_for_duplicates (&self) -> bool {
        let all_item_iter = self.data_registers.iter().chain(self.inputs.iter()).chain(self.outputs.iter());

        let mut duplicates = false;
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

    fn check_for_uppercase (&self) -> bool {
        let mut case = false;
        for register in self.data_registers.iter().chain(self.inputs.iter()).chain(self.outputs.iter()) {
            if register.chars().any(char::is_uppercase) {
                true;
            }
        }
        false
    } 
}
