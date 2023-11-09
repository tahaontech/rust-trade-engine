#![allow(dead_code)]

use std::collections::HashMap;
use rust_decimal::prelude::*;

#[derive(Debug)]
pub enum BidOrAsk {
    Bid,
    Ask,
}

#[derive(Debug)]
pub struct Order {
    size: f64,
    bid_or_ask: BidOrAsk,
}

impl Order {
    pub fn new(bid_or_ask: BidOrAsk, size: f64) -> Order {
        Order { size, bid_or_ask }
    }

    pub fn is_filled(&self) -> bool {
        self.size == 0.0
    }
}

#[derive(Debug)]
pub struct Limit {
    price: Decimal,
    orders: Vec<Order>
}

impl Limit {
    fn new(price: Decimal) -> Limit {
        Limit { 
            price: price,
            orders: Vec::new(), 
        }
    }

    fn total_volume(&self) -> f64 {
        self.orders.iter()
            .map(|order| order.size)
            .reduce(|a, b| a + b)
            .unwrap()
    }

    fn fill_order(&mut self, market_order: &mut Order) {
        for limit_order in self.orders.iter_mut() {
            match market_order.size >= limit_order.size {
                true => {
                    market_order.size -= limit_order.size;
                    limit_order.size = 0.0;
                },
                false => {
                    limit_order.size -= market_order.size;
                    market_order.size = 0.0;
                }
            }

            if market_order.is_filled() {
                break;
            }
        }
    }

    fn add_order(&mut self, order: Order) {
        self.orders.push(order);
    }

}


#[derive(Debug)]
pub struct OrderBook {
    asks: HashMap<Decimal, Limit>,
    bids: HashMap<Decimal, Limit>,
}

impl OrderBook {
    pub fn new() -> OrderBook {
        OrderBook { 
            asks: HashMap::new(), 
            bids: HashMap::new(),
        }
    }

    pub fn fill_market_order(&mut self, market_order: &mut Order) {
        let limits = match market_order.bid_or_ask {
            BidOrAsk::Ask => self.bid_limits(),
            BidOrAsk::Bid => self.ask_limits()
        };

        for limit_order in limits {
            limit_order.fill_order(market_order);

            if market_order.is_filled() {
                break;
            }
        }
    }

    pub fn ask_limits(&mut self) -> Vec<&mut Limit> {
        let mut limits = self.asks.values_mut().collect::<Vec<&mut Limit>>();
        limits.sort_by(|a, b| a.price.cmp(&b.price));

        limits
    }

    pub fn bid_limits(&mut self) -> Vec<&mut Limit> {
        let mut limits = self.bids.values_mut().collect::<Vec<&mut Limit>>();
        limits.sort_by(|a, b| b.price.cmp(&a.price));

        limits
    }

    pub fn add_limit_order(&mut self, price: Decimal, order: Order) {
        match order.bid_or_ask {
            BidOrAsk::Bid => {
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
                match self.asks.get_mut(&price) {
                    Some(limit) => limit.add_order(order),
                    None => {
                        let mut limit = Limit::new(price);
                        limit.add_order(order);
                        self.asks.insert(price, limit);
                    },
                }
            }
        }
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn orderbook_fill_market_order_ask() {
        let mut orderbook = OrderBook::new();
        orderbook.add_limit_order(dec!(500), Order::new(BidOrAsk::Ask, 10.0));
        orderbook.add_limit_order(dec!(100), Order::new(BidOrAsk::Ask, 10.0));
        orderbook.add_limit_order(dec!(200), Order::new(BidOrAsk::Ask, 10.0));
        orderbook.add_limit_order(dec!(300), Order::new(BidOrAsk::Ask, 10.0));

        assert_eq!(orderbook.ask_limits()[0].price, dec!(100));

        let mut market_order = Order::new(BidOrAsk::Bid, 20.0);
        orderbook.fill_market_order(&mut market_order);

        assert_eq!(market_order.is_filled(), true);

        let first_limit_order = orderbook.asks.get(&dec!(100)).unwrap().orders.get(0).unwrap();
        let second_limit_order = orderbook.asks.get(&dec!(200)).unwrap().orders.get(0).unwrap();

        assert_eq!(first_limit_order.is_filled(), true);
        assert_eq!(second_limit_order.is_filled(), true);
    }

    #[test]
    fn orderbook_fill_market_order_bid() {
        let mut orderbook = OrderBook::new();
        orderbook.add_limit_order(dec!(500), Order::new(BidOrAsk::Bid, 10.0));
        orderbook.add_limit_order(dec!(100), Order::new(BidOrAsk::Bid, 10.0));
        orderbook.add_limit_order(dec!(200), Order::new(BidOrAsk::Bid, 10.0));
        orderbook.add_limit_order(dec!(300), Order::new(BidOrAsk::Bid, 10.0));

        assert_eq!(orderbook.bid_limits()[0].price, dec!(500));

        let mut market_order = Order::new(BidOrAsk::Ask, 20.0);
        orderbook.fill_market_order(&mut market_order);

        assert_eq!(market_order.is_filled(), true);

        let first_limit_order = orderbook.bids.get(&dec!(500)).unwrap().orders.get(0).unwrap();
        let second_limit_order = orderbook.bids.get(&dec!(300)).unwrap().orders.get(0).unwrap();

        assert_eq!(first_limit_order.is_filled(), true);
        assert_eq!(second_limit_order.is_filled(), true);
    }

    #[test]
    fn limit_total_volume() {
        let price = dec!(10000);
        let mut limit = Limit::new(price);
        let buy_limit_order1 = Order::new(BidOrAsk::Bid, 100.0);
        let buy_limit_order2 = Order::new(BidOrAsk::Bid, 100.0);
        limit.add_order(buy_limit_order1);
        limit.add_order(buy_limit_order2);


        assert_eq!(limit.total_volume(), 200.0)
    }

    #[test]
    fn limit_order_single_fill() {
        let price = dec!(10000);
        let mut limit = Limit::new(price);
        let buy_limit_order = Order::new(BidOrAsk::Bid, 100.0);
        limit.add_order(buy_limit_order);

        let mut market_sell_order = Order::new(BidOrAsk::Ask, 99.0);
        limit.fill_order(&mut market_sell_order);

        assert_eq!(market_sell_order.is_filled(), true);
        assert_eq!(limit.orders.get(0).unwrap().size, 1.0);
    }

    #[test]
    fn limit_order_multiple_fill() {
        let price = dec!(10000);
        let mut limit = Limit::new(price);
        let buy_limit_order1 = Order::new(BidOrAsk::Bid, 100.0);
        let buy_limit_order2 = Order::new(BidOrAsk::Bid, 100.0);
        limit.add_order(buy_limit_order1);
        limit.add_order(buy_limit_order2);

        let mut market_sell_order = Order::new(BidOrAsk::Ask, 199.0);
        limit.fill_order(&mut market_sell_order);

        assert_eq!(market_sell_order.is_filled(), true);
        assert_eq!(limit.orders.get(0).unwrap().is_filled(), true);
        assert_eq!(limit.orders.get(1).unwrap().is_filled(), false);
        assert_eq!(limit.orders.get(1).unwrap().size, 1.0);
    }
}