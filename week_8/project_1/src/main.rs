use std::io;

fn main() {
    let levels = vec![
        "APS 1-2",
        "APS 3-5",
        "APS 5-8",
        "EL 1 8-10",
        "EL 2 10-13",
        "SES",
    ];

    let jobs = vec![
        vec!["Intern", "Paralegal", "Placement"],
        vec!["Administrator", "Research Assistant", "Junior Associate", "Classroom Teacher"],
        vec!["Senior Administrator", "PhD Candidate", "Associate", "Senior Teacher"],
        vec!["Office Manager", "Post-Doc Researcher", "Senior Associate 1-2", "Leading Teacher"],
        vec!["Director", "Senior Lecturer", "Senior Associate 3-4", "Deputy Principal"],
        vec!["CEO", "Dean", "Partner", "Principal"],
    ];

    println!("Enter the staff position:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let position = input.trim().to_lowercase();

    let mut found = false;

    for i in 0..jobs.len() {
        for job in &jobs[i] {
            if job.to_lowercase() == position {
                println!("{} → {}", job, levels[i]);
                found = true;
            }
        }
    }

    if !found {
        println!("Position not found.");
    }
}
