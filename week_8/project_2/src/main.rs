fn main() {
    // Vector of tuples containing names and years of experience
    let candidates = vec![
        ("Nelson", 3),
        ("Chidi", 5),
        ("Emeka", 2),
        ("David", 8),
        ("Samuel", 6),
    ];

    // Assume the first candidate has the highest experience
    let mut highest = candidates[0];

    // Loop through the vector
    for candidate in &candidates {
        if candidate.1 > highest.1 {
            highest = *candidate;
        }
    }

    // Display the candidate with the highest experience
    println!("Candidate with the highest experience:");
    println!("Name: {}", highest.0);
    println!("Years of programming experience: {}", highest.1);
}