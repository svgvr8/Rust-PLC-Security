use std::io;

fn main() {
    let password = 1234;
    
    println!("Enter 4 digit password to unlock: ");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let input: u32 = input.trim().parse().expect("Please enter a valid number");
    
    if input == password {
        println!("Access granted!");
        // Code to unlock the door
    } else {
        println!("Access denied!");
        // Code to trigger an alarm or other security measure
    }
}
