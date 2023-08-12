use std::collections::HashMap;

fn main() {
    let mut person: HashMap<&str, i32> = HashMap::new();
    
    person.insert("Nouman", 40);
    person.insert("Shahid", 55);
    
    // get it gracefully 
    match person.get("Nouman"){
        Some(age) => println!("Age: {}", age),
        None => println!("No age found for Nouman"),
        
    }
    
    
    println!("Press Enter to exit...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
}
