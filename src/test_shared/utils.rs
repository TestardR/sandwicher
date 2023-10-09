#[cfg(test)]
pub mod shared {
    use crate::internal::domain::sandwich::Sandwich;
    use crate::internal::domain::sandwich_type::SandwichType;

    pub const SANDWICH_ID: &str = "sand-id";
    pub const SANDWICH_NAME: &str = "Hot dog";
    pub const SANDWICH_TYPE: SandwichType = SandwichType::Meat;
    pub const CHEESEBURGER_NAME: &str = "Cheeseburger";

    /*
        Assertion helpers
    */
    pub fn assert_on_sandwich(expected: Sandwich, actual: &Sandwich, assert_on_id: bool) {
        if assert_on_id {
            assert_eq!(actual.id().value().as_ref().unwrap(), expected.id().value().as_ref().unwrap());
        }
        assert_eq!(actual.name().value(), expected.name().value());
        assert_on_ingredients(expected.ingredients().value(), actual.ingredients().value());
    }

    pub fn assert_on_ingredients(expected_ingredients: &Vec<String>, actual_ingredients: &Vec<String>) {
        assert_eq!(expected_ingredients.len(), actual_ingredients.len());

        for (i, ingredient) in expected_ingredients.iter().enumerate() {
            assert_eq!(ingredient, &actual_ingredients[i]);
        }
    }


    /*
        Stubbing helpers
    */
    pub fn stub_sandwich(with_id: bool) -> Sandwich {
        let sandwich_id = if with_id { SANDWICH_ID } else { "" };
        let sandwich_name = SANDWICH_NAME;

        let hot_dog = Sandwich::new(sandwich_id.to_string(),
                                    sandwich_name.to_string(),
                                    stub_ingredients(),
                                    SANDWICH_TYPE)
            .unwrap();

        hot_dog
    }

    pub fn stub_ingredients() -> Vec<String> {
        vec!["Wurst".to_string(), "Ketchup".to_string()]
    }

    /*
        Type conversion helpers
    */
    pub fn string_vec_to_vec_str(list: &Vec<String>) -> Vec<&str> {
        let mut pointer_list = Vec::<&str>::new();
        for item in list {
            pointer_list.push(item);
        }
        pointer_list
    }
}