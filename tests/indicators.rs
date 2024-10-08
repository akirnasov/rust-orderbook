use orderbook_lib::{Indicator, Order, OrderBook, Side};
use pretty_assertions::assert_eq;
use rstest::rstest;

fn empty_ob() -> OrderBook {
    OrderBook::new()
}

fn full_ob(bid: u32, ask: u32) -> OrderBook {
    let mut ob = OrderBook::new();
    let buy_order = Order {
        id: 666,
        side: Side::Bid,
        price: bid,
        qty: 10,
        ts_create: 0
    };
    ob.add_limit_order(buy_order, 0);
    let sell_order = Order {
        id: 999,
        side: Side::Ask,
        price: ask,
        qty: 10,
        ts_create: 0
    };
    ob.add_limit_order(sell_order, 0);
    ob
}

#[rstest]
#[case(empty_ob(), None)]
#[case(full_ob(99, 101), Some(100.0))]
fn midprice_test(#[case] input: OrderBook, #[case] expected: Option<f32>) {
    let midprice = Indicator::Midprice;
    assert_eq!(midprice.evaluate(&input), expected);
}
