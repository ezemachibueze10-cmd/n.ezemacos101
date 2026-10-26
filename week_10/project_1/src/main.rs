struct Laptop {
    brand: String,
    price: u32,
}

impl Laptop {
    fn cost(&self, quantity: u32) -> u32 {
        self.price * quantity
    }
}

fn main() {
    let hp = Laptop {
        brand: String::from("HP"),
        price: 650000,
    };

    let ibm = Laptop {
        brand: String::from("IBM"),
        price: 755000,
    };

    let toshiba = Laptop {
        brand: String::from("Toshiba"),
        price: 550000,
    };

    let dell = Laptop {
        brand: String::from("Dell"),
        price: 850000,
    };

    let hp_cost = hp.cost(3);
    let ibm_cost = ibm.cost(3);
    let toshiba_cost = toshiba.cost(3);
    let dell_cost = dell.cost(3);

    let total = hp_cost + ibm_cost + toshiba_cost + dell_cost;

    println!("{} cost: N{}", hp.brand, hp_cost);
    println!("{} cost: N{}", ibm.brand, ibm_cost);
    println!("{} cost: N{}", toshiba.brand, toshiba_cost);
    println!("{} cost: N{}", dell.brand, dell_cost);

    println!("Total cost: N{}", total);
}