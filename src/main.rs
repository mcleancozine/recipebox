use clap::Parser;
use log::{debug};
use recipe_box;
use std::io;

/// A simple program to take in and store the source and ingredients of a recipe.
#[derive(Parser)]
struct Args {
    /// Name of the recipe 
    #[arg(short, long)]
    name: Option<String>,

    /// Source of the recipe (cookbook name, website, etc.)
    #[arg(short, long)]
    source: Option<String>,
    /*
    /// List of ingredients in the recipe
    #[arg(short, long)]
    ingredients: Vec<String>,
    */
}

fn main() {
    env_logger::init();
    debug!("Starting log");
    run();
}

fn run() {
    let args = Args::parse();
    let name = parse_name(args.name);
    let source = parse_source(args.source);
    recipe_box::create_recipe(&name[..], &source[..]);
}

fn parse_name(arg_name: Option<String>) -> String {
    match arg_name {
        Some(name) => name,
        None => prompt_name()
    }
}

fn parse_source(arg_source: Option<String>) -> String {
    match arg_source {
        Some(source) => source,
        None => prompt_source()
    }
}

fn prompt_name() -> String {
    println!("Enter recipe name:");

    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read name");

    name
}

fn prompt_source() -> String {
    println!("Enter recipe source:");

    let mut source = String::new();

    io::stdin()
        .read_line(&mut source)
        .expect("Failed to read source");

    source
}