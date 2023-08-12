fn main() {
    
    // floats
    let result = 20.00;
    let interest:f32 = 9.0;
    let cost:f64 = 16000.600;
    
    let isfun:bool = true;
    
    println!("result value {}", result);
    println!("interest {}", interest);
    println!("cost {}", cost);
    
    // bool
    println!("Rust Programming Fun? {}", isfun);
    
    // char 
    let special_character = '@';
    let alphabet: char = 'D';
    
    println!("Special character {}", special_character);
    println!("Alphabet {}", alphabet);

    // Add the below lines at the end
    println!("Press Enter to exit...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
}
