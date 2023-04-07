use clap::Parser;
use log::{info, trace};
use recipe_box;

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
    run();
}

fn run() {
    trace!("Entered fn run()");
    let args = Args::parse();
    recipe_box::create_recipe(&args.name, &args.source);
}