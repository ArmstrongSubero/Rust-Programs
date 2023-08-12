enum Conveyance {
    Car,
    Train,
    Air,
}

impl Conveyance {
    fn travel_allowance(&self, miles: i32) -> f32 {
        let allowance: f32 = match self {
            Conveyance::Car => miles as f32 * 14.0 * 2.0,
            Conveyance::Train => miles as f32 * 18.0 * 2.0,
            Conveyance::Air => miles as f32 * 30.0 * 2.0,
        };
        
        return allowance;
    }
}

fn main() {
    // variance with type of car
    let participant_1: Conveyance = Conveyance::Car;
    let participant_2: Conveyance = Conveyance::Train;
    let participant_3: Conveyance = Conveyance::Air;

    //println!("The value of the option is {}", participant_1 as i32);
    
    println!("1 allowance of {}", participant_1.travel_allowance(60));
    
    println!("2 allowance of {}", participant_2.travel_allowance(60));
    
    println!("3 allowance of {}", 
participant_3.travel_allowance(60));

    println!("Press Enter to exit...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
}
