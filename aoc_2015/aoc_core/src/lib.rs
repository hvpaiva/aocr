use anyhow::Result;
use clipboard::{ClipboardContext, ClipboardProvider};
use colored::*;
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Mutex;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Part {
    One,
    Two,
}

type SolutionFn = fn(&str) -> Result<i64>;
type FunctionRegistry = HashMap<String, SolutionFn>;

lazy_static::lazy_static! {
    static ref FUNCTION_REGISTRY: Mutex<FunctionRegistry> = Mutex::new(HashMap::new());
}

pub fn register_function(part: &str, func: fn(&str) -> Result<i64>) {
    FUNCTION_REGISTRY
        .lock()
        .unwrap()
        .insert(part.to_string(), func);
}

pub fn run_day(part: &str, input_path: &str) -> Result<i64> {
    let input = std::fs::read_to_string(input_path).expect("Failed to read input file");

    let registry = FUNCTION_REGISTRY.lock().unwrap();
    let func = registry
        .get(part)
        .expect("Function not found for the specified part");

    func(&input)
}

pub fn run(input_path: &str) -> Result<()> {
    let part = std::env::args()
        .nth(1)
        .expect("Expected 'one' or 'two' as argument");

    println!("Running part {} solution:\n", part.cyan().bold());
    let output = run_day(&part, input_path)?;

    println!(
        "{} {}",
        "Answer:".italic(),
        format!("{output}").green().bold()
    );

    let mut ctx: ClipboardContext =
        ClipboardProvider::new().expect("Failed to initialize clipboard");
    ctx.set_contents(output.to_string())
        .expect("Failed to copy to clipboard");

    println!("{}", "\nOutput copied to clipboard.".blue().italic());

    Ok(())
}
