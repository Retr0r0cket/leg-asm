mod error_handling;
mod opcodes;
mod registers;

// Default file extensions
const LEGASM_FILE_EXTENSION: &str = ".lasm";
const LEGASM_MACHINE_CODE_EXTENSION: &str = ".lmc";

// Default architecture info json locations
const OPCODES_JSON_LOCATION: &str = "./architecture_data/opcodes.json";
const REGISTERS_JSON_LOCATION: &str = "./architecture_data/registers.json";

// Architecture does not contain variable length instructions, must be 4 instructions per line
const INSTRUCTIONS_PER_LINE: u8 = 4;
const OPCODES_BIT_PREFIX: u8 = 8;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        panic!("Usage: lasm <source file> <destination>");
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

    // Opcodes and registers are small enough to read as a string
    let registers_data = registers::Registers::new(REGISTERS_JSON_LOCATION);
    let opcodes_list: Vec<opcodes::Opcodes> = opcodes::new(OPCODES_JSON_LOCATION);

    // Read with buffer for source file, don't know how long it will be
    let source_file_data =
        read_file_or_io_error(source_file_location, error_handling::FilesType::Source);
    let source_file_buffer = std::io::BufReader::new(source_file_data);
}

fn read_file_or_io_error(file_path: &str, file_type: error_handling::FilesType) -> std::fs::File {
    let file = if std::fs::File::open(file_path).is_ok() {
        Some(std::fs::File::open(file_path).unwrap())
    } else {
        error_handling::exit_from_io_error(
            std::fs::File::open(file_path).unwrap_err(),
            file_type,
            file_path,
        );
        None
    }
    .unwrap();
    return file;
}

// add conf file
// add tests
// add documentation
// add embedded json data
// add register alias support
// make file listing what all exit codes actually mean
