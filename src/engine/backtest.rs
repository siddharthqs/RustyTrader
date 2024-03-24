use chrono::NaiveDateTime;
use crate::core::candle::{Candle, CandleSticks};
use crate::core::instrument::{Instrument, SingleInstrument};
use csv;
use crate::core::utils::BackTestConfig;
use crate::core::strategy::Strategy;
use polars::prelude::*;
use polars::prelude::CsvReader;
use std::sync::mpsc;
use std::thread;

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