use chrono::NaiveDateTime;
use csv::Reader;
#[derive(Debug,Clone)]
pub struct Candle {
    pub date: NaiveDateTime,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: i32,
}
impl Candle{
    pub fn new(date: NaiveDateTime, open: f64, high: f64, low: f64, close: f64, volume: i32) -> Candle {
        Candle {
            date,
            open,
            high,
            low,
            close,
            volume,
        }
    }
    pub fn from_strdate(date: String, open: f64, high: f64, low: f64, close: f64, volume: i32) -> Candle {
        let date = NaiveDateTime::parse_from_str(&date, "%Y-%m-%d %H:%M:%S").unwrap();
        Candle {
            date,
            open,
            high,
            low,
            close,
            volume,
        }
    }
}
pub struct CandleSticks {
    pub candles: Vec<Candle>,
    pub time_frame: String,
}
impl CandleSticks {
    pub fn new(time_frame: String) -> CandleSticks {
        CandleSticks {
            candles: Vec::new(),
            time_frame,
        }
    }
    pub fn add_candle(&mut self, candle: Candle) {
        self.candles.push(candle);
    }
    pub fn latest_candle(&self) -> &Candle {
        &self.candles[self.candles.len() - 1]
    }
    pub fn second_latest_candle(&self) -> &Candle {
        &self.candles[self.candles.len() - 2]
    }
    pub fn from_csv(file: &str) -> CandleSticks {
        let mut rdr = Reader::from_path(file).unwrap();
        let mut candle_sticks = CandleSticks::new("5min".to_string());
        for result in rdr.records() {
            let record = result.unwrap();
            let date = record[0].to_string();
            let open = record[1].parse::<f64>().unwrap();
            let high = record[2].parse::<f64>().unwrap();
            let low = record[3].parse::<f64>().unwrap();
            let close = record[4].parse::<f64>().unwrap();
            let volume = record[5].parse::<i32>().unwrap();
            let candle = Candle::from_strdate(date, open, high, low, close, volume);
            candle_sticks.add_candle(candle);
        }
        candle_sticks
    }

}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_candle() {
        let date = NaiveDateTime::parse_from_str("2021-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let candle = Candle::new(date, 1.0, 2.0, 0.5, 1.5, 100);
        assert_eq!(candle.open, 1.0);
        assert_eq!(candle.high, 2.0);
        assert_eq!(candle.low, 0.5);
        assert_eq!(candle.close, 1.5);
        assert_eq!(candle.volume, 100);
    }
    #[test]
    fn test_candlesticks() {
        let candle = Candle::new(NaiveDateTime::parse_from_str("2021-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap(), 1.0, 2.0, 0.5, 1.5, 100);
        let mut candle_sticks = CandleSticks::new("5min".to_string());
        candle_sticks.add_candle(candle);
        let candle = Candle::new(NaiveDateTime::parse_from_str("2021-01-01 00:05:00", "%Y-%m-%d %H:%M:%S").unwrap(), 1.5, 3.0, 1.0, 2.5, 100);
        candle_sticks.add_candle(candle);
        assert_eq!(candle_sticks.candles.len(), 2);
        assert_eq!(candle_sticks.latest_candle().open, 1.5);
        assert_eq!(candle_sticks.second_latest_candle().open, 1.0);

    }
    #[test]
    fn test_candlesticks_from_csv() {
        let candle_sticks = CandleSticks::from_csv("./src/tests/CL_5min.txt");
        assert_eq!(candle_sticks.candles.len(), 755);
        assert_eq!(candle_sticks.latest_candle().open, 77.8);
        assert_eq!(candle_sticks.second_latest_candle().open, 77.78);
    }

}