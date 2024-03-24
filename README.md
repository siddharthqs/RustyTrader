# RustyTrader
RustyTrader is a simple trading backtester written in Rust.
It uses Instrument struct to iterate over the data and execute the strategy and same infrastructure can be used to
execute the strategy on live trading.
## Usage
Add this to your Cargo.toml:
```toml
[dependencies]
rusty_trader = "0.1.0"
```
## Example
```rust

use rusty_trader::strategy::Strategy;
use RustyTrader::core::candle::{CandleSticks};
use RustyTrader::engine::backtest::BackTester;
```
### Create a strategy
```rust
pub struct SMAStrategy{
    name: String,
    lookback: usize,
    sma: f64,
    sma_para: usize
}
impl Strategy for SMAStrategy {
    fn lookback(&self) -> usize {
        self.lookback
    }
    // for preprocessing the data
    fn on_bar(&mut self, ohlc: &CandleSticks) {
        let len = ohlc.candles.len();
        let price = &ohlc.candles[len - self.lookback..];
        self.sma = price[price.len() - self.sma_para..].iter()
            .map(|x| x.close).sum::<f64>() / self.sma_para as f64;
    }
    fn buy_signal(&self, ohlc: &CandleSticks) -> bool {
        let len = ohlc.candles.len();
        if ohlc.candles[len - 1].close > self.sma {
            return true;
        }
        false
    }
    fn sell_signal(&self, ohlc: &CandleSticks) -> bool {
        let len = ohlc.candles.len();
        if ohlc.candles[len - 1].close < self.sma {
            return true;
        }
        false
    }
    fn close_buy_signal(&self, ohlc: &CandleSticks) -> bool {
        let len = ohlc.candles.len();
        if ohlc.candles[len - 1].close < self.sma {
            return true;
        }
        false
    }
    fn close_sell_signal(&self, ohlc: &CandleSticks) -> bool {
        let len = ohlc.candles.len();
        if ohlc.candles[len - 1].close > self.sma {
            return true;
        }
        false
    }
}
```
Each strategy should implement the Strategy trait which has the following methods:
- lookback: returns the lookback period of the strategy
- on_bar: preprocessing the data
- buy_signal: returns true if the buy signal is generated
- sell_signal: returns true if the sell signal is generated
- close_buy_signal: returns true if the close buy signal is generated
- close_sell_signal: returns true if the close sell signal is generated

### Backtest the strategy
```rust
fn main() {
    let strategy = SMAStrategy{ name: "SMA".to_string(), lookback: 20, sma: 0.0, sma_para: 5 };
    let file = r"ES_5min.txt";
    
    let back_tester = BackTester{ data_file: file.to_string(), quantity: 1, commission: 0.0008, stop_loss: 0.99, take_profit: 1.03 };
    let pnl = back_tester.run(Box::new(strategy));
    println!("{:?}", pnl);

}
```
## License
This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details

## Features
- [x] Backtesting
- [ ] Live Trading
- [ ] Data Feeds
- [ ] Risk Management
- [ ] Optimization
- [ ] Reporting
- [ ] Visualization

It is still in the early stage of development and many features are yet to be implemented.
