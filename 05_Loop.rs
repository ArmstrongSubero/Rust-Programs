fn main() {
    // infinite loop
    //loop {
    //    println!("Hello everyone");
    //}
    
    // while loop
    // break keyword 
    let mut x = 1; 
    
    loop{
        println!("Hello everyone");
        if x == 7
        {
            break;
        }
        x += 1;
    }
    
    // for loop 
    for x in 1..12
    {
        print!("{}\n", x);
    }
    
    let fruits = ["apple", "orange", "mango", "banana",
        "watermelon"];
    
    for s in fruits.iter()
    {
        print!("{} ", s);
    }

    // Add the below lines at the end
    println!("Press Enter to exit...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
}
