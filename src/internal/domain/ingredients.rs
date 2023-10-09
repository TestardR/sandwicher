#[derive(Debug)]
pub struct SandwichIngredients(Vec<String>);

impl SandwichIngredients {
    pub fn value(&self) -> &Vec<String> {
        &self.0
    }
}

impl TryFrom<Vec<String>> for SandwichIngredients {
    type Error = &'static str;

    fn try_from(ingredients: Vec<String>) -> Result<Self, Self::Error> {
        if ingredients.is_empty() {
            Err("Any sandwich must have at least one ingredient")
        } else {
            Ok(Self(ingredients))
        }
    }
}