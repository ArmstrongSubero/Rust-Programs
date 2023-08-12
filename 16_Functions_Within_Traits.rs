// fucntions within a trait may use each other

struct Data {
    some_data: Vec<i32>,
}

trait BasicStats {
    fn mean(&self) -> f32;
    fn variance(&self) -> f32;
}

impl BasicStats for Data {
    fn mean(&self) -> f32 {
        if self.some_data.is_empty() {
            return 0.0; // or another suitable value or handle this case differently
        }

        let sum: i32 = self.some_data.iter().sum();
        sum as f32 / self.some_data.len() as f32
    }

    fn variance(&self) -> f32 {
        let mu = self.mean();
        let sum_squared_diff: f32 = self.some_data.iter().map(|&i| {
            let diff = i as f32 - mu;
            diff * diff
        }).sum();

        sum_squared_diff / self.some_data.len() as f32
    }
}

fn main() {
    let data = Data { some_data: vec![1, 2, 3, 4, 5] };
    println!("Mean: {}", data.mean());
    println!("Variance: {}", data.variance());

    println!("Press Enter to exit...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
}