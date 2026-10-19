use std::io::Write; 

fn main() { 
   let headings = "Larger, Stout, Non-Alcoholic";
   let data_1 = "\n33Export, Legend, Maltina";
   let data_2 = "\n Desperados, Turbo King, Amstel Malta"; 
   let data_3 = "\n Goldberg, Williams, Malta Gold"; 
   let data_4 = "\n Guilder, Fayrouz";
   let data_5 = "\n Heineken";
   let data_6 = "\n Star";

   let mut file = std::fs::File::create("Breweries.csv").expect("Create failed");
   file.write_all("List of Drinks sold by Nigerian Breweries\n" 
    .as_bytes()).expect("Write failed");
   file.write(headings.as_bytes()).expect("write failed");
    file.write(data_1.as_bytes()).expect("write failed");
    file.write(data_2.as_bytes()).expect("write failed");
    file.write(data_3.as_bytes()).expect("write failed");
    file.write(data_4.as_bytes()).expect("write failed");
    file.write(data_5.as_bytes()).expect("write failed");
    file.write(data_6.as_bytes()).expect("write failed");
    
    println!("Data created");
}