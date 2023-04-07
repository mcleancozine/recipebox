use clap::Parser;
use log::{info, debug};
use recipe_box::Recipe;

/// Simple program to take in and store a recipe with its source and ingredients
#[derive(Parser)]
struct Args {
    /// Name of the recipe 
    #[arg(short, long)]
    name: String,

    /// Source of the recipe (cookbook name, website, etc.)
    #[arg(short, long)]
    source: String,
    /*
    /// List of ingredients in the recipe
    #[arg(short, long)]
    ingredients: Vec<String>,
    */
}

fn main() {
    env_logger::init();
    info!("Starting log");
    let args = Args::parse();
    let mut recipe = Recipe::new(&args.name);
    recipe.add_source(&args.source);
    debug!("Provided recipe name: {}", args.name);
    debug!("Provided recipe source: {}", recipe.get_source());
}