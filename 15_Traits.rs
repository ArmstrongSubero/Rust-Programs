// a trait tells the rust compiler about a particular 
// functionality a type has 
struct Person{
    citizenship: String,
    name: String,
    age: u8,
    gender: char,
    salary: i32,
}

struct Student{
    name_std: String,
    age: u8,
    sex: char, 
    country: String,
}

trait GeneralInfo{
    fn info(&self) -> (&str, u8, char);
    
    fn country_info(&self) -> &str;
}

impl GeneralInfo for Person{
    fn info(&self) -> (&str, u8, char){
        (&self.name, self.age, self.gender)
    }
    
    fn country_info(&self) -> &str{
        &self.citizenship
    }
}


impl GeneralInfo for Student {
    fn info(&self) -> (&str, u8, char) {
        (&self.name_std, self.age, self.sex)
    }
    
    fn country_info(&self) -> &str {
        &self.country
    }
}


// generic fucntion with trait bounds 
// generic fucntions that work on any type that 
// implements a specific trait 
fn display_info<T: GeneralInfo>(entity: &T) {
    let (name, age, gender) = entity.info();
    println!("Name: {}, Age: {}, Gender: {}", name, age, gender);
}

fn main() {
    // using trait methods 
    let person = Person{
        citizenship: String::from("USA"),
        name: String::from("John"),
        age: 30,
        gender: 'M',
        salary: 50000,
    };
    
    let (name, age, gender) = person.info();
    let country = person.country_info();
    
    println!("{:?}", person.info());
    
    
    display_info(&person);

    
    //'dyn' with traits 
    let student = Student{
        name_std: String::from("Bob"),
        age: 20,
        sex: 'M',
        country: String::from("England"),
    };
    
    let entities: Vec<&dyn GeneralInfo> = vec![&person, &student];
    
    for entity in entities {
        println!("Country: {}", entity.country_info());
    }
    
    println!("Press Enter to exit...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
}
