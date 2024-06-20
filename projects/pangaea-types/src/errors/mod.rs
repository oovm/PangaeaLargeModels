use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
};

mod convert;
mod display;

/// The result type of this crate.
pub type Result<T> = std::result::Result<T, PangaeaError>;

/// A boxed error kind, wrapping an [ExampleErrorKind].
#[derive(Clone)]
pub struct PangaeaError {
    kind: Box<ExampleErrorKind>,
}

/// The kind of [PangaeaError].
#[derive(Clone, Debug)]
pub enum ExampleErrorKind {
    /// An unknown error.
    UnknownError,

    CustomError {
        message: String,
    },
}

impl PangaeaError {
    pub fn custom(message: impl Into<String>) -> Self {
        Self { kind: Box::new(ExampleErrorKind::CustomError { message: message.into() }) }
    }
}
