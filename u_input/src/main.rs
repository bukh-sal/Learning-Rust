use std::io::{self, Write};

fn main() {
    let mut u_name = String::new();
    print!("Name: ");
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut u_name).expect("Error in reading input");

    println!("");
    println!("Hello {}", u_name);
}
