use std::sync::mpsc;
use std::thread;
use std::time::Duration;


fn main() {
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move ||{
        let vals = vec![
            String::from("hello"),
            String::from("everyone"),
            String::from("from"),
            String::from("thread"),
        ];
        
        for val in vals{
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    
    for receive in rx{
        println!("Got: {}", receive);
    }

    println!("Press Enter to exit...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
}
