use chrono::NaiveDateTime;
use crate::core::candle::{Candle, CandleSticks};
use crate::core::instrument::{Instrument, SingleInstrument};
use csv;
use crate::core::utils::BackTestConfig;
use crate::core::strategy::{Strategy, SMAStrategy};
use polars::prelude::*;
use polars::prelude::CsvReader;
use std::sync::mpsc;
use std::thread;

/// date is in the format of "%Y-%m-%d %H:%M:%S"
fn get_candles_from_csv(tx: mpsc::Sender<Candle>,file: &str) {
    let df = CsvReader::from_path(file).expect("Error Reading CSV").has_header(false)
        .finish();
    let mut df = df.unwrap();
    let _ = df.set_column_names(&["date", "open", "high", "low", "close", "volume"]);
    for row in 0..df.height() {
        let date = df.column("date").unwrap().get(row).expect("REASON").to_string();
        let date = date.trim_matches('\"');
        let date = NaiveDateTime::parse_from_str(&date, "%Y-%m-%d %H:%M:%S").unwrap();
        let open = df.column("open").unwrap().get(row).expect("Error").try_extract::<f64>().unwrap();
        let high = df.column("high").unwrap().get(row).expect("Error").try_extract::<f64>().unwrap();
        let low = df.column("low").unwrap().get(row).expect("Error").try_extract::<f64>().unwrap();
        let close = df.column("close").unwrap().get(row).expect("Error").try_extract::<f64>().unwrap();
        let volume = df.column("volume").unwrap().get(row).expect("Error").try_extract::<i32>().unwrap();
        let candle = Candle::new(date, open, high, low, close, volume);
        tx.send(candle).unwrap();
    }


}
pub struct BackTester{
    pub data_file: String,
    pub quantity: i32,
    pub commission: f64,
    pub stop_loss: f64,
    pub take_profit: f64,
}
impl BackTester {
    /// Run the backtest
    /// # Example
    /// let file = r"./src/tests/CL_5min.txt";
    ///  let back_tester = BackTester{ data_file: file.to_string(), quantity: 1, commission: 0.0008, stop_loss: 0.99, take_profit: 1.03 };
    ///  let strategy = SMAStrategy::new("SMA 10".to_string(), 30, 10);
    ///  let pnl = back_tester.run(Box::new(strategy));
    ///  assert_approx_eq!(pnl, -9.56, 1e-2);
    pub fn run( self,strategy: Box<dyn Strategy>) -> f64 {
        let data = self.data_file;
        let (tx, rx) = mpsc::channel();
        let config = BackTestConfig::new(self.commission, 0.0, self.stop_loss, self.take_profit);
        let producer_handle = thread::spawn(move || {
            get_candles_from_csv(tx, &data);
        });
        let candle_stick = CandleSticks::new("".to_string());
        let mut instrument = SingleInstrument::new("ZC".to_string(), candle_stick, config, strategy, false);
        for candle in rx {
            instrument.on_bar(candle);
            instrument.on_trade(self.quantity);
        }
        producer_handle.join().unwrap();
        println!("{:?}", instrument.total_return());
        return instrument.total_return();
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::strategy::SMAStrategy;
    use assert_approx_eq::assert_approx_eq;
    #[test]
    fn test_back_test() {

        let file = r"./src/tests/CL_5min.txt";
        //let file = r"D:\boltzmann_research\futures\futures_full_5min_con_ADJ\scope\ZC_5min_continuous_adjusted.txt";
        let back_tester = BackTester{ data_file: file.to_string(), quantity: 1, commission: 0.0008, stop_loss: 0.99, take_profit: 1.03 };
        let strategy = SMAStrategy::new("SMA 10".to_string(), 30, 10);
        let pnl = back_tester.run(Box::new(strategy));
        assert_approx_eq!(pnl, -9.627, 1e-2);

    }
}