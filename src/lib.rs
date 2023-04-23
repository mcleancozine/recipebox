use log::{info};

pub struct Recipe {
    name: String,
    source: String,
    ingredients: Vec<String>,
}

impl Recipe {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            source: String::new(),
            ingredients: Vec::new(),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn set_source(&mut self, source: &str) {
        self.source = source.to_string();
    }

    pub fn get_source(&self) -> &str {
        &self.source
    }

    pub fn add_ingredient(&mut self, ingredient: &str) {
        self.ingredients.push(ingredient.to_string());
    }

    pub fn get_ingredients(&self) -> Vec<&str> {
        let mut result = Vec::new();
        for ingredient in &self.ingredients {
            result.push(&ingredient[..]);
        }
        result
    }
}

pub fn create_recipe(name: &str, source: &str) {
    let mut recipe = Recipe::new(name);
    recipe.set_source(source);
    info!("Created recipe with name {} and source {}", name, source);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_recipe() {
        let recipe = Recipe::new("test_recipe");
        assert_eq!(recipe.get_name(), "test_recipe");
    }

    #[test]
    fn adding_source() {
        let mut recipe = Recipe::new("test_recipe");
        recipe.set_source("test_source");
        assert_eq!(recipe.get_source(), "test_source");
    }

    #[test]
    fn adding_ingredients() {
        let mut recipe = Recipe::new("test_recipe");
        recipe.add_ingredient("salt");
        let mut expected_result = Vec::new();
        expected_result.push("salt");
        assert_eq!(recipe.get_ingredients(), expected_result);
        recipe.add_ingredient("pepper");
        expected_result.push("pepper");
        assert_eq!(recipe.get_ingredients(), expected_result);
    }
}