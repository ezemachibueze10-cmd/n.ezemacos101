use std::io;

fn main() {
    println!("=================================");
    println!(" Compound Interest Calculator");
    println!("=================================");

    loop {
        println!("\nEnter principal amount (or q to quit):");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .unwrap();

        let input = input.trim();

        if input == "q" || input == "Q" {
            println!("Program ended.");
            break;
        }

        let p: f64 = match input.parse() {
            Ok(value) => value,
            Err(_) => {
                println!("Invalid principal amount.");
                continue;
            }
        };

        println!("Enter interest rate (%):");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .unwrap();

        let r: f64 = match input.trim().parse() {
            Ok(value) => value,
            Err(_) => {
                println!("Invalid interest rate.");
                continue;
            }
        };

        println!("Enter number of years:");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .unwrap();

        let t: f64 = match input.trim().parse() {
            Ok(value) => value,
            Err(_) => {
                println!("Invalid number of years.");
                continue;
            }
        };

        let a = p * (1.0 + r / 100.0).powf(t);

        let interest = a - p;

        // Display results
        println!("\n---------- RESULTS ----------");
        println!("Principal: ₦{:.2}", p);
        println!("Interest Rate: {:.2}%", r);
        println!("Time: {:.2} years", t);
        println!("Compound Interest: ₦{:.2}", interest);
        println!("Total Amount: ₦{:.2}", a);
        println!("-----------------------------");
    }
}