fn main() {
    // Principal
    let p: f64 = 520_000_000.0;

    // Interest rate
    let r: f64 = 10.0;

    // Number of years
    let n: u32 = 5;

    // Compound interest formula
    let a = p * (1.0 + r / 100.0).powi(n as i32);

    // Compound Interest
    let ci = a - p;

    println!("Principal: ₦{:.2}", p);
    println!("Rate: {}%", r);
    println!("Time: {} years", n);
    println!("Amount: ₦{:.2}", a);
    println!("Compound Interest: ₦{:.2}", ci);
}