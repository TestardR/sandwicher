#[derive(Debug)]
pub struct AddSandwich {
    name:  String
}

impl AddSandwich {
    pub fn new(
        name: String,
    ) -> Self {
        Self { name }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
