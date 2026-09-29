use std::io; 
fn main() {
    let mut input1 = String::new();
 
    println!("What is your height");
    io::stdin().read_line(&mut input1).expect("Invalid String");
    let height:f32 = input1.trim().parse().expect("Not a valid number");

    if height >= 150.0 && height <= 180.0{
        println!("You are medium height");
    } 
    else if height < 150.0{ 
        println!("You are short");
    } 
    else if height > 180.0{ 
        println!("You are tall");
    } 

}