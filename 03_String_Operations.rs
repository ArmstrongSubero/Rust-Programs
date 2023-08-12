fn main() {
    // utilize string object
    let empty_string = String::new();
    println!("length {}", empty_string.len());
    let content_string = String::from("RustPoint");
    println!("length{}", content_string.len());

    // you can create string
    let s = String::from("Hello, world!");
    println!("{}", s);
    
    // new creates a new string
    let mut a = String::new();
    a.push_str("helloo");
    println!("{}", a);
    
    // convert string literal to object type 
    let names = "Hello RustPoint, Hello!".to_string();
    println!("{}", names);
    
    // replace 
    let names1 = "Hello RustPoint, Hello".to_string();
    let names2 = names1.replace("Hello", "Rowdy");
    println!("{}", names2);
    
    // add specified character to end of the string 
    let mut company1 = "Rust".to_string();
    company1.push('s');
    println!("{}", company1);
    
    //append a specified string slice to end of string 
    let mut company1 = "Rust".to_string();
    company1.push_str("Point");
    println!("{}", company1);
    
    // len() total number of chars 
    // trim() eliminate leading and trailing whitespace
	
	// string slice
	let st = String::from(“hello world”);
	let hello = &st[0..5];
	let world = &st[6..11];
	
	// remove trailing number 
	let st = String::from(“hello”);
	let len = st .l en();
	let slice = &st[3. .l en];
	// same
	let slice = &st[3..];
    
    

    // Add the below lines at the end
    println!("Press Enter to exit...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
}
