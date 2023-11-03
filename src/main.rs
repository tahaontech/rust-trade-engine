enum BidOrAsk {
    Bid,
    Ask,
}

struct Price {
    integral: u64,
    fractional: u64,
    scalar: u64,
}

impl Price {
    fn new(price: f64) -> Price {
        let scalar: u64 = 100000;
        let integral: u64 = price as u64;
        let fractional: u64 = ((price % 1.0) * scalar as f64) as u64;
        Price { integral, fractional, scalar }
    }
}

struct Limit {
    price: Price,
    orders: Vec<Order>
}

struct Order {
    size: f64,
    bid_or_ask: BidOrAsk,
}

impl Order {
    fn new(bid_or_ask: BidOrAsk, size: f64) -> Order {
        Order { size, bid_or_ask }
    }
}


fn main() {
    println!("Hello, world!");
}
