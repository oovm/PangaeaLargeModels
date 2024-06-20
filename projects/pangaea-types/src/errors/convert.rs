use super::*;

impl From<ExampleErrorKind> for PangaeaError {
    fn from(value: ExampleErrorKind) -> Self {
        Self { kind: Box::new(value) }
    }
}

impl From<mongodb::error::Error> for PangaeaError {
    fn from(value: mongodb::error::Error) -> Self {
        Self { kind: Box::new(ExampleErrorKind::CustomError { message: value.to_string() }) }
    }
}
