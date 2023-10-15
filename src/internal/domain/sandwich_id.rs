#[derive(Debug, PartialEq)]
pub struct SandwichId(i64);

impl SandwichId {
    pub fn new(id: i64) -> Self {
        Self(id)
    }

    pub fn value(&self) -> &i64 { &self.0 }
}