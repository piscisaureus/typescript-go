mod ast;
mod parser;
mod scanner;
mod types;
mod error;

use std::fs;
use std::path::Path;

fn main() {
    let demo_path = Path::new("../demo.ts");
    let source = fs::read_to_string(demo_path).expect("Could not read demo.ts");
    
    println!("Source code:");
    println!("{}", source);
    
    let result = parse_and_check(&source);
    match result {
        Ok(_) => println!("Successfully parsed and type-checked demo.ts"),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn parse_and_check(source: &str) -> Result<(), String> {
    // Create scanner
    let scanner = scanner::Scanner::new(source);
    
    // Create parser and parse
    let mut parser = parser::Parser::new(scanner).map_err(|e| e.to_string())?;
    let ast = parser.parse().map_err(|e| e.to_string())?;
    
    // Type check
    let checker = types::TypeChecker::new(ast);
    checker.check().map_err(|e| e.to_string())?;
    
    Ok(())
}
