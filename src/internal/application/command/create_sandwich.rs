use crate::internal::domain::sandwich_type::SandwichType;

pub struct CreateSandwich<'a> {
    name:  &'a str,
    ingredients: &'a Vec<&'a str>,
    sandwich_type: &'a SandwichType,
}

impl<'a> CreateSandwich<'a> {
    pub fn new(
        name: &'a str,
        ingredients: &'a Vec<&'a str>,
        sandwich_type: &'a SandwichType
    ) -> Self {
        Self { name, ingredients, sandwich_type }
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
