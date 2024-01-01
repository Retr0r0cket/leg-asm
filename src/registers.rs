use std::fs::File;

// No input overlap in register vectors
#[derive(serde::Deserialize, Debug)]
pub struct Registers {
    data_registers: Vec<String>,
    inputs: Vec<String>,
    outputs: Vec<String>,
}

impl Registers {
    pub fn init(file_path: &str) -> Registers {
        let registers_file = File::open(file_path).unwrap();

        let json_data = serde_json::from_reader::<&File, Registers>(&registers_file).unwrap();

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
