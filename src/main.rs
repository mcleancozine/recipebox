struct Cli {
    name: String,
    source: String,
    ingredients: Vec<String>,
}

fn main() {
    let name = std::env::args().nth(1).expect("no name given");
    let source = std::env::args().nth(2).expect("no source given");
    let ingredients = std::env::args().next_chunk().expect("no ingredients given");
}