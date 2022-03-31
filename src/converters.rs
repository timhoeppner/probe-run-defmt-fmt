use std::time::{Duration, UNIX_EPOCH};
use chrono::{DateTime, Utc};
use defmt_json_schema::v1;
use log::Level;
use crate::output::{Color, LogEntry};

impl Into<LogEntry> for v1::JsonFrame {
    fn into(self) -> LogEntry {
        let date_time = DateTime::<Utc>::from(UNIX_EPOCH + Duration::from_nanos(self.host_timestamp as u64));

        let mut fqn: Vec<String> = vec![];

        if let Some(mut module_path) = self.location.module_path {
            fqn.push(module_path.crate_name);
            fqn.append(&mut module_path.modules);
            fqn.push(module_path.function);
        }

        let fqn = if fqn.len() > 0 {
            format!(" {}", fqn.join("::"))
        } else {
            "".to_string()
        };

        let mut source = String::new();

        if let Some(file) = self.location.file {
            source.push_str(" [");
            source.push_str(&file);

            if let Some(line) = self.location.line {
                source.push_str(":");
                source.push_str(&line.to_string())
            }

            source.push_str("]");
        }

        let color = if let Some(level) = self.level {
            match level {
                Level::Error => Color::Red,
                Level::Warn => Color::Yellow,
                _ => Color::None,
            }
        } else {
            Color::None
        };

        LogEntry {
            date_time,
            level: self.level,
            fqn,
            source,
            data: self.data,
            color
        }
    }
}
