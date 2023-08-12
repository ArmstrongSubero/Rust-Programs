fn main() {
    // if statement
    let num: i32 = 9;

    if num > 0 {
        println!("number positive");
    }

    // if else
    let num = 15;
    if num % 2 == 0 {
        println!("Evennum");
    } else {
        println!("Oddnum");
    }
    
    // nested if 
    let numb = 4;
    
    if numb > 0{
        println!("{} positive", numb);
    }else if numb < 0{
        println!("{} negative", numb);
    }else{
        println!("{} neither positive or negative", numb);
    }
    
    // match statement 
    let state_code = "DL";
    let state = match state_code{
        "DL" => {println!("Found match"); "Delhi"},
        "IN" => "India",
        "KL" => "Kolkatta",
        "AB" => "Ahmdabad",
        _=> "Unknown"
    };
    
    println!("State name: {}", state);
    
    // if with let 
    let x = if true{
        2
    }else{
        4
    };
    
    println!("value of x is: {}", x);
    

    // Add the below lines at the end
    println!("Press Enter to exit...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
}
