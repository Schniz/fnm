use miette::SourceOffset;

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
#[error("malformed json\n{}", self.report())]
pub struct DecodeError {
    cause: serde_json::Error,
    #[source_code]
    input: String,
    #[label("at this position")]
    location: SourceOffset,
}

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
#[error("")]
pub struct ClonedError {
    message: String,
    #[source_code]
    input: String,
    #[label("{message}")]
    location: SourceOffset,
}

impl DecodeError {
    pub fn from_serde(input: impl Into<String>, cause: serde_json::Error) -> Self {
        let input = input.into();
        let location = SourceOffset::from_location(&input, cause.line(), cause.column());
        DecodeError {
            cause,
            input,
            location,
        }
    }

    pub fn report(&self) -> String {
        use colored::Colorize;
        let report = miette::Report::from(ClonedError {
            message: self.cause.to_string().italic().to_string(),
            input: self.input.clone(),
            location: self.location,
        });

        let mut output = String::new();

        for line in format!("{report:?}").lines().skip(1) {
            use std::fmt::Write;
            writeln!(&mut output, "{line}").unwrap();
        }

        output.white().to_string()
    }
}
