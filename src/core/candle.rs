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
    pub fn from_csv(file: &str, time_frame: String) -> CandleSticks {
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
            time_frame,
        }
    }
}