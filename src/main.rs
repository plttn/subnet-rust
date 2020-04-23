use std::io;

fn main() {
    println!("Enter an IP address/mask in the format 192.168.1.1/24: ");

    let mut input = String::new();
    
    io::stdin().read_line(&mut input).expect("failed to read line");
    println!("{}", input);
}
