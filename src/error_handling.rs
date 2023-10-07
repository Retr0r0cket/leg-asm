// Culd panic for all of these, but it wouldn't provide good error information
// Also allows for custom error codes instead of just 101 if something goes wrong

pub enum FilesType {
    Opcodes,
    Registers,
    Source,
}

pub fn exit_from_io_error(error: std::io::Error, io_error_file_type: FilesType, file_path: &str) {
    let file_type_as_str: &str = match io_error_file_type {
        FilesType::Opcodes => "opcodes",
        FilesType::Registers => "registers",
        FilesType::Source => "source",
    };

    eprintln!("{}", error);
    eprintln!(
        "Error: failed to read {} file located at {}",
        file_type_as_str, file_path
    );

    let exit_code: i32 = match error.kind() {
        std::io::ErrorKind::NotFound => 20,
        _ => 10,
    } + match io_error_file_type {
        FilesType::Opcodes => 0,
        FilesType::Registers => 1,
        FilesType::Source => 2,
    };

    std::process::exit(exit_code);
}

// Should be able to get rid of this enum and just use file types enum
pub enum JsonFileType {
    Opcodes,
    Registers,
}

pub fn exit_from_json_parsing_error(error: serde_json::Error, json_file_type: JsonFileType) {
    let file_type_as_str: &str = match json_file_type {
        JsonFileType::Opcodes => "opcodes",
        JsonFileType::Registers => "registers",
    };

    eprintln!("{}", error);
    eprintln!("Error: failed to parse {} JSON file data", file_type_as_str);

    let exit_code: i32 = 30
        + match json_file_type {
            JsonFileType::Opcodes => 0,
            JsonFileType::Registers => 1,
        };

    std::process::exit(exit_code);
}
