use crate::recipe_components::{Ingredient, Step};

fn main() {
    println!("Hello, world!");
}

#[derive(Debug)]
struct Recipe {
    name: String,
    description: String,
    source: String,
    ingredients: Vec<recipe_components::Ingredient>,
    steps: Vec<recipe_components::Step>,
}

impl Recipe {
    fn new(name: &str) -> Self {
        Self {
            name,
            description = String::new(),
            source = String::new(),
            ingredients = Vec::new(),
            steps = Vec::new(),
        }
    }
    
    fn add_step(&mut self, recipe_components::Step step) {
        self.steps.push(step);
    }

    fn get_step(&self, i32 step_index) -> Option<Step> {
        self.steps.get(step_index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_recipe() {
        let recipe = Recipe::new("test_recipe");
    }
}