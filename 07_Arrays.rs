fn main() {
    // array 
    let arra: [i32; 4] = [20, 30, 80, 50];
    println!("array {:?}", arra);
    println!("array size: {}", arra.len());
    
    // assign deault values 
    // assign -1 to all of the elements
    let arra: [i32;4] = [-1; 4];
    println!("array {:?}", arra);
    println!("array size: {}", arra.len());
    
    // mutable array 
    let mut arra: [i32; 4] = [20, 30, 80, 50];
    
    arra[1] = 0;
    println!("{:?}", arra);

    // Add the below lines at the end
    println!("Press Enter to exit...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
}
