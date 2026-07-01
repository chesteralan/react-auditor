pub mod compact;
pub mod json;
pub mod stylish;

use crate::scanner::ScanResult;

pub trait Formatter {
    fn format(&self, results: &[ScanResult], quiet: bool) -> String;
}

pub fn get_formatter(name: &str) -> Box<dyn Formatter> {
    match name {
        "json" => Box::new(json::JsonFormatter),
        "compact" => Box::new(compact::CompactFormatter),
        _ => Box::new(stylish::StylishFormatter),
    }
}
