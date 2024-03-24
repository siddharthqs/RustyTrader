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
impl SampleStrategy {
    fn new(name: String, lookback: i32) -> SampleStrategy {
        SampleStrategy {
            name,
            lookback,
        }
    }
}
impl Strategy for SampleStrategy {

    fn on_bar(&mut self, price_series: &CandleSticks) {
        let len = price_series.candles.len();
        if len < self.lookback as usize {
            return;
        }
    }
    fn lookback(&self) -> usize {
        90
    }
    fn buy_signal(&self, price_series: &CandleSticks) -> bool {
        let len = price_series.candles.len();
        if len < self.lookback as usize {
            return false;
        }
        false
    }
    fn sell_signal(&self, price_series: &CandleSticks) -> bool {
        let len = price_series.candles.len();
        if len < self.lookback as usize {
            return false;
        }
        false
    }
    fn close_buy_signal(&self, price_series: &CandleSticks) -> bool {
        let len = price_series.candles.len();
        if len < self.lookback as usize {
            return false;
        }
        false
    }
    fn close_sell_signal(&self, price_series: &CandleSticks) -> bool {
        let len = price_series.candles.len();
        if len < self.lookback as usize {
            return false;
        }
        false
    }
}
