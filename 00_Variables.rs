fn main() {
    
    let fees = 35_000;
    let salary: f64 = 45_000.00;
    
    println!("Fees {} and salary is {}", fees, salary);

    // Add the below lines at the end
    println!("Press Enter to exit...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
}
