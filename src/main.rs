use std::collections::HashMap;

#[derive(Debug)]
enum BidOrAsk {
    Bid,
    Ask,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
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

#[derive(Debug)]
struct Order {
    size: f64,
    bid_or_ask: BidOrAsk,
}

impl Order {
    fn new(bid_or_ask: BidOrAsk, size: f64) -> Order {
        Order { size, bid_or_ask }
    }
}

#[derive(Debug)]
struct Limit {
    price: Price,
    orders: Vec<Order>
}

impl Limit {
    fn new(price: Price) -> Limit {
        Limit { 
            price: price,
            orders: Vec::new(), 
        }
    }

    fn add_order(&mut self, order: Order) {
        self.orders.push(order);
    }
}
#[derive(Debug)]
struct OrderBook {
    asks: HashMap<Price, Limit>,
    bids: HashMap<Price, Limit>,
}

impl OrderBook {
    fn new() -> OrderBook {
        OrderBook { 
            asks: HashMap::new(), 
            bids: HashMap::new(),
        }
    }

    fn add_limit_order(&mut self,price: f64, order: Order) {
        match order.bid_or_ask {
            BidOrAsk::Bid => {
                let price = Price::new(price);
                match self.bids.get_mut(&price) {
                    Some(limit) => limit.add_order(order),
                    None => {
                        let mut limit = Limit::new(price);
                        limit.add_order(order);
                        self.bids.insert(price, limit);
                    },
                }
            }

            BidOrAsk::Ask => {

            }
        }
    }
}

fn main() {
    let buy_order: Order = Order::new(BidOrAsk::Bid, 5.5);
    let sell_order: Order = Order::new(BidOrAsk::Ask, 2.4);
    
    let mut order_book  = OrderBook::new();

    order_book.add_limit_order(20.6, buy_order);
    order_book.add_limit_order(21.6, sell_order);

    println!("order book:  {:?}", order_book);

    println!("Hello, world!");
}
