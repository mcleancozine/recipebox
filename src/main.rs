fn main() {
    println!("Hello, world!");
}

struct Ingredient {
    name: String,
    amount: Measurement,
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

enum Measurement {
    tspn(f32),
    tbsp(f32),
    cup(f32),
    quart(f32),
    pint(f32),
    gallon(f32),
    gram(u8),
    oz(f32),
    floz(f32),
    ml(u8),
    liter(f32),
}
