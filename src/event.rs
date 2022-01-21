use crate::fill::FillEvent;
use crate::market::MarketEvent;
use crate::order::OrderEvent;

pub enum DefaultEvent {
    Market(MarketEvent),
    Order(OrderEvent),
    Fill(FillEvent),
}

pub enum Event<T> {
    Default(DefaultEvent),
    Custom(T),
}
