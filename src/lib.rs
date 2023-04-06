struct Recipe {
    name: String,
    source: String,
    ingredients: Vec<String>,
}

impl Recipe {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            source: String::new(),
            ingredients: Vec::new(),
        }
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