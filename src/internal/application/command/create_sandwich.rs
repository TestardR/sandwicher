#[derive(Debug)]
pub struct CreateSandwich {
    name:  String
}

impl CreateSandwich {
    pub fn new(
        name: String,
    ) -> Self {
        Self { name }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
