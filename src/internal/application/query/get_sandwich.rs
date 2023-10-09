pub struct GetSandwich<'a> {
    id: &'a str,
}

impl<'a> GetSandwich<'a> {
    pub fn new(
        id: &'a str,
    ) -> Self {
        Self { id }
    }

    pub fn id(&self) -> &'a str {
        self.id
    }
}
