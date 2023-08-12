fn main() {
    // tuple
    let tup = (600, 7.4, 2);

    // destructure the tuple
    let (a, b, c) = tup;

    println!("The value of b is: {}", b);

    // another tuple
    let x: (i32, f64, u8) = (600, 7.4, 1);
    let six_hundred = x.0;
    let seven_point_four = x.1;
    let one = x.2;
    
    println!("one is {}", one);

    // array when you want data on stack rather than heap
    let x = [11, 22, 33, 44, 55];
    let first = x[0];
    let second = x[1];

    println!("first {} second {}", first, second);
    
    // constansts 
    const USER_LIMIT:i32 = 110;
    const PI:f32=3.14;
    println!("user limit {}", USER_LIMIT);
    println!("pi value {}", PI);
    
    // string literal
    let company: &str="RustPoint";
    let location: &str="Ludhiana";
    println!("Company: {} location: {}", company, location);

    // Add the below lines at the end
    println!("Press Enter to exit...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
}
