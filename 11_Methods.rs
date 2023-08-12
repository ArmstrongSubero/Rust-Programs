#[derive(Debug)]
struct Rectangle { // Change the name to Rectangle
    width: u32,
    height: u32,
}

// implementation block
impl Rectangle { 
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle { // Change the name to Rectangle
        width: 40,
        height: 60,
    };
    
    println!(
        "Area of the rectangle {} square pixels.",
        rect1.area()
    );

    // Add the below lines at the end
    println!("Press Enter to exit...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
}
