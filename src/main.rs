use std::env::args;
use std::fs::File;

use serde::{Deserialize, Serialize};

mod registers;
mod opcodes;

use registers::Registers;

const LEGASM_FILE_EXTENSION: &str = ".lasm";
const LEGASM_MACHINE_CODE_EXTENSION: &str = ".lmc";

const OPCODES_JSON_LOCATION: &str = "./architecture_data/opcodes.json";
const REGISTERS_JSON_LOCATION: &str = "./architecture_data/registers.json";

const INSTRUCTIONS_PER_LINE: u8 = 4;

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() != 3 {
        panic!("Usage: legasm <source file> <destination>");
    }

    let source: &String = &args[1];
    let destination: &String = &args[2];

    if !source.ends_with(LEGASM_FILE_EXTENSION) {
        println!("Warning: source file should end with '.lasm' but instead ends with {}", LEGASM_FILE_EXTENSION);
    }
    if !destination.ends_with(LEGASM_MACHINE_CODE_EXTENSION) {
        println!("Warning: destination file should end with '.lmc' but instead ends with {}", LEGASM_MACHINE_CODE_EXTENSION);
    }

    let registers_file_data = File::open(REGISTERS_JSON_LOCATION).expect("Failed to open registers data file");
    let registers_json_data: registers::Registers = serde_json::from_reader(registers_file_data).expect("Failed to parse registers data file");
    {
        if registers_json_data.check_for_duplicates == True {
            panic!("Register json contains duplicate entries");
        }
        if registers_json_data.check_for_uppercase == True {
            panic("Register json includes registers with uppercase");
        }
    };

    //read with buffer for source file
    let source_file_data = File::open(source).expect("Failed to open source file");
    let opcodes_file_data = File::open(OPCODES_JSON_LOCATION).expect("Failed to open opcodes data file");
}
