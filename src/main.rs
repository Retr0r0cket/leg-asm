use std::fs::File;

mod error_handling;
mod registers;

// Default file extensions
const LEGASM_FILE_EXTENSION: &str = ".lasm";
const LEGASM_MACHINE_CODE_EXTENSION: &str = ".lmc";

// Default architecture info json locations
const OPCODES_JSON_LOCATION: &str = "./architecture_data/opcodes.json";
const REGISTERS_JSON_LOCATION: &str = "./architecture_data/registers.json";

// Architecture does not contain variable length instructions, must be 4 instructions per line
const INSTRUCTIONS_PER_LINE: u8 = 4;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        panic!("Usage: legasm <source file> <destination>");
    }

    let source_file_location: &String = &args[1];
    let destination_file_location: &String = &args[2];

    if !source_file_location.ends_with(LEGASM_FILE_EXTENSION) {
        println!(
            "Warning: source file should end with '.lasm' but instead ends with {}",
            LEGASM_FILE_EXTENSION
        );
    }
    if !destination_file_location.ends_with(LEGASM_MACHINE_CODE_EXTENSION) {
        println!(
            "Warning: destination file should end with '.lmc' but instead ends with {}",
            LEGASM_MACHINE_CODE_EXTENSION
        );
    }

    let registers_data = registers::Registers::new(REGISTERS_JSON_LOCATION);

    // Read with buffer for source file, don't know how long it will be
    // Opcodes and registers are small enough to read as a string
    let source_file_data = File::open(source_file_location).expect("Failed to open source file");
    let opcodes_file_data =
        File::open(OPCODES_JSON_LOCATION).expect("Failed to open opcodes data file");
}

// add conf file
// add tests
// add documentation
// add embedded json data