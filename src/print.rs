

pub fn run() 
{   
    // print to console
    println!("Hello from the print.rs file");

    // Basic formatting
    println!("{} is number {}","Iman",1);

    // Positional Arguments
    println!("{0} is number {1} and {0} likes {2}", "Iman", 1, "Solana");

    // Named Arguments
    println!("{Name} likes to play {Activity}", Name = "John", Activity = "Code");

    // Placeholder traiits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10,10,10);

    // Placeholder for debug trait
    println!("{:?}",(12, true,"hello"));

    // Basic math
    println!("10 + 10 = {}", 10+10)
  
    
}