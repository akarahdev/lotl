use lotl_error::diagnostic::DiagnosticError;

pub struct UnexpectedEOFWhileFinding(pub char);

impl DiagnosticError for UnexpectedEOFWhileFinding {
    fn message(self) -> String {
        format!("Unexpected EOF while trying to find terminating {:?}", self.0)
    }
}

pub struct InvalidCharacter(pub char);
impl DiagnosticError for InvalidCharacter {
    fn message(self) -> String {
        format!("{:?} is not a valid character", self.0)
    }
}
