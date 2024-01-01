#![feature(str_split_whitespace_remainder)]

use std::fs::File;
use std::io::BufRead;
use std::path::Path;

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

#[repr(u8)]
enum OutputTextType {
    Hexadecimal,
    Base10,
    Plaintext,
}

struct Config<'a> {
    output_mode: OutputTextType,
    use_embedded_json: bool,
    opcode_json_location: &'a str,
    registers_json_location: &'a str,
}
fn main() {
    let config = parse_config("./leg-asm.conf");
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        eprintln!(
            "Usage: {:?} <source file> <destination>",
            std::env::current_exe()
        );
        std::process::exit(1);
    }

    let source_file_location: &str = &args[1];
    let destination_file_location: &str = &args[2];

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
}

fn parse_config(config_file_path: &str) -> Config {
    let config_file = File::open(config_file_path).unwrap();
    let config_lines = lines_from_file(config_file_path);

    for line in config_lines {
        let line_as_vec = line.split_whitespace().remainder().unwrap().filter();
    }

    {
        no
    }
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = std::io::BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

// add conf file
// add tests
// add documentation
// add embedded json data
// add register alias support
// make file listing what all exit codes actually mean
// Add hex output support
// Use proper .expect/
// Handle filenames/paths properly
// deal with import statements at top of files
