use ferris_says::say;
use std::io::{stdout, BufWriter};
fn main() {
    println!("Hello, world!");
    println!("{}",1+4);
    println!("{}",9*10);
    println!("{}",16/2);
    let stdout = stdout();
    let message = String::from("hi chat");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer).unwrap();
}
