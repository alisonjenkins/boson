use std::io;

pub fn generate_command(command: &str) -> io::Result<String> {
    let output_command = command;

    Ok(String::from(output_command))
}
