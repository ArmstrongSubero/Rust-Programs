fn main() {
    // vector
    let mut number_vec: Vec<i32> = vec![4, 5, 7, 8, 9];
    
    // single vector element 
    println!("{}", number_vec[0]);
    
    // entire vector
    println!("{:?}", number_vec);
    
    // update value of vector 
    number_vec[4] = 5;
    
    // initialize with same elements
    let array_with_same_elements: Vec<i32> = vec![0;10];
    println!("{:?}", array_with_same_elements);
    
    // slicing 
    let subset_vec: &&[i32] = &&number_vec[0..3];
    println!("The subset of values of the array are {:?}", subset_vec);
    
    // length 
    println!("Length of vector {}", number_vec.len());
    
    // get at index 
    let check_index: Option<&i32> = number_vec.get(100);
    println!("{:?}", check_index);
    
    // add to vector 
    number_vec.push(30);
    number_vec.push(40);
    println!("{:?}", number_vec);
    
    // remove at index
    number_vec.remove(5);
    println!("{:?}", number_vec);
    
    // see it is contains 
    let contains = number_vec.contains(&10);
    println!("{}", contains);
    
    
    
    
    println!("Press Enter to exit...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
}
