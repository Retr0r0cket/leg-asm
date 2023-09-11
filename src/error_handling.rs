enum FilesType {
    Opcodes,
    Registers,
    Source
}

fn exit_from_io_error(error: std::io::Error, io_error_file_type: FilesType, file_path: &str) {
    let file_type_as_str: &str = match io_error_file_type {
        FilesType::Opcodes => "opcodes",
        FilesType::Registers => "registers",
        FilesType::Source => "source",
    };

    let mut exit_code: i32 = match error.kind() {
        std::io::ErrorKind::NotFound => 20,
        _ => 10,
    };

    exit_code += match io_error_file_type {
        FilesType::Opcodes => 0,
        FilesType::Registers => 1,
        FilesType::Source => 2,
    };

    eprintln!("{}", error);
    eprintln!("Error: failed to read {} file located at {}", file_type_as_str, file_path);

    std::process::exit(exit_code);
}