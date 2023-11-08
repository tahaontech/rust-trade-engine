mod matching_engine;
use matching_engine::orderbook::{Order, OrderBook};
use matching_engine::engine::{MatchingEngine, TradingPair};
use crate::matching_engine::orderbook::BidOrAsk;


fn main() {
    let buy_order: Order = Order::new(BidOrAsk::Bid, 5.5);
    let sell_order: Order = Order::new(BidOrAsk::Ask, 2.4);
    
    let mut order_book  = OrderBook::new();

    order_book.add_limit_order(20.6, buy_order);
    order_book.add_limit_order(21.6, sell_order);

    // println!("order book:  {:?}", order_book);

    let mut engine: MatchingEngine = MatchingEngine::new();
    let pair = TradingPair::new("BTC".to_string(), "USD".to_string());
    engine.add_new_market(pair.clone());

    let buy_order: Order = Order::new(BidOrAsk::Bid, 5.5);
    engine.place_limit_order(pair, 10.000, buy_order).unwrap();

}
