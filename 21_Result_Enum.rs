use std::num::ParseIntError;

fn multiply_numbers(a: &str, b: &str) -> Result<i32, ParseIntError> {
    let a = a.parse::<i32>()?;
    let b = b.parse::<i32>()?;
    Ok(a * b)
}


fn main() {
    match multiply_numbers("2", "3") {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    println!("Press Enter to exit...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
}
