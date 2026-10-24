
use std::fs::File;
use std::io::Write;
fn main() {
    let commissioners = vec![
        "Aigbogun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbona",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etieye",
    ];
    let ministries = vec![
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum",
    ];
    let zones = vec![
        "South West",
        "North East",
        "South South",
        "South West",
        "South East",
    ];
    let mut file = File::create("efcc_records.txt")
        .expect("Failed to create file");
    for i in 0..commissioners.len() {
        file.write_all(commissioners[i].as_bytes())
            .expect("Write failed");
        file.write_all(", ".as_bytes())
            .expect("Write failed");
        file.write_all(ministries[i].as_bytes())
            .expect("Write failed");
        file.write_all(", ".as_bytes())
            .expect("Write failed");
        file.write_all(zones[i].as_bytes())
            .expect("Write failed");
        file.write_all("\n".as_bytes())
            .expect("Write failed");
    }
    println!("EFCC records merged successfully!");
}