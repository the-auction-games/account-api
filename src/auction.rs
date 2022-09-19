pub struct Auction {
    id: u64,
    name: String,
    description: String,
    price: f64
}

pub impl Auction {
    fn new(name: &str, description: &str, price: f64) -> Auction {
        Auction {
            id: 1,
            name: name.to_string(),
            description: description.to_string(),
            price: 100.00
        }
    }

    fn info(&self) -> String {
        format!("Auction #{id} -- {name}", id=self.id, name=self.name)
    }
}