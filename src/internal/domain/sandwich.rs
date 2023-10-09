use crate::internal::domain::ingredients::SandwichIngredients;
use crate::internal::domain::name::SandwichName;
use crate::internal::domain::sandwich_id::SandwichId;
use crate::internal::domain::sandwich_type::SandwichType;


#[derive(Debug)]
pub struct Sandwich {
    id: SandwichId,
    name: SandwichName,
    ingredients: SandwichIngredients,
    sandwich_type: SandwichType,
}

impl Sandwich {
    pub fn new(id: String, name: String, ingredients: Vec<String>, sandwich_type: SandwichType) -> Result<Sandwich, String> {
        let sandwich_id = SandwichId::try_from(id)?;
        let sandwich_name = SandwichName::try_from(name)?;
        let sandwich_ingrs = SandwichIngredients::try_from(ingredients)?;

        Ok(Self {
            id: sandwich_id,
            name: sandwich_name,
            ingredients: sandwich_ingrs,
            sandwich_type,
        })
    }

    pub fn id(&self) -> &SandwichId {
        &self.id
    }

    pub fn name(&self) -> &SandwichName {
        &self.name
    }

    pub fn ingredients(&self) -> &SandwichIngredients {
        &self.ingredients
    }

    pub fn sandwich_type(&self) -> &SandwichType {
        &self.sandwich_type
    }
}

mod tests {
    use super::*;

    const SANDWICH_ID: &str = "sand-id";
    const SANDWICH_NAME: &str = "Hot dog";

    #[test]
    fn should_create_the_expected_sandwich() {
        let ingredients = vec!["Wurst".to_string(), "Ketchup".to_string()];

        let hot_dog = Sandwich::new(SANDWICH_ID.to_string(),
                                    SANDWICH_NAME.to_string(),
                                    ingredients.clone(),
                                    SandwichType::Meat)
            .unwrap();

        assert_eq!(hot_dog.id().value().as_ref().unwrap(), SANDWICH_ID);
        assert_eq!(hot_dog.name.value(), SANDWICH_NAME);

        assert_eq!(ingredients.len(), hot_dog.ingredients.value().len());

        for (i, ingredient) in ingredients.iter().enumerate() {
            assert_eq!(ingredient, &hot_dog.ingredients.value()[i]);
        }
    }

    #[test]
    fn should_fail_without_a_name_or_ingredients() {

        // without name
        let err_sandwich = Sandwich::new("".to_string(),
                                         "".to_string(),
                                         vec!["Wurst".to_string(), "Ketchup".to_string()],
                                         SandwichType::Meat);

        assert_eq!(err_sandwich.is_err(), true);
        assert_eq!(err_sandwich.unwrap_err(), "Any sandwich must have a name");

        // without at least 1 ingredient
        let err_sandwich = Sandwich::new(SANDWICH_ID.to_string(),
                                         SANDWICH_NAME.to_string(),
                                         vec![],
                                         SandwichType::Meat);

        assert_eq!(err_sandwich.is_err(), true);
        assert_eq!(err_sandwich.unwrap_err(), "Any sandwich must have at least one ingredient");
    }
}