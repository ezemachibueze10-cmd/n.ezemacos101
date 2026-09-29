use std::io; 
fn main() {
loop{
    let mut p = String::new();
    let mut r = String::new();
    let mut t = String::new(); 


    println!("\nInsert principle or q to quit ");
    io::stdin().read_line(&mut p).expect("Invalid String");

    if p.trim() == "q"{
        println!("Program ended.");
        break; 
    } 

    let p:f32 = p.trim().parse().expect("Not a valid number"); 

    println!("Insert rate");
    io::stdin().read_line(&mut r).expect("Invalid String");
    let r:f32 = r.trim().parse().expect("Not a valid number");

    println!("Insert time");
    io::stdin().read_line(&mut t).expect("Invalid String");
    let t:f32 = t.trim().parse().expect("Not a valid number"); 

    let a = p * (1.00 + r/100.00).powf(t);
    println!("Amount is {}",a );
    let ci = a - p ; 
    println!("Compund interest is {}",ci ); 

}

}