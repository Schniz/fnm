use std::fmt::Display;

use clap::ValueEnum;

#[derive(Debug, Default, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, ValueEnum)]
pub enum LogLevel {
    Quiet,
    Error,
    #[default]
    #[value(alias("all"))]
    Info,
}

impl LogLevel {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Quiet => "quiet",
            Self::Info => "info",
            Self::Error => "error",
        }
    }

    pub fn is_writable(self, logging: Self) -> bool {
        use std::cmp::Ordering;
        matches!(self.cmp(&logging), Ordering::Greater | Ordering::Equal)
    }

    pub fn writer_for(self, logging: Self) -> Box<dyn std::io::Write> {
        if self.is_writable(logging) {
            match logging {
                Self::Error => Box::from(std::io::stderr()),
                _ => Box::from(std::io::stdout()),
            }
        } else {
            Box::from(std::io::sink())
        }
    }

    pub fn possible_values() -> &'static [&'static str; 4] {
        &["quiet", "info", "all", "error"]
    }
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[macro_export]
macro_rules! outln {
    ($config:ident, $level:path, $($expr:expr),+) => {{
        use $crate::log_level::LogLevel::*;
        writeln!($config.log_level().writer_for($level), $($expr),+).expect("Can't write output");
    }}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_writable() {
        assert!(!LogLevel::Quiet.is_writable(LogLevel::Info));
        assert!(!LogLevel::Error.is_writable(LogLevel::Info));
        assert!(LogLevel::Info.is_writable(LogLevel::Info));
        assert!(LogLevel::Info.is_writable(LogLevel::Error));
    }
}
