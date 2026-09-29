use std::io;
fn main() {
    let mut base = String::new();
    let mut height = String::new(); 

    println!("Enter base of the triangle: ");
    io::stdin().read_line(&mut base).expect("Invalid String"); 
    let a:f32 = base.trim().parse().expect("Not a valid number"); 

    println!("Enter height of the triangle: "); 
    io::stdin().read_line(&mut height).expect("Invalid String");
    let b:f32 = height.trim().parse().expect("Not a valid number");

    if b > 0.0 { 
        let area = (a * b)/2.0; 
    println!("The area is {}",area );
    }

}