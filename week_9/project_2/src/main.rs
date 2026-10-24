use std::fs::File;
use std::io::Write;
fn main() {
    let students = vec![
        ("Oluchi Mordi", "ACC10211111", "Accounting", 300),
        ("Adams Aliyu", "ECO10110101", "Economics", 100),
        ("Shania Bolade", "CSC10328828", "Computer", 200),
        ("Adekunle Gold", "EEE11020202", "Electrical", 200),
        ("Blanca Edemoh", "MEE10202001", "Mechanical", 100),
    ];
    let mut file = File::create("pau_students.txt")
        .expect("Failed to create file");
    for i in 0..students.len() {
        println!("Name: {}", students[i].0);
        println!("Matric Number: {}", students[i].1);
        println!("Department: {}", students[i].2);
        println!("Level: {}", students[i].3);
        println!();
        file.write_all(students[i].0.as_bytes())
            .expect("Write failed");
        file.write_all(", ".as_bytes())
            .expect("Write failed");
        file.write_all(students[i].1.as_bytes())
            .expect("Write failed");
        file.write_all(", ".as_bytes())
            .expect("Write failed");
        file.write_all(students[i].2.as_bytes())
            .expect("Write failed");
        file.write_all(", ".as_bytes())
            .expect("Write failed");
        file.write_all(students[i].3.to_string().as_bytes())
            .expect("Write failed");
        file.write_all("\n".as_bytes())
            .expect("Write failed");
    }
    println!("Student records saved successfully!");
}