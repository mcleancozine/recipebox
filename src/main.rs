fn main() {
    println!("Hello, world!");
}

struct Ingredient {
    name: String,
    amount: f32,
    unit: String,
    special_instructions: String,
}

struct Step {
    order: u8,
    description: String,
}

struct Recipe {
    name: String,
    description: String,
    source: String,
    ingredients: Vec<Ingredient>,
    steps: Vec<Step>,
}
