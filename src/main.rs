use clap::{error::{Error, ErrorKind}, Parser};
use dialoguer::Input;
use log::{debug};
use recipe_box;

/// A simple program to take in and store the source and ingredients of a recipe.
#[derive(Parser)]
struct Args {
    /// Name of the recipe 
    #[arg(short, long, value_parser = Args::arg_not_empty)]
    name: Option<String>,

    /// Source of the recipe (cookbook name, website, etc.)
    #[arg(short, long, value_parser = Args::arg_not_empty)]
    source: Option<String>,
    /*
    /// List of ingredients in the recipe
    #[arg(short, long)]
    ingredients: Vec<String>,
    */
}

impl Args {
    fn arg_not_empty(arg_val: &str) -> Result<String, Error> {
        if arg_val.trim().len() == 0 {
            return Err(Error::new(ErrorKind::InvalidValue));
        } else {
            return Ok(arg_val.to_string());
        }
    }
}

fn main() {
    env_logger::init();
    debug!("Starting log");
    run();
}

fn run() {
    let args = Args::parse();
    let name = parse_name(args.name).expect("Failed to read name");
    let source = parse_source(args.source).expect("Failed to read source");
    recipe_box::create_recipe(&name[..], &source[..]);
}

fn parse_name(arg_name: Option<String>) -> Result<String, Error> {
    match arg_name {
        Some(name) => Ok(name),
        None => prompt_name()
    }
}

fn parse_source(arg_source: Option<String>) -> Result<String, Error> {
    match arg_source {
        Some(source) => Ok(source),
        None => prompt_source()
    }
}

fn prompt_name() -> Result<String, Error> {
    let input : String = Input::new()
        .with_prompt("Enter recipe name")
        .interact_text()?;
    Ok(input)
}

fn prompt_source() -> Result<String, Error> {
    let input : String = Input::new()
        .with_prompt("Enter recipe source")
        .interact_text()?;
    Ok(input)
}