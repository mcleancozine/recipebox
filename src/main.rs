use crate::recipe_components::{Ingredient, Step};

fn main() {
    println!("Hello, world!");
}

struct Recipe {
    name: String,
    description: String,
    source: String,
    ingredients: Vec<recipe_components::Ingredient>,
    steps: Vec<recipe_components::Step>,
}