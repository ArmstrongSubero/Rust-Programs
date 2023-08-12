// restrict genetic to any types that includes the 
// trait of multiplication and copy
fn square<T> (x: T) -> T
where T: std::ops::Mul<Output = T> + Copy{
    return x*x;
}

fn main() {
    println!("The square of the number is {}", square(5));
    
    println!("The square of the number is {}",
    square(5.5));
    println!("Press Enter to exit...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
}
