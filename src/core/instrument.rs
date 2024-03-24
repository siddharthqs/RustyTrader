use std::collections::HashMap;
use chrono::NaiveDateTime;

use crate::core::candle::{Candle,CandleSticks};
use crate::core::utils::{BackTestConfig, Broker, Environment};
use crate::core::strategy::Strategy;
pub trait Instrument {
    fn on_bar(&mut self, candle: Candle);
    fn on_trade(&mut self, quantity: i32);
    fn buy(&mut self);
    fn close_buy(&mut self);
    fn sell(&mut self);
    fn close_sell(&mut self);
}
pub struct SingleInstrument {
    ticker: String,
    win_count: i32,
    loss_count: i32,
    buy_price: f64,
    sell_price: f64,
    buy_count: i32,
    sell_count: i32,
    position: i32,
    price_series: CandleSticks,
    profit: Vec<(NaiveDateTime,f64)>,
    quantity_size: i32,
    config: BackTestConfig,
    strategy: Box<dyn Strategy>,
    aggressive: bool,
    stop_loss_price: f64,
}
impl SingleInstrument {
    pub fn total_return(&self) -> f64 {
        let mut total_return = 0.0;
        for (_, pnl) in self.profit.iter() {
            total_return += pnl;
        }
        total_return
    }
    pub fn new(ticker:String,price_series: CandleSticks, config: BackTestConfig, strategy: Box<dyn Strategy>, aggressive: bool) -> SingleInstrument {
        SingleInstrument {
            ticker: ticker,
            win_count: 0,
            loss_count: 0,
            buy_price: 0.0,
            sell_price: 0.0,
            buy_count: 0,
            sell_count: 0,
            position: 0,
            price_series,
            profit: Vec::new(),
            quantity_size: 1,
            config,
            strategy,
            aggressive,
            stop_loss_price: 0.0,
        }
    }
    fn broker(&self) -> &Broker {
        &self.config.broker
    }
    fn environment(&self) -> &Environment{
        &self.config.environment
    }
    fn take_profit(&self) -> f64 {
        self.config.take_profit
    }
    fn stop_loss(&self) -> f64 {
        self.config.stop_loss
    }
    pub fn on_bar(&mut self, candle: Candle) {
        self.price_series.add_candle(candle);
        let len = self.price_series.candles.len();
        if len < self.strategy.lookback() {
            return;
        }
        let change =  (self.position as f64)*(self.price_series.latest_candle().close - self.price_series.second_latest_candle().close);
        let candle = self.price_series.latest_candle();
        let datetime = candle.clone().date;
        self.profit.push((datetime, change));
        self.strategy.on_bar(&self.price_series);
    }
    pub fn on_trade(&mut self, quantity: i32) {
        let buy_signal = self.strategy.buy_signal(&self.price_series);
        let sell_signal = self.strategy.sell_signal(&self.price_series);
        let close_buy_signal = self.strategy.close_buy_signal(&self.price_series);
        let close_sell_signal = self.strategy.close_sell_signal(&self.price_series);
        if self.position== 0 {
            match (buy_signal, sell_signal) {
                (true, _) => self.buy(quantity),
                (_, true) => self.sell(quantity),
                _ => (),
            }
        }
        else if self.position > 0 {
            let profit_pct = self.price_series.latest_candle().close / self.buy_price;
            match (buy_signal, close_buy_signal,sell_signal,close_sell_signal) {
                (true, _,_,_) => (),
                (_, true,false,_) => self.close_buy(),
                (_,_,true,_) => {self.close_buy();
                    if self.aggressive { self.sell(quantity);}
                },
                _ => {
                    if profit_pct > self.take_profit(){self.close_buy(); }
                    else if self.stop_loss_price > self.price_series.latest_candle().close {
                        self.close_buy();
                    }
                },
            }
        }
        else if self.position < 0 {
            let profit_pct = self.sell_price / self.price_series.latest_candle().close;
            match (buy_signal, close_buy_signal,sell_signal,close_sell_signal) {
                (_, _,true,_) => (),
                (false,_,_,true) => self.close_sell(),
                (true,_,_,_) => {self.close_sell();
                    if self.aggressive { self.buy(quantity);}
                },
                _ => {
                    if profit_pct > self.take_profit(){ self.close_sell();}
                    else if self.stop_loss_price < self.price_series.latest_candle().close {
                        self.close_sell();
                    }
                },
            }

        }
    }
    fn buy(&mut self, quantity: i32) {
        self.buy_price = self.price_series.latest_candle().close;
        self.position = 1;
        self.buy_count += 1;
        self.quantity_size = quantity;
        self.stop_loss_price = self.buy_price * self.stop_loss();
    }
    fn close_buy(&mut self) {
        let close_price = self.price_series.latest_candle().close;
        let profit = (close_price - self.buy_price) * self.quantity_size as f64;

        let transaction_cost = close_price * self.quantity_size as f64 * self.config.commission;
        let tmp = self.profit.pop();
        match tmp {
            Some((datetime,pnl)) => {
                self.profit.push((datetime, pnl - transaction_cost));
            },
            None => (),
        }

        self.position = 0;
        self.quantity_size = 0;
        if profit < 0.0 { self.loss_count += 1; } else { self.win_count += 1; }
    }
    fn sell(&mut self, quantity: i32) {
        self.sell_price = self.price_series.latest_candle().close;
        self.position = -1;
        self.sell_count += 1;
        self.quantity_size = quantity;
        self.stop_loss_price = self.sell_price * (1.0/self.stop_loss());
    }
    fn close_sell(&mut self) {
        let close_price = self.price_series.latest_candle().close;
        let profit = (self.sell_price - close_price) * self.quantity_size as f64;
        let transaction_cost = close_price * self.quantity_size as f64 * self.config.commission;
        let tmp = self.profit.pop();
        //let adjusted_profit = profit - transaction_cost;
        match tmp {
            Some((datetime,pnl)) => {
                self.profit.push((datetime, pnl - transaction_cost));
            },
            None => (),
        }
        self.position = 0;
        self.quantity_size = 0;
        if profit < 0.0 { self.loss_count += 1; } else { self.win_count += 1; }
    }
}