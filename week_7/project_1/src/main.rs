use std::io;

fn main() {
    loop {
        println!("                              ");
        println!("   PROJECT: SHAPE CALCULATOR  ");
        println!("                              ");
        println!("1. Trapezium (Area)");
        println!("2. Rhombus (Area)");
        println!("3. Parallelogram (Area)");
        println!("4. Cube (Surface Area)");
        println!("5. Cylinder (Volume)");
        println!("6. Exit");
        println!("                              ");
        println!("Enter your choice (1-6): ");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input! Please enter a number between 1 and 6.\n");
                continue;
            }
        };

        match choice {
            1 => {
                let result = calculate_trapezium_area();
                println!("\n>> Result: Area of Trapezium = {:.2}\n", result);
            }
            2 => {
                let result = calculate_rhombus_area();
                println!("\n>> Result: Area of Rhombus = {:.2}\n", result);
            }
            3 => {
                let result = calculate_parallelogram_area();
                println!("\n>> Result: Area of Parallelogram = {:.2}\n", result);
            }
            4 => {
                let result = calculate_cube_surface_area();
                println!("\n>> Result: Surface Area of Cube = {:.2}\n", result);
            }
            5 => {
                let result = calculate_cylinder_volume();
                println!("\n>> Result: Volume of Cylinder = {:.2}\n", result);
            }
            6 => {
                println!("Exiting program. Goodbye!");
                break;
            }
            _ => {
                println!("Invalid choice. Please choose an option between 1 and 6.\n");
            }
        }
    }
}

// Helper function to get numeric input from the user safely
fn get_user_input(prompt: &str) -> f64 {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        
        match input.trim().parse::<f64>() {
            Ok(num) => return num,
            Err(_) => println!("Invalid number. Please try again."),
        }
    }
}

fn calculate_trapezium_area() -> f64 {
    println!("--- Trapezium Area Calculation ---");
    let height = get_user_input("Enter height:");
    let base1 = get_user_input("Enter base 1:");
    let base2 = get_user_input("Enter base 2:");
    
    height / 2.0 * (base1 + base2)
}

fn calculate_rhombus_area() -> f64 {
    println!("--- Rhombus Area Calculation ---");
    let diagonal1 = get_user_input("Enter diagonal 1:");
    let diagonal2 = get_user_input("Enter diagonal 2:");
    
    0.5 * diagonal1 * diagonal2
}

fn calculate_parallelogram_area() -> f64 {
    println!("--- Parallelogram Area Calculation ---");
    let base = get_user_input("Enter base:");
    let altitude = get_user_input("Enter altitude:");
    
    base * altitude
}

fn calculate_cube_surface_area() -> f64 {
    println!("--- Cube Surface Area Calculation ---");
    let side = get_user_input("Enter side length:");
    
    6.0 * side * side
}

fn calculate_cylinder_volume() -> f64 {
    println!("--- Cylinder Volume Calculation ---");
    let radius = get_user_input("Enter radius:");
    let height = get_user_input("Enter height:");
    
    std::f64::consts::PI * radius * radius * height
}