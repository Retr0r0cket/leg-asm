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

// Default binary name for assembler
const DEFAULT_BINARY_NAME: &str = "./lasm";

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <source file> <destination>", DEFAULT_BINARY_NAME);
        std::process::exit(1);
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
    let registers_data = registers::Registers::init(REGISTERS_JSON_LOCATION);
    let opcodes_list: Vec<opcodes::Opcodes> = opcodes::init_opcodes_list(OPCODES_JSON_LOCATION);
    let opcode_name_list: Vec<String> = opcodes_list
        .iter()
        .map(|opcode| opcode.name.to_owned())
        .collect();

    let source_file =
        read_file_or_io_error(source_file_location, error_handling::FilesType::Source);
    let source_buf_reader = std::io::BufReader::new(source_file);
    let source_line_iter = std::io::BufRead::lines(source_buf_reader);

    let mut jump_labels: Vec<String> = Vec::new();

    for line in source_line_iter {
        if line.is_err() {
            error_handling::exit_from_io_error(
                line.unwrap_err(),
                error_handling::FilesType::Source,
                source_file_location,
            );
            std::process::exit(32);
        }
        let line_string = line.unwrap();
        let line_word_vector: Vec<&str> = line_string.split_whitespace().collect();
    }
}

fn read_file_or_io_error(file_path: &str, file_type: error_handling::FilesType) -> std::fs::File {
    let file = std::fs::File::open(file_path);
    if file.is_err() {
        error_handling::exit_from_io_error(
            std::fs::File::open(file_path).unwrap_err(),
            file_type,
            file_path,
        );
    }
    file.unwrap()
}

// add conf file
// add tests
// add documentation
// add embedded json data
// add register alias support
// make file listing what all exit codes actually mean
// Add hex output support
