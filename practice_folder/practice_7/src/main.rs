use std::io; 
fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Please enter your name: ");
    io::stdin().read_line(&mut input1).expect("Invalid String");

    println!("Please enter your age: ");  
    io::stdin().read_line(&mut input2).expect("Invalid String");
    let b:i32 = input2.trim().parse().expect("Not a valid number");

    if b < 18{
        println!("Do not let inside the party");
    }
    else if b >= 18{
        println!("Let inside the party"); 
    }
}