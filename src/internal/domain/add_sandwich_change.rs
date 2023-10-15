use crate::internal::domain::name::SandwichName;

pub struct AddSandwichChange {
    name: SandwichName,
}

impl AddSandwichChange {
    pub fn name(&self) -> &SandwichName {
        &self.name
    }
}

pub fn new_add_sandwich_change(
    name: SandwichName
) -> AddSandwichChange {
    AddSandwichChange{name}
}