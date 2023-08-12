fn find_name(id: i32) -> Option<String> {
    if id == 1 {
        Some("Alice".to_string())
    } else {
        None
    }
}

fn main() {
    let name = find_name(1);
match name {
    Some(n) => println!("Found: {}", n),
    None => println!("Name not found"),
}
    
    
    
    println!("Press Enter to exit...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
}
