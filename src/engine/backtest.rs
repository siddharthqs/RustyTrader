use crate::core::candle::{Candle,CandleSticks};
use crate::core::instrument::Instrument;
use csv;
// pub fn backtest(inst: Box<Instrument>, quantity: i32) {
//     let mut candle_sticks = CandleSticks::new("1H".to_string());
//     let mut rdr = csv::Reader::from_path("data/eurusd_1H.csv").unwrap();
//     for result in rdr.records() {
//         let record = result.unwrap();
//         let date = record[0].to_string();
//         let open = record[1].parse::<f64>().unwrap();
//         let high = record[2].parse::<f64>().unwrap();
//         let low = record[3].parse::<f64>().unwrap();
//         let close = record[4].parse::<f64>().unwrap();
//         let volume = record[5].parse::<i32>().unwrap();
//         let candle = Candle::from_str_date(date, open, high, low, close, volume);
//         candle_sticks.add_candle(candle);
//     }
//     let mut strategy = SampleStrategy::new("Sample Strategy".to_string(), 10);
//     for candle in candle_sticks.candles {
//         strategy.on_bar(&candle_sticks);
//     }
// }
fn read_historical_data(file: &str, time_frame:&str) -> CandleSticks {
    let mut candles: Vec<Candle> = Vec::new();
    let mut rdr = csv::Reader::from_path(file).unwrap();
    for result in rdr.records() {
        let record = result.unwrap();
        let date = record[0].to_string();
        let open = record[1].parse::<f64>().unwrap();
        let high = record[2].parse::<f64>().unwrap();
        let low = record[3].parse::<f64>().unwrap();
        let close = record[4].parse::<f64>().unwrap();
        let volume = record[5].parse::<i32>().unwrap();
        let candle = Candle::from_strdate(date, open, high, low, close, volume);
        candles.push(candle);
    }
    CandleSticks {
        candles,
        time_frame: time_frame.to_string(),
    }
}