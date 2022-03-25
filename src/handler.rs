use std::io::BufRead;
use defmt_json_schema::{SchemaVersion, v1};
use chrono::{DateTime, Utc};
use std::time::{UNIX_EPOCH, Duration};
use log::Level;
use colored::*;

pub fn process<T: BufRead>(stream: T) {
    let mut stream = stream;

    if let Some(handle_json_data) = handle_json_schema(&mut stream) {
        handle_json_data(&mut stream);
    } else {
        panic!("Unknown data format, unable to process.");
    }
}

fn handle_json_schema<T: BufRead>(stream: &mut T) -> Option<fn(stream: &mut T)> {
    let mut buffer = String::new();

    stream.read_line(&mut buffer).expect("Error reading JSON schema version");

    let schema_version: SchemaVersion = serde_json::from_str(&buffer).unwrap();

    match schema_version {
        v1::SCHEMA_VERSION => Some(handle_v1_data),
        _ => None,
    }
}

fn handle_v1_data<T: BufRead>(stream: &mut T) {
    loop {
        let mut buffer = String::new();
        let result = stream.read_line(&mut buffer);

        match result {
            Ok(0) => { break /* EOF */ },
            Ok(_) => {
                let frame: v1::JsonFrame = serde_json::from_str(&buffer)
                    .expect("Error parsing JSON.");

                let datetime = DateTime::<Utc>::from(UNIX_EPOCH + Duration::from_nanos(frame.host_timestamp as u64));

                let mut fqn: Vec<String> = vec![];

                if let Some(mut module_path) = frame.location.module_path {
                    fqn.push(module_path.crate_name);
                    fqn.append(&mut module_path.modules);
                    fqn.push(module_path.function);
                }

                let mut source = String::new();

                if let Some(file) = frame.location.file {
                    source.push_str(" [");
                    source.push_str(&file);

                    if let Some(line) = frame.location.line {
                        source.push_str(":");
                        source.push_str(&line.to_string())
                    }

                    source.push_str("]");
                }

                let level = match frame.level {
                    Some(level) => format!("{:5}", level),
                    None => "     ".to_string()
                };

                let output = format!(
                    "{} {}{}{}: {}",
                    datetime.format("%Y-%m-%dT%H:%M:%S.%f"),
                    level,
                    if fqn.len() > 0 { format!(" {}", fqn.join("::")) } else { "".to_string() },
                    source,
                    frame.data
                );

                if let Some(level) = frame.level {
                    match level {
                        Level::Error => println!("{}", output.red()),
                        Level::Warn => println!("{}", output.yellow()),
                        _ => println!("{}", output)
                    };
                }
            },
            Err(_) => panic!("Error while reading stream.")
        }
    }
}
