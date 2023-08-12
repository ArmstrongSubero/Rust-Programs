fn main() {
    let my_information:(&str, i32) = ("Salary", 40_000);
    
    // destructure tuple
    let (salary, salary_value) = my_information;
    
    let salary: &str = my_information.0;
    let salary_value: i32 = my_information.1;
    
    // nested tuple 
    let nested_tuple: (i32, f64, (i32, i32), &str) =
    (4, 5.0, (3, 2), "Hello");
    
    // 3
    let element = nested_tuple.2.0;
    
    println!("{}", element);
    
    // empty tuple 
    let empty_tuple: () = ();
    
    println!("Press Enter to exit...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
}
