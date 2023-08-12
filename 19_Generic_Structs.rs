struct Point<T, U>{
    x: T,
    y: U,
}

impl <T: std::fmt::Debug, U: std::fmt::Debug> Point <T, U>{
    fn printing(&self){
        println!("The value of the point coordiantes are {:?}, {:?}", self.x, self.y);
        
    }
}

fn main() {
    let p1: Point<i32, i32> = Point {x: 5, y: 5};
    let p2: Point<f64, f64> = Point {x: 1.0, y: 4.0};
    let p3: Point<i32, f64> = Point {x: 5, y: 5.0};
    
    
    p1.printing();
    p2.printing();
    p3.printing();
    
    println!("Press Enter to exit...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
}
