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
    let mut file = File::create("students.txt")
        .expect("Failed to create file");
    for student in &students {
        println!(
            "Name: {}, Matric No: {}, Department: {}, Level: {}",
            student.0, student.1, student.2, student.3
        );
    }
    for student in &students {
        let line = format!(
            "{}, {}, {}, {}\n",
            student.0, student.1, student.2, student.3
        );
        file.write_all(line.as_bytes())
            .expect("Failed to write to file");
    }
    println!("Student details have been saved successfully!");
}