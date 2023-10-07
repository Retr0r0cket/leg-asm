use crate::error_handling;
use serde_json::Value;

pub struct Opcodes {
    pub name: String,
    pub bit_prefix: i64,
    pub num_of_args: i64,
    pub opcode_set: i64,
}

pub fn init_opcodes_list(file_path: &str) -> Vec<Opcodes> {
    let opcodes_file = crate::read_file_or_io_error(file_path, error_handling::FilesType::Opcodes);
    let opcodes_json_result: Result<serde_json::Value, serde_json::Error> =
        serde_json::from_reader::<&std::fs::File, serde_json::Value>(&opcodes_file);
    if opcodes_json_result.is_err() {
        error_handling::exit_from_json_parsing_error(
            serde_json::from_reader::<std::fs::File, serde_json::Value>(opcodes_file).unwrap_err(),
            error_handling::JsonFileType::Opcodes,
        );
    }

    let opcodes_json = opcodes_json_result.unwrap();
    let opcodes_json_array_option = opcodes_json.as_array();
    if opcodes_json_array_option.is_none() {
        opcodes_empty_error(file_path)
    }
    let opcodes_as_array = opcodes_json_array_option.unwrap();
    if opcodes_as_array.len() < 2 {
        opcodes_empty_error(file_path);
    }

    let mut opcodes_sets: Vec<Value> = Vec::new();
    let mut opcode_set_iter = opcodes_as_array.iter();
    _ = opcode_set_iter.next_back();

    for set in opcode_set_iter {
        let set_length = set.as_array().unwrap().len();

        if set_length == 0 || set_length > 8 {
            eprintln!(
                "Error: opcode set {} contains too many (>8) or too few (0) elements",
                set
            );
            std::process::exit(41);
        }
        opcodes_sets.push(set.clone());
    }

    let mut opcodes: Vec<Opcodes> = Vec::new();
    let mut set_count: u8 = 0;

    for set in opcodes_sets.into_iter() {
        for subset in set.as_array().to_owned().unwrap().into_iter() {
            for opcode in subset.as_array().to_owned().unwrap().into_iter() {
                let name = opcode["name"].as_str().unwrap().to_owned();
                let bit_prefix = opcode["bit_prefix"].as_u64().unwrap() as i64;
                let num_of_args = opcode["num_of_args"].as_u64().unwrap() as i64;
                let opcode_set = set_count as i64;

                opcodes.push(Opcodes {
                    name,
                    bit_prefix,
                    num_of_args,
                    opcode_set,
                });
            }
        }
        set_count += 1;
    }

    let mut seen: Vec<&String> = Vec::new();

    for opcode in opcodes.iter() {
        if opcode.bit_prefix > 7 {
            eprintln!(
                "Error: opcode {} has a bit prefix greater than 8",
                opcode.name
            );
            std::process::exit(42);
        }
        if opcode.num_of_args < 1 || opcode.num_of_args > 2 {
            eprintln!(
                "Error: opcode {} has invalid number of arguments",
                opcode.name
            );
            std::process::exit(43);
        }

        if opcode.opcode_set < 0 || opcode.opcode_set > 3 {
            eprintln!("Error: opcode {} has invalid opcode set", opcode.name);
            std::process::exit(44);
        }

        if opcode.name.chars().any(char::is_uppercase) {
            eprintln!("Error: opcode {} contains uppercase letters", opcode.name);
            std::process::exit(45);
        }

        if seen.contains(&&opcode.name) {
            eprintln!("Error: opcode {} is a duplicate", opcode.name);
            std::process::exit(46);
        }
        seen.push(&opcode.name);
    }

    opcodes
}

fn opcodes_empty_error(file_path: &str) {
    eprintln!(
        "Error: opcodes file located at {} is empty or does not contain bit code",
        file_path
    );
    std::process::exit(40);
}
