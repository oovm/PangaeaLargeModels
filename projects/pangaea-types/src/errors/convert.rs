use super::*;
use mongodb::error::{ErrorKind, WriteFailure};

impl From<ExampleErrorKind> for PangaeaError {
    fn from(value: ExampleErrorKind) -> Self {
        Self { kind: Box::new(value) }
    }
}

impl From<mongodb::error::Error> for PangaeaError {
    fn from(value: mongodb::error::Error) -> Self {
        match value.kind.as_ref() {
            // ErrorKind::Write(WriteFailure::WriteConcernError(e)) => {
            //
            // }
            ErrorKind::Write(WriteFailure::WriteError(e)) => {
                tracing::error!("Database Write Error: {}", e.message);
                Self { kind: Box::new(ExampleErrorKind::WriteError { internal_id: 0 }) }
            }
            _ => Self { kind: Box::new(ExampleErrorKind::CustomError { message: value.to_string() }) },
        }
    }
}


