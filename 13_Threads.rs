use std::thread;
use std::time::Duration;


fn main() {
    let handle = thread::spawn(||{
        for x in 1..10{
            println!("Hello number {} from spawned thread!", x);
            thread::sleep(Duration::from_millis(1));
            }
    });
    
    for x in 1..5{
        println!("Hello number {} from main thread!", x);
        thread::sleep(Duration::from_millis(1));
    }
    
    handle.join().unwrap();
            


    println!("Press Enter to exit...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
}
