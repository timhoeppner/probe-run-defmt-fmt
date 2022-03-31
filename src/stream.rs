use std::io::BufRead;
use defmt_json_schema::{SchemaVersion, v1};
use crate::output::LogEntry;

pub fn process<T: BufRead>(stream: T) {
    let mut stream = stream;

    if let Some(handle_json_data) = handle_json_schema(&mut stream) {
        loop {
            let mut buffer = String::new();
            let result = stream.read_line(&mut buffer);

            match result {
                Ok(0) => break, /* EOF */
                Ok(_) => handle_json_data(buffer).print(),
                Err(_) => panic!("Error while reading stream.")
            }
        }
    } else {
        panic!("Unknown data format, unable to process.");
    }
}

fn handle_json_schema<T: BufRead>(stream: &mut T) -> Option<fn(buffer: String) -> LogEntry> {
    let mut buffer = String::new();

    stream.read_line(&mut buffer).expect("Error reading JSON schema version");

    let schema_version: SchemaVersion = serde_json::from_str(&buffer).unwrap();

    match schema_version {
        v1::SCHEMA_VERSION => Some(handle_v1_data),
        _ => None,
    }
}

fn handle_v1_data(buffer: String) -> LogEntry {
    serde_json::from_str::<v1::JsonFrame>(&buffer).expect("").into()
}
