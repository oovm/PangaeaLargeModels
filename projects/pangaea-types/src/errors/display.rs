use super::*;

impl Error for PangaeaError {}

impl Debug for PangaeaError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.kind, f)
    }
}

impl Display for PangaeaError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.kind, f)
    }
}

impl Display for ExampleErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ExampleErrorKind::UnknownError => {
                write!(f, "UnknownError")
            }
            ExampleErrorKind::CustomError { message } => {
                write!(f, "CustomError: {}", message)
            }
        }
    }
}
