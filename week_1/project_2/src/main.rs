fn main() {
    let sales = [
        ("Toshiba", 450_000.00),
        ("Mac", 1_500_000.00),
        ("HP", 750_000.00),
        ("Dell", 2_850_000.00),
        ("Acer", 250_000.00),
    ];

    let mut sum = 0.0;

    for (_, amount) in sales.iter() {
        sum += amount;
    }

    let average = sum / sales.len() as f64;

    println!("Total Sales: ₦{:.2}", sum);
    println!("Average Sales: ₦{:.2}", average);
}