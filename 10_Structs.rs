struct User {
    active: bool, // corrected field name to `active`
    username: String, // corrected field name to `username`
    email: String,
    sign_in_count: u64, // corrected field name to `sign_in_count`
}

fn main() {
    // Moved the user instantiation inside the main function
    let mut user1 = User {
        email: String::from("example@example.com"),
        username: String::from("example1234"), // added missing comma
        active: true,
        sign_in_count: 1,
    };
    
    user1.email = String::from("anotherexample@example.com");
    
    

    // Add the below lines at the end
    println!("Press Enter to exit...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
}