// Purpose of this file:
// Parses an emulator machine config file and passes that to the main program.
// The machine config file should have the opcode and opcode value (ADD 37)
// Maybe ram size and whatever will be defined in there? I'd rather just set those in code so nah.

use std::fs::{self, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

#[derive(Debug)]
struct OpcodeEntry {
    opcode: String,
    number: u32,
}

// Expects an opcode and value, (ADD 37)
fn parse_machine_file(file_path: &str) -> Vec<OpcodeEntry> {
    let path = Path::new(file_path);

    // Ensure the file ends with a newline
    ensure_newline_at_end(&path);

    // Open the file
    let file = match fs::File::open(&path) {
        Ok(file) => file,
        Err(err) => panic!("Error opening file: {}", err),
    };

    let reader = BufReader::new(file);
    let mut entries = Vec::new();

    for line in reader.lines() {
        match line {
            Ok(line) => {
                let parts: Vec<&str> = line.split_whitespace().collect();

                if parts.len() != 2 {
                    panic!("Parsing error: line '{}' does not have exactly two parts", line);
                }

                let opcode = parts[0].to_string();
                let number: u32 = parts[1].parse().unwrap_or_else(|_| {
                    panic!("Parsing error: '{}' is not a valid number", parts[1])
                });

                entries.push(OpcodeEntry { opcode, number });
            }
            Err(err) => panic!("Error reading line: {}", err),
        }
    }

    entries
}

// Does what it says, but also error handling
fn ensure_newline_at_end(path: &Path) {
    let metadata = match fs::metadata(path) {
        Ok(metadata) => metadata,
        Err(_) => return, // File does not exist or can't access metadata
    };

    if metadata.len() == 0 {
        return; // Empty file, no need to add newline
    }

    let file = match fs::File::open(path) {
        Ok(file) => file,
        Err(err) => panic!("Error opening file: {}", err),
    };

    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer).expect("Error reading file");

    if !buffer.ends_with(b"\n") {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(path)
            .expect("Error opening file for appending a newline");
        writeln!(file).expect("Error writing newline to file");
    }
}
