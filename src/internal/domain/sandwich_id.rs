#[derive(Debug)]
pub struct SandwichId(Option<String>);

impl SandwichId {
    pub fn new(id: Option<String>) -> Self {
        Self(id)
    }
    pub fn value(&self) -> &Option<String> {
        &self.0
    }
}

impl TryFrom<String> for SandwichId {
    type Error = &'static str;

    fn try_from(id: String) -> Result<Self, Self::Error> {
        if id.is_empty() {
            Ok(Self(None))
        } else {
            Ok(Self(Some(id)))
        }
    }
}