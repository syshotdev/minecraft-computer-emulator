// Purpose of this file:
// Parses an emulator machine config file and passes that to the main program.
// The machine config file should have the opcode and opcode value (ADD 37)
// Maybe ram size and whatever will be defined in there? I'd rather just set those in code so nah.

use std::fs::{self, OpenOptions};
use std::io::{Read, BufRead, BufReader, Write};
use std::path::Path;
use crate::types::OpcodeEntry;
use crate::format_err;

// Expects an opcode and value, (ADD 37)
pub fn parse_machine_file(file_path: &str) -> Result<Vec<OpcodeEntry>, String> {
    let path = Path::new(file_path);
    println!("Trying to open file '{}'", path.display());

    // Ensure the file ends with a newline
    ensure_newline_at_end(&path)?;

    // Open the file
    let file = match fs::File::open(&path) {
        Ok(file) => file,
        Err(e) => return format_err!("Error opening file: {}", e),
    };

    let reader = BufReader::new(file);
    let mut entries = Vec::new();

    for line in reader.lines() {
        match line {
            Ok(line) => {
                let parts: Vec<&str> = line.split_whitespace().collect();

                if parts.len() != 2 {
                    return format_err!("Parsing error: line '{}' does not have exactly two parts", line);
                }
                let opcode = parts[0].to_string();
                let number: u32 = match parts[1].parse() {
                    Ok(number) => number,
                    Err(_) => return format_err!("Parsing error: '{}' is not a valid number", parts[1]),  
                };

                // Just gave up on trying to use .try_into()
                entries.push(OpcodeEntry { opcode, number: number as usize });
            },

            Err(e) => return format_err!("Could not read line: {}", e),
        }
    }

    Ok(entries)
}

// Does what it says, but also error handling
fn ensure_newline_at_end(path: &Path) -> Result<(), String> {
    let metadata = fs::metadata(path).map_err(|e| format!("Error opening file: {e}"))?;

    if metadata.len() == 0 {
        return Ok(()); // Empty file, no need to add newline
    };

    // I need to stick to a standard when it comes to errors
    // Like map_err for compounding dot syntax but format_err for match statements that need to be
    // matched  (or the variables can't fit in one letter)
    let file = fs::File::open(path).map_err(|e| format!("Error opening file: {e}"))?;

    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer).map_err(|e| format!("Error reading file: {e}"))?;

    if !buffer.ends_with(b"\n") {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(path)
            .map_err(|e| format!("Error appending newline to file: {e}"))?;
        writeln!(file).map_err(|e| format!("Error appending newline to file: {e}"))?;
    }

    Ok(())
}
