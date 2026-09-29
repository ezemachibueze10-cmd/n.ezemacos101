use std::io;
 fn main() {
    let mut side1 = String::new();
    let mut side2 = String::new();
    let mut side3 = String::new(); 

    println!("Enter first edge of triangle: "); 
    io::stdin().read_line(&mut side1).expect("Not a valid String");
    let a:f32 = side1.trim().parse().expect("Not a valid number");

    println!("Enter second edge of triangle: ");
    io::stdin().read_line(&mut side2).expect("Not a valid string");
    let b:f32 = side2.trim().parse().expect("Not a valid number"); 

    println!("Enter third egde of triangle: "); 
    io::stdin().read_line(&mut side3).expect("Not a valid string");
    let c:f32 = side3.trim().parse().expect("Not a valid number");

    let s:f32 = (a + b + c)/2.0;
    let mut area:f32 = s * (s - a) * (s - b) * (s - c);
    area = area.sqrt(); 
    println!("Area of triangle is {}", area);
}