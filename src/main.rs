use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead};
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
    if let Ok(lines) = read_lines("./test.txt") {
        for line in lines.flatten() {
            println!("{}", line);
        }
    }
}
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
