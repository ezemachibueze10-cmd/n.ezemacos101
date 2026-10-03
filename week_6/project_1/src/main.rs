use std::io;

fn main() {
     println!("         RESTAURANT MENU         ");
    println!("P - Poundo Yam / Edikaiko Soup - ₦3200");
    println!("F - Fried Rice & Chicken       - ₦3000");
    println!("A - Amala & Ewedu Soup         - ₦2500");
    println!("E - Eba & Egusi Soup           - ₦2000");
    println!("W - White Rice & Stew           - ₦2500");
    println!("                                  "); 

    let mut food = String::new();
    println!("Enter you food type (P, F, A, E, W)");
    io::stdin()
        .read_line(&mut food)
        .expect("Failed to read input");

    let food = food.trim().to_uppercase();

    let mut quantity = String::new();
    println!("Enter quantity:");
    io::stdin()
        .read_line(&mut quantity)
        .expect("Failed to read input");

    let quantity: f64 = quantity 
        .trim()
        .parse()
        .expect("Please enter a valid number");

    let price: f64 = match food.as_str() { 
     "P" => 3200.0,
        "F" => 3000.0,
        "A" => 2500.0,
        "E" => 2000.0,
        "W" => 2500.0,
        _ => {
            println!("Invalid food type!");
            return;
        }
    };
    let total = price * quantity; 
     println!("Total before discount: ₦{:.2}", total); 
       if total > 10000.0 {
        let discount = total * 0.05;
        let final_total = total - discount;

        println!("Discount: ₦{:.2}", discount);
        println!("Final amount: ₦{:.2}", final_total);
    } else {
        println!("No discount applied.");
        println!("Final amount: ₦{:.2}", total);
    }
}



