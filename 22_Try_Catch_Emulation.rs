fn some_function() -> Result<(), &'static str> {
    Err("Some error occurred!")
}


fn main() {
    let result = some_function();

    match result {
        Ok(_) => println!("All good!"),
        Err(e) => {
            println!("Handling error: {}", e);
            // Handle error, maybe by logging, retrying, etc.
        }
    }
    
    println!("Press Enter to exit...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
}
