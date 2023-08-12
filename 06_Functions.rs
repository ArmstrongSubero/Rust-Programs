// function no return value
fn fn_hello() {
    println!("Hello fn_hello");
}

// fucntion with return
// fn function_name() -> return_type
fn get_pi() -> f64 {
    return 22.0 / 7.0;
}

// Pass by value
/*When you pass data to a function in Rust, you're often passing it by value. This means that the function receives a copy of the data, and modifications made to the data inside the function do not affect the original value outside of the function.
fn mutate*/
fn modify_value(mut x: i32) {
    x += 1;
    println!("Inside function Value: {}", x);
}

// Pass by reference
/*Rust introduces the concept of "borrowing", which is akin to passing by reference in other languages. When you pass data by borrowing it, the function gets a reference to the original data and doesn't own it. As a result, you're not passing the actual data but a reference to it.*/

fn modify_reference(x: &mut i32) {
    *x += 1;
    println!("Inside function Reference: {}", x);
}

fn main() {
    fn_hello();

    println!("pi value {}", get_pi());

    let a = 5;
    modify_value(a);
    println!("Outside function Value: {}", a);

    let mut a = 5;
    modify_reference(&mut a);
    println!("Outside function Reference: {}", a);

    // Add the below lines at the end
    println!("Press Enter to exit...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
}
