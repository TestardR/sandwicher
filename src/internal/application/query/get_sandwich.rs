use crate::internal::domain::sandwich::SandwichType;

pub struct GetSandwich<'a> {
    id: &'a str,
    name:  &'a str,
    ingredients: &'a Vec<&'a str>,
    sandwich_type: &'a SandwichType,
}

impl<'a> GetSandwich<'a> {
    pub fn new(
        id: &'a str,
        name: &'a str,
        ingredients: &'a Vec<&'a str>,
        sandwich_type: &'a SandwichType
    ) -> Self {
        Self { id, name, ingredients, sandwich_type }
    }

    pub fn id(&self) -> &'a str {
        self.id
    }

    pub fn name(&self) -> &'a str {
        self.name
    }

    pub fn ingredients(&self) -> &'a Vec<&'a str> {
        self.ingredients
    }

    pub fn sandwich_type(&self) -> &'a SandwichType {
        self.sandwich_type
    }
}
