#[derive(Debug)]
pub enum Environment {
    Backtest,
    PaperTrading,
    LIVE,
}
#[derive(Debug)]
enum OrderType {
    Market,
    Limit,
    Stop,
    StopLimit,
    TrailingStop,
}
#[derive(Debug)]
enum OrderStatus {
    Submitted,
    Accepted,
    PartiallyFilled,
    Filled,
    Canceled,
    Rejected,
}
#[derive(Debug)]
enum OrderSide {
    BUY,
    SELL,
}
#[derive(Debug)]
pub enum Broker {
    Alpaca,
    Binance,
    Bitfinex,
    Bitstamp,
    Coinbase,
    Kraken,
    Oanda,
    TD,
    None,
}
#[derive(Debug)]
pub struct BackTestConfig{
    start_date: String,
    end_date: String,
    initial_cash: f64,
    pub commission: f64,
    pub slippage: f64,
    pub stop_loss: f64,
    pub take_profit: f64,
    pub environment: Environment,
    pub broker: Broker,
}
impl BackTestConfig {
    pub fn new(commission: f64, slippage: f64, stop_loss: f64, take_profit: f64) -> BackTestConfig {
        BackTestConfig {
            start_date: "2020-01-01".to_string(),
            end_date: "2020-12-31".to_string(),
            initial_cash: 100000.0,
            commission,
            slippage,
            stop_loss,
            take_profit,
            environment: Environment::Backtest,
            broker: Broker::None,
        }
    }
}
