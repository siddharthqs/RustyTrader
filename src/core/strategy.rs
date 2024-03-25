use crate::core::candle::{Candle,CandleSticks};
pub trait Strategy {
    fn on_bar(&mut self, price_series: &CandleSticks);
    fn buy_signal(&self, price_series: &CandleSticks) -> bool;
    fn sell_signal(&self, price_series: &CandleSticks) -> bool;
    fn close_buy_signal(&self, price_series: &CandleSticks) -> bool;
    fn close_sell_signal(&self, price_series: &CandleSticks) -> bool;
    fn lookback(&self) -> usize;
}

#[derive(Debug)]
pub struct SampleStrategy{
    name: String,
    lookback: i32,
}
pub struct SMAStrategy{
    name: String,
    lookback: usize,
    sma: f64,
    sma_para: usize
}
impl SMAStrategy {
    pub fn new(name: String, lookback: usize,sma_para:usize) -> SMAStrategy {
        SMAStrategy{ name, lookback, sma: 0.0, sma_para }
    }
}
impl Strategy for SMAStrategy {
    fn lookback(&self) -> usize {
        self.lookback
    }
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

