use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
fn main() {
    println!("Hello, world!");
    println!("{}",1+4);
    println!("{}",9*10);
    println!("{}",16/2);
    let path = Path::new("test.txt");
    let display = path.display();
    let message = "Hello, World!";
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };
    match file.write_all(message.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}
