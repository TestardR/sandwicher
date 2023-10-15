#[derive(Debug)]
pub struct GetSandwich {
    id: i64,
}

impl GetSandwich {
    pub fn new(
        id: i64,
    ) -> Self {
        Self { id }
    }

    pub fn id(&self) -> &i64 {
        &self.id
    }
}
