use std::fmt;

#[derive(Debug, Clone)]
pub struct DomainViolation {
    violation: String
}

impl DomainViolation {
    pub fn new(violation: String) -> Self {
        Self { violation }
    }
}

impl fmt::Display for DomainViolation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.violation)
    }
}