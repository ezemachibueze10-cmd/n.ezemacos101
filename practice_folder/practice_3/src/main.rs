fn main() {
    let p:f64 = 210_000.00; 
    let r:f64 = 5.00; 
    let n:f64 = 3.00; 

    let a = p * (1.00 - (r/100.0)).powf(n);
    println!("Amount is {}",a );
}
