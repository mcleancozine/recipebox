pub struct Ingredient {
    pub name: String,
    pub amount: Measurement,
    pub special_instructions: String,
}

pub struct Step {
    order: u8,
    pub description: String,
}

pub enum Measurement {
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