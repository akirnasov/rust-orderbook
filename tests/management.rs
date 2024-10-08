mod common;
use common::{empty_ob, full_ob};
use orderbook_lib::{
    account::TradingAccount,
    backtest::{Strategy, StrategyName},
    management::OrderManagementSystem,
    Indicator, Order, OrderBook, Side,
};
use pretty_assertions::assert_eq;
use rstest::rstest;

#[rstest]
#[case(empty_ob(), Err("Missing Ref Price"))]
#[case(full_ob(), Ok(Order{id: 777, side: Side::Bid, price: 100, qty: 10, ts_create: 0}))]
fn ref_price_to_order_test(#[case] ob: OrderBook, #[case] expected: Result<Order, &str>) {
    let mut strategy = Strategy::new(StrategyName::TestStrategy);
    let account = TradingAccount::new(0);
    strategy.buy_criterion = 0.0;
    strategy.buy_position_limit = 10;
    strategy.qty = 100;
    let trader_id = 777;
    let midprice = Indicator::Midprice;
    let m = midprice.evaluate(&ob);
    let oms = OrderManagementSystem::new(&mut strategy, account);
    let trader_order = oms.calculate_buy_order(m, trader_id, 0);

    assert_eq!(trader_order, expected);
}
