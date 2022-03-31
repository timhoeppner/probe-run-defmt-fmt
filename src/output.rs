use chrono::{DateTime, Utc};
use log::Level;
use std::fmt;
use std::fmt::Formatter;
use colored::*;

pub struct LogEntry {
    pub date_time: DateTime<Utc>,
    pub level: Option<Level>,
    pub fqn: String,
    pub source: String,
    pub data: String,
    pub color: Color,
}

pub enum Color {
    None,
    Yellow,
    Red,
}

impl fmt::Display for LogEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let level = match self.level {
            Some(level) => format!("{:5}", level),
            None => "     ".to_string()
        };

        write!(
            f,
            "{} {}{}{}: {}",
            self.date_time.format("%Y-%m-%dT%H:%M:%S.%f"),
            level,
            self.fqn,
            self.source,
            self.data
        )
    }
}

impl LogEntry {
    pub fn print(&self) {
        match self.color {
            Color::Red => println!("{}", self.to_string().red()),
            Color::Yellow => println!("{}", self.to_string().yellow()),
            _ => println!("{}", self)
        }
    }
}
